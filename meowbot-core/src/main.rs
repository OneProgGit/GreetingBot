use std::env;

use dotenvy::dotenv;
use meowbot_proto::generated::{
    ai::ai_client::AiClient,
    db::{db_client::DbClient, platform_client::PlatformClient},
    weather::weather_client::WeatherClient,
};

use crate::{app::App, config::load_config};

mod app;
mod config;
mod formats;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();

    let ai_addr = env::var("AI_ADDR").expect("AI_ADDR must be set!");
    let ai_client = AiClient::connect(ai_addr).await?;

    let db_addr = env::var("DB_ADDR").expect("DB_ADDR must be set!");
    let db_client = DbClient::connect(db_addr).await?;

    let platform_addr = env::var("PLATFORM_ADDR").expect("PLATFORM_ADDR must be set!");
    let platform_client = PlatformClient::connect(platform_addr).await?;

    let weather_addr = env::var("WEATHER_ADDR").expect("WEATHER_ADDR must be set!");
    let weather_client = WeatherClient::connect(weather_addr).await?;

    let config_path = env::var("CONFIG_PATH").expect("CONFIG_PATH must be set!");
    let config = load_config(&config_path).expect("Failed to load config");

    let mut app = App::new(
        ai_client,
        db_client,
        platform_client,
        weather_client,
        config,
    );

    app.bind_all_commands();
    app.schedule_all_tasks();

    Ok(())
}
