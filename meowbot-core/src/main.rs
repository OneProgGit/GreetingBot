use std::{env, sync::Arc};

use meowbot_proto::generated::{
    ai::ai_client::AiClient, commands::command_handler_server::CommandHandlerServer,
    db::db_client::DbClient, platform::platform_client::PlatformClient,
    weather::weather_client::WeatherClient,
};
use tonic::transport::Server;

use crate::{app::App, config::load_config};

mod app;
mod config;
mod formats;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenvy::from_filename("bin/meowbot-core/.env").ok();

    let ai_addr = env::var("AI_ADDR").expect("AI_ADDR must be set!");

    println!("Connecting to AI at {ai_addr}...");

    let ai_client = AiClient::connect(ai_addr).await?;

    let db_addr = env::var("DB_ADDR").expect("DB_ADDR must be set!");

    println!("Connecting to DB at {db_addr}...");

    let db_client = DbClient::connect(db_addr).await?;

    let platform_addr = env::var("PLATFORM_ADDR").expect("PLATFORM_ADDR must be set!");

    println!("Connecting to PLATFORM at {platform_addr}");

    let platform_client = PlatformClient::connect(platform_addr).await?;

    let weather_addr = env::var("WEATHER_ADDR").expect("WEATHER_ADDR must be set!");

    println!("Connecting to WEATHER at {weather_addr}");

    let weather_client = WeatherClient::connect(weather_addr).await?;

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

    arc_app.clone().bind_all_commands().await;

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
