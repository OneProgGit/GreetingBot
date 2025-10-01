use std::env;

use dotenvy::dotenv;
use meowbot_proto::generated::{
    commands::command_handler_client::CommandHandlerClient,
    platform::platform_server::PlatformServer,
};
use tonic::transport::Server;

use crate::telegram_service::TelegramService;

mod telegram;
mod telegram_service;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();

    let commands_addr = env::var("COMMANDS_ADDR").expect("COMMANDS_ADDR must be set!");
    let commands_client = CommandHandlerClient::connect(commands_addr).await?;

    let platform_addr = env::var("PLATFORM_ADDR").expect("PLATFORM_ADDR must be set!");

    let tg = TelegramService::new(commands_client);
    let tg_clone = tg.clone();

    tokio::spawn(async move {
        Server::builder()
            .add_service(PlatformServer::new(tg))
            .serve(
                platform_addr
                    .parse()
                    .expect("Failed to parse platform address"),
            )
            .await
            .expect("Failed to serve");
    });

    tg_clone.run().await;

    Ok(())
}
