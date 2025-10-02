use std::{collections::HashMap, sync::Arc};

use meowbot_proto::generated::{
    commands::command_handler_client::CommandHandlerClient,
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

enum Bind {
    Start,
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
        if let Some(cmd) = self.binds.lock().await.get(msg) {
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
        let new_message = new_message.get_ref().to_owned();
        let user = new_message.user.expect("User must be set!");

        self.bot
            .send_message(user.id, new_message.text)
            .parse_mode(ParseMode::Html)
            .await
            .expect("Failed to send message");

        Ok(Response::new(()))
    }

    pub async fn bind_start_command(
        self: Arc<Self>,
        cmd: Request<Command>,
    ) -> Result<Response<()>, Status> {
        let cmd = cmd.get_ref().to_owned();
        let mut cmd_text = String::from("/");
        cmd_text.push_str(&cmd.text);

        self.binds.lock().await.insert(cmd_text, Bind::Start);
        Ok(Response::new(()))
    }
}
