use std::sync::Arc;

use meowbot_proto::generated::{
    commands::command_handler_client::CommandHandlerClient,
    platform::{Command, NewMessage, platform_server::Platform},
};
use tonic::{Request, Response, Status, transport::Channel};

use crate::telegram::Telegram;

#[derive(Clone)]
pub struct TelegramService {
    tg: Arc<Telegram>,
}

impl TelegramService {
    pub fn new() -> Self {
        Self {
            tg: Arc::new(Telegram::new()),
        }
    }

    pub async fn init(&self, commands_service: CommandHandlerClient<Channel>) {
        self.tg.clone().init(commands_service).await;
    }

    pub async fn run(&self) {
        self.tg.clone().run().await;
    }
}

#[tonic::async_trait]
impl Platform for TelegramService {
    async fn send_message(&self, new_message: Request<NewMessage>) -> Result<Response<()>, Status> {
        self.tg.clone().send_message(new_message).await
    }

    async fn bind_start_command(&self, cmd: Request<Command>) -> Result<Response<()>, Status> {
        self.tg.clone().bind_start_command(cmd).await
    }
}
