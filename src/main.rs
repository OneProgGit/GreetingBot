//! Program's entry point.
//! TODO: Rewrite some parts of architecture

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
    log::info!(
        "Handling user @{} (full name {})...",
        user.username,
        user.full_name
    );
    let response = ai::process_ollama(weather.clone())
        .await
        .unwrap_or(CONFIG.ai_msg_off.clone());
    log::info!(
        "Ai's response for user @{} (full name {}) is `{response}`",
        user.username,
        user.full_name
    );
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
    log::info!(
        "Message sent success to user @{} (full name {})",
        user.username,
        user.full_name
    );
}

/// Processes a chat
async fn process_chat(chat: Chat, weather: String) {
    log::info!("Handling chat @{}...", chat.username);
    let response = ai::process_ollama(weather.clone())
        .await
        .unwrap_or(CONFIG.ai_msg_off.clone());
    log::info!(
        "Ai's response for chat @{} (full name {}) is `{response}`",
        chat.username,
        chat.full_name
    );
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
    log::info!(
        "Message sent success to chat @{} (full name {})",
        chat.username,
        chat.full_name
    );
}

/// Send a daily messages for all users
async fn daily_message() {
    log::info!("Daily message time!");

    log::info!("Getting weather...");

    let weather = get_weather().await.unwrap_or_else(|err| {
        log::error!("Could not get weather: `{err}`");
        "Пасмурно".into()
    });

    log::info!("Handling all users...");

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
    log::info!("Draw time!");
    log::info!("Getting a random user...");

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

    log::info!(
        "The winner is @{} (full name {})!",
        choice.username,
        choice.full_name
    );

    BOT.send_message(
        UserId(choice.id),
        string_format!(CONFIG.draw_win_fmt.clone(), choice.username.clone()),
    )
    .parse_mode(ParseMode::Html)
    .await
    .expect("Send message failed");

    BOT.send_message(
        UserId(CONFIG.admin),
        string_format!(
            CONFIG.draw_admin_fmt.clone(),
            choice.username.clone(),
            choice.full_name.clone()
        ),
    )
    .parse_mode(ParseMode::Html)
    .await
    .expect("Send message failed");

    log::info!(
        "Message sent success to user @{} (full name {})",
        choice.username,
        choice.full_name
    );
}

pub static BOT: LazyLock<Bot> = LazyLock::new(Bot::from_env);

pub static DB: LazyLock<Database> =
    LazyLock::new(|| Database::from_config().expect("DB connection failed"));

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();
    pretty_env_logger::init();

    log::info!("Making some magic with panic...");

    pretty_panic();

    let mut cron = AsyncCron::new(Utc);

    log::info!("Creating daily message task...");

    let daily = cron
        .add_fn(&CONFIG.greeting_date_cron, || async {
            log::info!("Daily msg");
            daily_message().await;
        })
        .await
        .expect("Failed to daily");

    log::info!("Creating draw task...");

    let draw = cron
        .add_fn(&CONFIG.draw_date_cron, || async {
            log::info!("Draw");
            draw().await;
        })
        .await
        .expect("Failed to draw");

    log::info!("Starting tasks...");

    cron.start().await;

    log::info!("Daily job id: {daily}. Draw job id: {draw}");

    log::info!("Starting replier...");

    teloxide::repl(BOT.clone(), move |bot: Bot, msg: Message| {
        let db = DB.clone();

        async move {
            let username = msg.chat.username().unwrap_or("user").to_string();
            let full_name = msg.chat.first_name().unwrap_or("").to_string()
                + " "
                + (msg.chat.last_name().unwrap_or(""));

            log::info!("Handling a message from @{username} (full name {full_name})...");
            if let Some(message) = msg.text() {
                log::info!("Text from @{username} (full name {full_name}): `{message}`");
                bot.send_message(
                    msg.chat.id,
                    string_format!(
                        CONFIG.start_fmt.clone(),
                        username.clone(),
                        msg.chat.id.to_string()
                    ),
                )
                .parse_mode(ParseMode::Html)
                .await?;
                db.create_user(msg.chat.id.0, &username, &full_name)
                    .expect("Error accessing to database");
            }
            Ok(())
        }
    })
    .await;

    log::info!("Bot replies to the messages now!");
}
