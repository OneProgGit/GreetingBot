use std::{env, time::Duration};

use meowbot_proto::generated::{
    commands::command_handler_client::CommandHandlerClient,
    platform::platform_server::PlatformServer,
};
use tokio::time::sleep;
use tonic::transport::Server;

use crate::telegram_service::TelegramService;

mod telegram;
mod telegram_service;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenvy::from_filename("bin/meowbot-platform/.env").ok();

    let commands_addr = env::var("COMMANDS_ADDR").expect("COMMANDS_ADDR must be set!");
    let platform_addr = env::var("PLATFORM_ADDR").expect("PLATFORM_ADDR must be set!");

    let tg = TelegramService::new();
    let tg_clone = tg.clone();

    println!("Serving PLATFORM at {platform_addr}...");

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

    let commands_client;
    loop {
        println!("Connecting to COMMANDS at {commands_addr}...");
        match CommandHandlerClient::connect(commands_addr.clone()).await {
            Ok(c) => {
                commands_client = c;
                break;
            }
            Err(_) => {
                sleep(Duration::from_secs(5)).await;
                continue;
            }
        }
    }
    println!("Connected to COMMANDS at {commands_addr}...");

    tg_clone.init(commands_client).await;
    tg_clone.run().await?;

    Ok(())
}
