use std::sync::Arc;

use chrono::Utc;
use cron_tab::AsyncCron;
use meowbot_proto::generated::{
    ai::{ai_client::AiClient, AiRequest}, commands::command_handler_server::CommandHandler, db::{db_client::DbClient, platform_client::PlatformClient, Command, NewMessage}, models::User, weather::weather_client::WeatherClient
};
use rand::random_range;
use tonic::{transport::Channel, Request, Response, Status};

use crate::{
    config::Configuration,
    formats::{format_datetime_russian, weather_to_emoji},
};
use string_format::string_format;

pub struct App {
    ai_client: AiClient<Channel>,
    db_client: DbClient<Channel>,
    platform_client: PlatformClient<Channel>,
    weather_client: WeatherClient<Channel>,
    config: Configuration,
}

#[tonic::async_trait]
impl CommandHandler for App {
    async fn handle_start(&self, user: Request<User>) -> Result<Response<()>, Status> {
        let user = user.get_ref().to_owned();
        
        let new_message = NewMessage {
            user: Some(user.clone()),
            text: string_format!(
                self.config.start_fmt.clone(),
                user.username.clone(),
                user.id.clone()
            )
        };
        
        self.platform_client.clone().send_message(new_message).await.expect("Failed to send message to user");
        
        Ok(Response::new(()))
    }
}

impl App {
    pub fn new(
        ai_client: AiClient<Channel>,
        db_client: DbClient<Channel>,
        platform_client: PlatformClient<Channel>,
        weather_client: WeatherClient<Channel>,
        config: Configuration,
    ) -> Arc<Self> {
        Arc::new(Self {
            ai_client,
            db_client,
            platform_client,
            weather_client,
            config,
        })
    }

    pub async fn bind_all_commands(self: Arc<Self>) {
        let start_cmd = Command {
            text: "start".into(),
        };
        self.platform_client
            .clone()
            .bind_start_command(start_cmd)
            .await
            .expect("Failed to bind commands");
    }

    pub async fn schedule_all_tasks(self: Arc<Self>) {
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

    async fn send_daily_messages(self: Arc<Self>) {
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
            let app = self.clone();
            let formatted_weather = formatted_weather.clone();

            tokio::spawn(async move {
                app.process_user(user, formatted_weather.clone()).await;
            });
        }
    }

    async fn process_user(&self, user: User, weather: String) {
        let ai_request = AiRequest {
            weather: weather.clone(),
            prompt: self.config.ai_prompt.clone(),
        };
        let ai_answer_response = self.ai_client.clone().get_response(ai_request).await;
        let ai_answer = if let Ok(ai_answer_response) = ai_answer_response {
            ai_answer_response.get_ref().to_owned().response
        } else {
            self.config.ai_msg_off.clone()
        };

        let now = Utc::now();

        let new_message = NewMessage {
            user: Some(user.clone()),
            text: string_format!(
                self.config.greeting_fmt.clone(),
                user.username.clone(),
                format_datetime_russian(now.naive_local()),
                weather,
                ai_answer.clone()
            ),
        };

        self.platform_client
            .clone()
            .send_message(new_message)
            .await
            .expect("Failed to send message to user");
    }

    async fn make_draw(self: Arc<Self>) -> () {
        let users_response = self
            .db_client
            .clone()
            .get_users(())
            .await
            .expect("Failed to get users");

        let users = users_response.get_ref().to_owned().users;

        let mut ind = random_range(0..users.len());
        let mut choice = &users[ind];

        let mut it = 0;

        while choice.id == self.config.admin {
            ind = random_range(0..users.len());
            choice = &users[ind];
            it += 1;
            if it == 10000000 {
                panic!("Unluckly, can't choose the winner");
            }
        }

        let new_message = NewMessage {
            user: Some(choice.clone()),
            text: string_format!(self.config.draw_win_fmt.clone(), choice.username.clone()),
        };

        self.platform_client
            .clone()
            .send_message(new_message)
            .await
            .expect("Send message failed");

        let results_fmt = string_format!(
            self.config.draw_results_fmt.clone(),
            choice.username.clone()
        );

        let admin = User {
            id: self.config.admin.clone(),
            username: "admin".into(),
        };

        let new_message = NewMessage {
            user: Some(admin),
            text: results_fmt.clone(),
        };

        self.platform_client
            .clone()
            .send_message(new_message)
            .await
            .expect("Send message failed");

        let channel: User = User {
            id: self.config.channel.clone(),
            username: "channel".into(),
        };

        let new_message = NewMessage {
            user: Some(channel),
            text: results_fmt,
        };

        self.platform_client
            .clone()
            .send_message(new_message)
            .await
            .expect("Send message failed");
    }
}
