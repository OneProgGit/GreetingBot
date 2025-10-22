use std::{collections::HashMap, sync::Arc};

use chrono::Utc;
use cron_tab::AsyncCron;
use meowbot_proto::generated::{
    ai::{AiRequest, ai_client::AiClient},
    commands::{HandleCommandMessage, command_handler_server::CommandHandler},
    db::{GetUserMessage, db_client::DbClient},
    models::User,
    platform::{NewMessage, platform_client::PlatformClient},
    weather::{GetWeatherMessage, weather_client::WeatherClient},
};
use rand::random_range;
use tonic::{Request, Response, Status, transport::Channel};

use crate::{
    config::Configuration,
    formats::{format_datetime_russian, weather_to_emoji},
};
use string_format::string_format;

#[derive(Clone)]
enum Command {
    Start,
    ChangeCity,
    ChangeAreasOfInterest,
}

#[derive(Clone)]
pub struct App {
    ai_client: AiClient<Channel>,
    db_client: DbClient<Channel>,
    platform_client: PlatformClient<Channel>,
    weather_client: WeatherClient<Channel>,
    config: Configuration,
    binds: HashMap<String, Command>,
}

#[tonic::async_trait]
impl CommandHandler for App {
    async fn handle_command(
        &self,
        request: Request<HandleCommandMessage>,
    ) -> Result<Response<()>, Status> {
        let request = request.get_ref().to_owned();
        println!("Handling message {request:?}...");

        let cmd = request.command;

        if let Some(b) = self.binds.get(&cmd) {
            match *b {
                Command::Start => self.handle_start(
                    request
                        .user
                        .ok_or_else(|| Status::invalid_argument("User must be set!"))?,
                ),
                Command::ChangeCity => todo!(),
                Command::ChangeAreasOfInterest => todo!(),
            }
            .await?;
        } else {
            let new_message = NewMessage {
                user: request.user,
                text: "Неизвестная команда".into(),
            };

            self.platform_client
                .clone()
                .send_message(new_message)
                .await?;
        }
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
    ) -> Self {
        Self {
            ai_client,
            db_client,
            platform_client,
            weather_client,
            config,
            binds: HashMap::new(),
        }
    }

    async fn handle_start(&self, user: User) -> Result<Response<()>, Status> {
        println!("Handling start command for user {user:?}...");

        let new_message = NewMessage {
            user: Some(user.clone()),
            text: string_format!(
                self.config.start_fmt.clone(),
                user.username.clone(),
                user.id.clone()
            ),
        };

        self.platform_client
            .clone()
            .send_message(new_message)
            .await?;

        self.db_client.clone().create_user(user).await?;

        Ok(Response::new(()))
    }

    async fn handle_change_city(
        &self,
        id: String,
        new_city: String,
    ) -> Result<Response<()>, Status> {
        println!("Changing city for id {id} and city {new_city}...");

        let get_user = GetUserMessage { id };
        let mut user = self
            .db_client
            .clone()
            .get_user(get_user)
            .await?
            .get_ref()
            .to_owned();
        user.city = new_city.clone();

        let get_weather = GetWeatherMessage {
            city: new_city.clone(),
        };

        if self
            .weather_client
            .clone()
            .get_weather(get_weather)
            .await
            .is_ok()
        {
            self.db_client.clone().update_user(user.clone()).await?;

            let new_message = NewMessage {
                user: Some(user),
                text: string_format!(self.config.changed_city_fmt.clone(), new_city),
            };

            self.platform_client
                .clone()
                .send_message(new_message)
                .await?;
        } else {
            let new_message = NewMessage {
                user: Some(user),
                text: string_format!(self.config.changed_city_failed_fmt.clone(), new_city),
            };

            self.platform_client
                .clone()
                .send_message(new_message)
                .await?;
        }

        Ok(Response::new(()))
    }

    async fn handle_change_areas_of_interest(
        &self,
        id: String,
        new_areas: String,
    ) -> Result<Response<()>, Status> {
        println!("Changing areas of interest for id {id:?} and areas {new_areas}...");

        let get_user = GetUserMessage { id };
        let mut user = self
            .db_client
            .clone()
            .get_user(get_user)
            .await?
            .get_ref()
            .to_owned();

        user.areas_of_interest = new_areas;

        self.db_client.clone().update_user(user.clone()).await?;

        let new_message = NewMessage {
            user: Some(user),
            text: self.config.changed_areas_of_interest_fmt.clone(),
        };

        self.platform_client
            .clone()
            .send_message(new_message)
            .await?;

        Ok(Response::new(()))
    }

