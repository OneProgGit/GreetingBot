//! Program's entry point.
//! TODO: Rewrite some parts of architecture

use chrono::Utc;
use cron_tab::AsyncCron;
use rand::random_range;
use std::sync::{Arc, LazyLock};
use string_format::string_format;
use teloxide::{prelude::*, types::ParseMode};

use crate::platforms::{platform::Platform, telegram::Telegram};

mod tools;
mod handlers;
mod infra;
mod models;
mod platforms;

pub static PLATFORM: LazyLock<Arc<dyn Platform>> = LazyLock::new(|| {
    tokio::runtime::Handle::current().block_on(Telegram::new())
});

pub static DB: LazyLock<Database> =
    LazyLock::new(|| Database::from_config().expect("DB connection failed"));

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();
    pretty_env_logger::init();

    // let tg =
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
