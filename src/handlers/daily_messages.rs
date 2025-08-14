use crate::{handlers::date::format_datetime_russian, models::user::User, tools::config::CONFIG, AI, DB, PLATFORM, WEATHER};
use chrono::Utc;
use string_format::string_format;

async fn process_user(user: User, weather: String) {
    log::info!(
        "Handling user @{}...",
        user.username,
    );
    let response = AI.process(weather.clone())
        .await
        .unwrap_or(CONFIG.ai_msg_off.clone());

    log::info!(
        "Ai's response for user @{} is `{response}`",
        user.username,
    );

    let now = Utc::now();

    PLATFORM.clone().send_message(
        user.clone(),
        &string_format!(
            CONFIG.greeting_fmt.clone(),
            user.username.clone(),
            format_datetime_russian(now.naive_local()),
            weather,
            response.clone()
        ),
    )
    .await
    .expect("Send message failed");
    log::info!(
        "Message sent success to user @{}",
        user.username,
    );
}

pub async fn daily_message() {
    log::info!("Daily message time!");
    log::info!("Getting weather...");

    let weather = WEATHER.clone().get_weather().await.unwrap_or_else(|err| {
        log::error!("Could not get weather: `{err}`");
        "Пасмурно".into()
    });

    log::info!("Handling all users...");

    let users = DB.clone().get_users().expect("Error while getting users");

    for user in users {
        tokio::spawn(process_user(user, weather.clone()));
    }

    let channel = User {
        id: CONFIG.channel.clone(),
        username: "oneprogofficial".into(),
    };

    tokio::spawn(process_user(channel, weather.clone()));
}
