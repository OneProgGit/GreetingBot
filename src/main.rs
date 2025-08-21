use std::sync::{Arc, OnceLock};

use crate::{
    ai_mod::{ai::AiModule, ollama_ai::OllamaAi},
    db_mod::{database::DatabaseModule, sqlite_database::SqliteDatabase},
    handlers_mod::{
        bind_commands_handler::bind_all_commands, scheduler_handler::schedule_all_tasks,
    },
    platforms_mod::{platform::PlatformModule, telegram_platform::TelegramPlatform},
    traits_mod::create_traits::{Create, CreateAsync},
    weather_mod::{weather::WeatherModule, wttr_in_weather::WttrInWeather},
};

mod ai_mod;
mod db_mod;
mod handlers_mod;
mod models_mod;
mod platforms_mod;
mod tools_mod;
mod traits_mod;
mod types_mod;
mod weather_mod;

pub static PLATFORM: OnceLock<Arc<dyn PlatformModule>> = OnceLock::new();
pub static DB: OnceLock<Arc<dyn DatabaseModule>> = OnceLock::new();
pub static AI: OnceLock<Arc<dyn AiModule>> = OnceLock::new();
pub static WEATHER: OnceLock<Arc<dyn WeatherModule>> = OnceLock::new();

#[tracing::instrument]
#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();
    tracing_subscriber::fmt::init();

    PLATFORM
        .set(TelegramPlatform::new().expect("Failed to initialize platform"))
        .expect("Failed to set platform");

    DB.set(
        SqliteDatabase::new()
            .await
            .expect("Failed to initialize database"),
    )
    .expect("Failed to set database");

    AI.set(OllamaAi::new().expect("Failed to initialize AI"))
        .expect("Failed to set AI");

    WEATHER
        .set(WttrInWeather::new().expect("Failed to initialize weather"))
        .expect("Failed to set weather");

    bind_all_commands().await;
    schedule_all_tasks().await;

    PLATFORM
        .get()
        .expect("Failed to get platform instance")
        .clone()
        .run()
        .await;
}
