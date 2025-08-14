use std::sync::{Arc, LazyLock};
use crate::{handlers::{commands::bind_all_commands, scheduler::schedule_all_tasks}, infra::{ai::AiProvider, database::Database, ollama::OllamaAi, sqlite::SqliteDb, weather::WeatherHandler, wttr_in::WttrInWetherHandler}, models::traits::Create, platforms::{platform::Platform, telegram::Telegram}, tools::panic_tweak::pretty_panic};

mod tools;
mod handlers;
mod infra;
mod models;
mod platforms;

pub static PLATFORM: LazyLock<Arc<dyn Platform>> = LazyLock::new(|| {
    Telegram::new().unwrap()
});

pub static DB: LazyLock<Arc<dyn Database>> = LazyLock::new(|| 
    SqliteDb::new().expect("Failed to connect to database")
);

pub static AI: LazyLock<Arc<dyn AiProvider>> = LazyLock::new(|| 
    OllamaAi::new().unwrap()
);

pub static WEATHER: LazyLock<Arc<dyn WeatherHandler>> = LazyLock::new(|| 
    WttrInWetherHandler::new().unwrap()
);

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();
    pretty_env_logger::init();
    pretty_panic();
    bind_all_commands().await;
    schedule_all_tasks().await;
    PLATFORM.clone().run().await;
}
