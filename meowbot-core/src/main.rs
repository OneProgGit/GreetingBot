use std::{env, sync::Arc, time::Duration};

use meowbot_proto::generated::{
    ai::ai_client::AiClient, commands::command_handler_server::CommandHandlerServer,
    db::db_client::DbClient, platform::platform_client::PlatformClient,
    weather::weather_client::WeatherClient,
};
use tokio::time::sleep;
use tonic::transport::Server;

use crate::{app::App, config::load_config};

mod app;
mod config;
mod formats;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenvy::from_filename("bin/meowbot-core/.env").ok();

    let ai_addr = env::var("AI_ADDR").expect("AI_ADDR must be set!");

    let ai_client;
    loop {
        println!("Connecting to AI at {ai_addr}...");
        match AiClient::connect(ai_addr.clone()).await {
            Ok(c) => {
                ai_client = c;
                break;
            }
            Err(_) => {
                sleep(Duration::from_secs(5)).await;
                continue;
            }
        }
    }
    println!("Connected to AI at {ai_addr}...");

    let db_addr = env::var("DB_ADDR").expect("DB_ADDR must be set!");
    let db_client;
    loop {
        println!("Connecting to DB at {db_addr}...");
        match DbClient::connect(db_addr.clone()).await {
            Ok(c) => {
                db_client = c;
                break;
            }
            Err(_) => {
                sleep(Duration::from_secs(5)).await;
                continue;
            }
        }
    }
    println!("Connected to DB at {db_addr}...");

    let platform_addr = env::var("PLATFORM_ADDR").expect("PLATFORM_ADDR must be set!");

    let platform_client;
    loop {
        println!("Connecting to PLATFORM at {platform_addr}...");
        match PlatformClient::connect(platform_addr.clone()).await {
            Ok(c) => {
                platform_client = c;
                break;
            }
            Err(_) => {
                sleep(Duration::from_secs(5)).await;
                continue;
            }
        }
    }
    println!("Connected to PLATFORM at {platform_addr}...");

    let weather_addr = env::var("WEATHER_ADDR").expect("WEATHER_ADDR must be set!");

    let weather_client;
    loop {
        println!("Connecting to WEATHER at {weather_addr}...");
        match WeatherClient::connect(weather_addr.clone()).await {
            Ok(c) => {
                weather_client = c;
                break;
            }
            Err(_) => {
                sleep(Duration::from_secs(5)).await;
                continue;
            }
        }
    }
    println!("Connected to WEATHER at {weather_addr}...");

    let config_path = env::var("CONFIG_PATH").expect("CONFIG_PATH must be set!");

    println!("Loading config at {config_path}");

    let config = load_config(&config_path).expect("Failed to load config");

    let app = App::new(
        ai_client,
        db_client,
        platform_client,
        weather_client,
        config,
    );

    let arc_app = Arc::new(app.clone());

    println!("Binding all commands...");

    app.clone().bind_all_commands().await;

    println!("Scheduling all tasks...");

    arc_app.schedule_all_tasks().await;

    let commands_addr = env::var("COMMANDS_ADDR").expect("COMMANDS_ADDR must be set!");

    println!("Serving COMMANDS at {commands_addr}...");

    Server::builder()
        .add_service(CommandHandlerServer::new(app))
        .serve(commands_addr.parse()?)
        .await?;

    Ok(())
}
