#![deny(warnings)]
#![deny(clippy::all)]
#![deny(clippy::pedantic)]
#![deny(clippy::nursery)]
#![deny(clippy::cargo)]
#![deny(rustdoc::all)]
#![deny(missing_docs)]
#![allow(clippy::multiple_crate_versions)]

//! Greeting Bot is a Telegram bot, which sends a message to all users in database with AI-generated text.
//! This file is an entry point.

use std::sync::{Arc, OnceLock};

mod handlers;
mod tools;

/// Platform module allows you send messages to users and bind commands.
/// # Example
/// Binding a command:
/// ```
/// PLATFORM
///     .get()
///     .except("Failed to get platform instance")
///     .bind("/start", |user: UserModel| Box::pin(handle_start(user)));
/// ```
/// Sending a message:
/// ```
/// fn handle_start(user: UserModel) {
///     PLATFORM
///         .get()
///         .except("Failed to get platform instance")
///         .send_message(user, format!("Hello, {}!", user.username));
/// }
/// ```
pub static PLATFORM: OnceLock<Arc<dyn PlatformModule>> = OnceLock::new();

/// Database module allows you create users and get all of them.
/// # Example
/// ```
/// let user = UserModel { id: "1234", username: "OneProg" };
/// let db = DB.get().except("Failed to get database instance");
///
/// db.create_user(user);
///
/// let users = db.get_users().except("Failed to get users");
/// assert!(users.contains(user));
/// ```
pub static DB: OnceLock<Arc<dyn DatabaseModule>> = OnceLock::new();

/// Ai module allows you generate response based on weather string.
/// # Example
/// ```
/// let weather = String::from("Rain");
/// let response = AI
///     .get()
///     .except("Failed to get Ai instance")
///     .process(weather);
///
/// println!("Response: {response}");
/// ```
pub static AI: OnceLock<Arc<dyn AiModule>> = OnceLock::new();

/// Weather module allows you get weather from provider.
/// # Example
/// ```
/// let weather = WEATHER
///     .get()
///     .except("Failed to get weather instance")
///     .get_weather();
///
/// println!("Today's wind speed: {}km/h", weather.wind_speed_kmph);
/// ```
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
