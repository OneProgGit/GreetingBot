use crate::{
    handlers::{commands::bind_all_commands, scheduler::schedule_all_tasks},
    infra::{
        ai::AiProvider, database::Database, ollama::OllamaAi, sqlite::SqliteDb,
        weather::WeatherHandler, wttr_in::WttrInWetherHandler,
    },
    models::traits::Create,
    platforms::{platform::Platform, telegram::Telegram},
    tools::panic_tweak::pretty_panic,
};
use std::sync::{Arc, LazyLock};

mod handlers;
mod infra;
mod models;
mod platforms;
mod tools;

pub static PLATFORM: LazyLock<Arc<dyn Platform>> = LazyLock::new(|| {
    log::info!("Create platform instance");
    Telegram::new().unwrap()
});

pub static DB: LazyLock<Arc<dyn Database>> = LazyLock::new(|| {
    log::info!("Create database instance");
    SqliteDb::new().expect("Failed to connect to database")
});

pub static AI: LazyLock<Arc<dyn AiProvider>> = LazyLock::new(|| {
    log::info!("Create AI instance");
    OllamaAi::new().unwrap()
});

pub static WEATHER: LazyLock<Arc<dyn WeatherHandler>> = LazyLock::new(|| {
    log::info!("Create Weather instance");
    WttrInWetherHandler::new().unwrap()
});

#[tokio::main]
async fn main() {
    println!("Load .env");
    dotenvy::dotenv().ok();
    println!("Setup logger");
    pretty_env_logger::init();
    log::info!("Setup pretty panic");
    pretty_panic();
    log::info!("Bind commands");
    bind_all_commands().await;
    log::info!("Schedule tasks");
    schedule_all_tasks().await;
    log::info!("Start bot");
    PLATFORM.clone().run().await;
}