    pub async fn bind_all_commands(&mut self) {
        self.binds.insert("start".into(), Command::Start);
        self.binds.insert("setcity".into(), Command::ChangeCity);
        self.binds
            .insert("setareas".into(), Command::ChangeAreasOfInterest);
    }

    pub async fn schedule_all_tasks(self: Arc<Self>) {
        let mut cron = AsyncCron::new(Utc);

        let app_clone_greeting = self.clone();
        let app_clone_draw = self.clone();
        cron.add_fn(&self.config.greeting_date_cron, move || {
            let app = app_clone_greeting.clone();
            async move {
                app.send_daily_messages()
                    .await
                    .expect("Failed to send dailty messages");
            }
        })
        .await
        .expect("Failed to schedule daily message");

        cron.add_fn(&self.config.draw_date_cron, move || {
            let app = app_clone_draw.clone();
            async move {
                app.make_draw().await.expect("Failed to make draw");
            }
        })
        .await
        .expect("Failed to schedule draw action");

        cron.start().await;
    }

    async fn process_user(&self, user: User) -> Result<(), Box<dyn std::error::Error>> {
        println!("Processing user {user:?}...");

        let get_weather = GetWeatherMessage {
            city: user.city.clone(),
        };

        let weather_response = self.weather_client.clone().get_weather(get_weather).await?;

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

        let ai_request = AiRequest {
            weather: formatted_weather.clone(),
            prompt: string_format!(
                self.config.ai_prompt.clone(),
                formatted_weather.clone(),
                user.areas_of_interest.clone()
            ),
            model: self.config.ai_model.clone(),
        };
        let ai_answer_response = self.ai_client.clone().get_response(ai_request).await;
        let ai_answer = ai_answer_response.map_or_else(
            |_| self.config.ai_msg_off.clone(),
            |ai_answer_response| ai_answer_response.get_ref().to_owned().response,
        );
        println!("Got ai answer: {ai_answer}...");

        let now = Utc::now();

        let new_message = NewMessage {
            user: Some(user.clone()),
            text: string_format!(
                self.config.greeting_fmt.clone(),
                user.username,
                format_datetime_russian(now.naive_local()),
                formatted_weather,
                ai_answer
            ),
        };

        self.platform_client
            .clone()
            .send_message(new_message)
            .await?;

        Ok(())
    }

    async fn send_daily_messages(self: Arc<Self>) -> Result<(), Box<dyn std::error::Error>> {
        println!("Sending daily messages...");

        let users_list_response = self.db_client.clone().get_users(()).await?;

        let mut users_list = users_list_response.get_ref().to_owned();

        let channel = User {
            id: self.config.channel.clone(),
            username: "oneprogofficial".into(),
            city: "Moscow".into(),
            areas_of_interest: "Rust, Unity, Unreal Engine, C#, C++".into(),
        };

        if !self.config.skip_channel {
            users_list.users.push(channel);
        }

        for user in users_list.users {
            let app = self.clone();

            tokio::spawn(async move {
                app.process_user(user.clone())
                    .await
                    .unwrap_or_else(|e| println!("Failed to process user {user:?}: {e}"));
            });
        }

        Ok(())
    }

    async fn make_draw(self: Arc<Self>) -> Result<(), Box<dyn std::error::Error>> {
        println!("Making a draw...");

        let users_response = self.db_client.clone().get_users(()).await?;

        let users = users_response.get_ref().to_owned().users;

        let mut ind = random_range(0..users.len());
        let mut choice = &users[ind];

        let mut it = 0;

        while choice.id == self.config.admin {
            ind = random_range(0..users.len());
            choice = &users[ind];
            it += 1;
            if it == 10000000 {
                return Err("Can't choose winner".into());
            }
        }

        let new_message = NewMessage {
            user: Some(choice.clone()),
            text: string_format!(self.config.draw_win_fmt.clone(), choice.username.clone()),
        };

        self.platform_client
            .clone()
            .send_message(new_message)
            .await?;

        let results_fmt = string_format!(
            self.config.draw_results_fmt.clone(),
            choice.username.clone()
        );

        let admin = User {
            id: self.config.admin.clone(),
            username: "admin".into(),
            city: "".into(),
            areas_of_interest: "".into(),
        };

        let new_message = NewMessage {
            user: Some(admin),
            text: results_fmt.clone(),
        };

        self.platform_client
            .clone()
            .send_message(new_message)
            .await?;

        let channel: User = User {
            id: self.config.channel.clone(),
            username: "channel".into(),
            city: "".into(),
            areas_of_interest: "".into(),
        };

        let new_message = NewMessage {
            user: Some(channel),
            text: results_fmt,
        };

        self.platform_client
            .clone()
            .send_message(new_message)
            .await?;

        Ok(())
    }
}
