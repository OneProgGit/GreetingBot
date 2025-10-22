use std::sync::Arc;

use meowbot_proto::generated::{
    commands::{HandleCommandMessage, command_handler_client::CommandHandlerClient},
    models::User,
    platform::NewMessage,
};
use teloxide::{
    Bot,
    payloads::SendMessageSetters,
    prelude::Requester,
    types::{Message, ParseMode},
};
use tokio::sync::Mutex;
use tonic::{Request, Response, Status, transport::Channel};

pub struct Telegram {
    bot: Arc<Bot>,
    commands_service: Mutex<Option<CommandHandlerClient<Channel>>>,
}

impl Telegram {
    pub fn new() -> Self {
        let bot = Bot::from_env();

        Self {
            bot: Arc::new(bot),
            commands_service: Mutex::new(None),
        }
    }

    pub async fn init(self: Arc<Self>, commands_service: CommandHandlerClient<Channel>) {
        *self.commands_service.lock().await = Some(commands_service);
    }

    async fn handle_message(
        self: Arc<Self>,
        user: User,
        msg: &str,
    ) -> Result<(), Box<dyn std::error::Error>> {
        println!("Handling message: '{msg}' from {user:?}...");

        let request = HandleCommandMessage {
            user: user.into(),
            command: msg.into(),
        };

        self.commands_service
            .lock()
            .await
            .clone()
            .ok_or_else(|| "Commands service must be set!")?
            .handle_command(request)
            .await?;

        Ok(())
    }

    pub async fn run(self: Arc<Self>) -> Result<(), Box<dyn std::error::Error>> {
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
                    .await
                    .expect("Failed to handle message");
                Ok(())
            }
        })
        .await;

        Ok(())
    }

    pub async fn send_message(
        &self,
        new_message: Request<NewMessage>,
    ) -> Result<Response<()>, Status> {
        println!("Sending message: {new_message:?}...");

        let new_message = new_message.get_ref().to_owned();
        let user = new_message
            .user
            .ok_or_else(|| Status::invalid_argument("Message must be set!"))?;

        self.bot
            .send_message(user.id, new_message.text)
            .parse_mode(ParseMode::Html)
            .await
            .map_err(|e| Status::from_error(Box::new(e)))?;

        Ok(Response::new(()))
    }
}
