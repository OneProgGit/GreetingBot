//! Program's entry point.

use chrono::Utc;
use cron_tab::AsyncCron;
use rand::random_range;
use std::sync::LazyLock;
use string_format::string_format;
use teloxide::{prelude::*, types::ParseMode};

use crate::{
    config::CONFIG,
    date::format_datetime_russian,
    db::{Chat, Database, User},
    panic_tweak::pretty_panic,
    weather::get_weather,
};

mod ai;
mod config;
mod date;
mod db;
mod panic_tweak;
mod weather;

/// Processes a user
async fn process_user(user: User, weather: String) {
    log::info!("Processing user @{}...", user.username);
    let response = ai::process_ollama(weather.clone())
        .await
        .unwrap_or(CONFIG.ai_msg_off.clone());
    log::info!("{response}");
    let now = Utc::now();

    BOT.send_message(
        UserId(user.id),
        string_format!(
            CONFIG.greeting_fmt.clone(),
            user.username.clone(),
            format_datetime_russian(now.naive_local()),
            weather,
            response.clone()
        ),
    )
    .parse_mode(ParseMode::Html)
    .await
    .expect("Send message failed");
    log::info!("Message sent success to user @{}", user.username);
}

/// Processes a chat
async fn process_chat(chat: Chat, weather: String) {
    log::info!("Processing chat @{}...", chat.username);
    let response = ai::process_ollama(weather.clone())
        .await
        .unwrap_or(CONFIG.ai_msg_off.clone());
    log::info!("{response}");
    let now = Utc::now();

    BOT.send_message(
        ChatId(chat.id),
        string_format!(
            CONFIG.greeting_fmt.clone(),
            chat.username.clone(),
            format_datetime_russian(now.naive_local()),
            weather,
            response.clone()
        ),
    )
    .parse_mode(ParseMode::Html)
    .await
    .expect("Send message failed");
    log::info!("Message sent success to chat @{}", chat.username);
}

/// Send a daily messages for all users
async fn daily_message() {
    log::info!("Processing all users...");

    let weather = get_weather().await.unwrap_or_else(|err| {
        log::error!("Could not get weather: `{err}`");
        "Пасмурно".into()
    });

    let users = DB.clone().get_users().expect("Error while getting users");

    for user in users {
        tokio::spawn(process_user(user, weather.clone()));
    }

    let channel = Chat {
        id: CONFIG.channel,
        username: "oneprogofficial".into(),
        full_name: "OneProg".into(),
    };

    tokio::spawn(process_chat(channel, weather.clone()));
}

/// Chooses a random user to make a draw
async fn draw() {
    let users = DB.clone().get_users().expect("Error while getting users");
    let mut ind = random_range(0..users.len());
    let mut choice = &users[ind];

    let mut it = 0;

    while choice.id == CONFIG.admin {
        ind = random_range(0..users.len());
        choice = &users[ind];
        it += 1;
        if it == 10000000 {
            panic!("Unluckly, can't choose the winner");
        }
    }

    BOT.send_message(
        UserId(choice.id),
        string_format!(CONFIG.draw_win_fmt.clone(), choice.username.clone()),
    )
    .parse_mode(ParseMode::Html)
    .await
    .expect("Send message failed");

    BOT.send_message(
        UserId(CONFIG.admin),
        string_format!(CONFIG.draw_admin_fmt.clone(), choice.username.clone()),
    )
    .parse_mode(ParseMode::Html)
    .await
    .expect("Send message failed");

    log::info!("Message sent success to user @{}", choice.username);
}

pub static BOT: LazyLock<Bot> = LazyLock::new(Bot::from_env);

pub static DB: LazyLock<Database> =
    LazyLock::new(|| Database::from_config().expect("DB connection failed"));

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();
    pretty_env_logger::init();
    pretty_panic();

    log::info!("Starting 💝 telegram bot");

    let mut cron = AsyncCron::new(Utc);

    let daily = cron
        .add_fn(&CONFIG.greeting_date_cron, || async {
            log::info!("Daily msg");
            daily_message().await;
        })
        .await
        .expect("Failed to daily");

    let draw = cron
        .add_fn(&CONFIG.draw_date_cron, || async {
            log::info!("Draw");
            draw().await;
        })
        .await
        .expect("Failed to draw");

    cron.start().await;
    log::info!("Daily job: {daily} & Draw job: {draw}");

    teloxide::repl(BOT.clone(), move |bot: Bot, msg: Message| {
        let db = DB.clone();

        async move {
            if let Some(_message) = msg.text() {
                bot.send_message(
                    msg.chat.id,
                    string_format!(
                        CONFIG.start_fmt.clone(),
                        msg.chat.username().unwrap_or("user").to_string(),
                        msg.chat.id.to_string()
                    ),
                )
                .parse_mode(ParseMode::Html)
                .await?;
                let full_name = msg.chat.first_name().unwrap_or("").to_string()
                    + " "
                    + (msg.chat.last_name().unwrap_or(""));
                db.create_user(
                    msg.chat.id.0,
                    msg.chat.username().unwrap_or("user"),
                    &full_name,
                )
                .expect("Error accessing to database");
            }
            Ok(())
        }
    })
    .await;
}
