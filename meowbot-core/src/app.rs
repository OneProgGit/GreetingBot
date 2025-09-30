use chrono::Utc;
use cron_tab::AsyncCron;
use meowbot_proto::generated::{
    ai::{AiRequest, ai_client::AiClient},
    db::{Command, db_client::DbClient, platform_client::PlatformClient},
    models::User,
    weather::weather_client::WeatherClient,
};
use tonic::transport::Channel;

use crate::{config::Configuration, formats::weather_to_emoji};
use string_format::string_format;

pub struct App {
    ai_client: AiClient<Channel>,
    db_client: DbClient<Channel>,
    platform_client: PlatformClient<Channel>,
    weather_client: WeatherClient<Channel>,
    config: Configuration,
}

impl App {
    pub fn new(
        ai_client: AiClient<Channel>,
        db_client: DbClient<Channel>,
        platform_client: PlatformClient<Channel>,
        weather_client: WeatherClient<Channel>,
        config: Configuration,
    ) -> Self {
        Self {
            ai_client,
            db_client,
            platform_client,
            weather_client,
            config,
        }
    }

    pub async fn bind_all_commands(&mut self) {
        let start_cmd = Command {
            text: "start".into(),
        };
        self.platform_client
            .bind_start_command(start_cmd)
            .await
            .expect("Failed to bind commands");
    }

    pub async fn schedule_all_tasks(&self) {
        let mut cron = AsyncCron::new(Utc);

        let app_clone_greeting = self.clone();
        let app_clone_draw = self.clone();
        cron.add_fn(&self.config.greeting_date_cron, move || {
            let app = app_clone_greeting.clone();
            async move {
                app.send_daily_messages().await;
            }
        })
        .await
        .expect("Failed to schedule daily message");

        cron.add_fn(&self.config.draw_date_cron, move || {
            let app = app_clone_draw.clone();
            async move {
                app.make_draw().await;
            }
        })
        .await
        .expect("Failed to schedule draw action");

        cron.start().await;
    }

    async fn send_daily_messages(&self) {
        let weather_response = self
            .weather_client
            .clone()
            .get_weather(())
            .await
            .expect("Failed to get weather");

        let weather_struct = weather_response.get_ref().to_owned();

        let formatted_weather = string_format!(
            self.config.weather_fmt.clone(),
            weather_struct.temp_c,
            weather_struct.feels_like_c,
            weather_struct.wind_speed_kmph,
            weather_struct.min_temp_c,
            weather_struct.max_temp_c,
            weather_to_emoji(&weather_struct.status),
            weather_struct.status
        );

        let users_list_response = self
            .db_client
            .clone()
            .get_users(())
            .await
            .expect("Failed to get users");

        let mut users_list = users_list_response.get_ref().to_owned();

        let channel = User {
            id: self.config.channel.clone(),
            username: "oneprogofficial".into(),
        };
        users_list.users.push(channel);

        for user in users_list.users {
            tokio::spawn(self.process_user(user, formatted_weather.clone()));
        }
    }

    async fn process_user(&self, user: User, weather: String) {
        let ai_request = AiRequest { weather };
        let ai_answer_response = self.ai_client.clone().get_response(ai_request).await;
        let ai_answer = if let Ok(ai_answer_response) = ai_answer_response {
            ai_answer_response.get_ref().to_owned().response
        } else {
            self.config.ai_msg_off.clone()
        };
    }

    async fn make_draw(&self) -> () {
        todo!()
    }
}
