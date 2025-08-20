use crate::{
    handlers::{commands::bind_all_commands, scheduler::schedule_all_tasks},
    infra::{
        ai::AiProvider, database::Database, ollama::OllamaAi, sqlite::SqliteDb,
        weather::WeatherHandler, wttr_in::WttrInWetherHandler,
    },
    models::traits::{Create, CreateAsync},
    platforms::{platform::Platform, telegram::Telegram},
};
use std::sync::{Arc, LazyLock};

mod handlers;
mod infra;
mod models;
mod platforms;
mod tools;

pub static PLATFORM: LazyLock<Arc<dyn Platform>> = LazyLock::new(|| Telegram::new().unwrap());

pub static DB: LazyLock<Arc<dyn Database>> = LazyLock::new({
    SqliteDb::new()
        .await // TODO: Fix it
        .expect("Failed to connect to database")
});

pub static AI: LazyLock<Arc<dyn AiProvider>> = LazyLock::new(|| OllamaAi::new().unwrap());

pub static WEATHER: LazyLock<Arc<dyn WeatherHandler>> =
    LazyLock::new(|| WttrInWetherHandler::new().unwrap());

#[tracing::instrument]
#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();
    tracing_subscriber::fmt::init();
    bind_all_commands().await;
    schedule_all_tasks().await;
    PLATFORM.clone().run().await;
}
