//! Program's entry point.
//! TODO: Rewrite some parts of architecture

use chrono::Utc;
use cron_tab::AsyncCron;
use rand::random_range;
use std::sync::{Arc, LazyLock};
use string_format::string_format;
use teloxide::{prelude::*, types::ParseMode};

use crate::{handlers::scheduler::schedule_all_tasks, infra::{ai::AiProvider, database::Database, ollama::OllamaAi, sqlite::SqliteDb, weather::WeatherHandler, wttr_in::WttrInWetherHandler}, models::traits::Create, platforms::{platform::Platform, telegram::Telegram}, tools::{config::CONFIG, panic_tweak::pretty_panic}};

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
    schedule_all_tasks().await;
    PLATFORM.clone().run().await;
}
