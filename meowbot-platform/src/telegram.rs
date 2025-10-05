use std::{collections::HashMap, sync::Arc};

use meowbot_proto::generated::{
    commands::{ChangeAreasOfInterest, ChangeCity, command_handler_client::CommandHandlerClient},
    models::User,
    platform::{Command, NewMessage},
};
use teloxide::{
    Bot,
    payloads::SendMessageSetters,
    prelude::Requester,
    types::{Message, ParseMode},
};
use tokio::sync::Mutex;
use tonic::{Request, Response, Status, transport::Channel};

pub enum Bind {
    Start,
    ChangeCity,
    ChangeAreasOfInterest,
}

pub struct Telegram {
    bot: Arc<Bot>,
    binds: Mutex<HashMap<String, Bind>>,
    commands_service: Mutex<Option<CommandHandlerClient<Channel>>>,
}

impl Telegram {
    pub fn new() -> Self {
        let bot = Bot::from_env();

        Self {
            bot: Arc::new(bot),
            binds: Mutex::new(HashMap::new()),
            commands_service: Mutex::new(None),
        }
    }

    pub async fn init(self: Arc<Self>, commands_service: CommandHandlerClient<Channel>) {
        *self.commands_service.lock().await = Some(commands_service);
    }

    async fn handle_message(self: Arc<Self>, user: User, msg: &str) {
        println!("Handling message: '{msg}' from {user:?}...");

        let msgs: Vec<&str> = msg.split(" ").collect();

        if let Some(cmd) = self.binds.lock().await.get(msgs[0]) {
            match *cmd {
                Bind::Start => {
                    self.commands_service
                        .lock()
                        .await
                        .clone()
                        .expect("Commands service must be set!")
                        .handle_start(user)
                        .await
                        .expect("Failed to handle start");
                }
                Bind::ChangeCity => {
                    let change_city = ChangeCity {
                        user: Some(user),
                        new_city: msgs[1].into(),
                    };

                    self.commands_service
                        .lock()
                        .await
                        .clone()
                        .expect("Commands service must be set!")
                        .handle_change_city(change_city)
                        .await
                        .expect("Failed to handle start");
                }
                Bind::ChangeAreasOfInterest => {
                    let change_areas_of_interest = ChangeAreasOfInterest {
                        user: Some(user),
                        new_areas_of_interest: msgs[1].into(),
                    };

                    self.commands_service
                        .lock()
                        .await
                        .clone()
                        .expect("Commands service must be set!")
                        .handle_change_areas_of_interest(change_areas_of_interest)
                        .await
                        .expect("Failed to handle start");
                }
            }
        } else {
            let new_message = NewMessage {
                user: Some(user),
                text: "Неизвестная команда!".into(),
            };

            self.send_message(Request::new(new_message))
                .await
                .expect("Failed to send message");
        }
    }

    pub async fn run(self: Arc<Self>) {
        let tg = Arc::clone(&self);
        let bot = tg.bot.clone();

        teloxide::repl(bot, move |_bot: Arc<Bot>, msg: Message| {
            let tg = Arc::clone(&tg);
            let user = User {
                id: msg.chat.id.0.to_string(),
                username: msg
                    .chat
                    .username()
                    .unwrap_or(&format!("User {}", msg.chat.id.0))
                    .to_string(),
                city: "".into(),
                areas_of_interest: "".into(),
            };

            async move {
                let tg = Arc::clone(&tg);
                tg.handle_message(user, msg.text().unwrap_or("No text"))
                    .await;
                Ok(())
            }
        })
        .await;
    }

    pub async fn send_message(
        &self,
        new_message: Request<NewMessage>,
    ) -> Result<Response<()>, Status> {
        println!("Sending message: {new_message:?}...");

        let new_message = new_message.get_ref().to_owned();
        let user = new_message.user.expect("User must be set!");

        self.bot
            .send_message(user.id, new_message.text)
            .parse_mode(ParseMode::Html)
            .await
            .expect("Failed to send message");

        Ok(Response::new(()))
    }

    pub async fn bind_command(
        self: Arc<Self>,
        bind: Bind,
        cmd: Request<Command>,
    ) -> Result<Response<()>, Status> {
        let cmd = cmd.get_ref().to_owned();
        let mut cmd_text = String::from("/");
        cmd_text.push_str(&cmd.text);

        self.binds.lock().await.insert(cmd_text, bind);
        Ok(Response::new(()))
    }
}
