use crate::{
    AI, DB, PLATFORM, WEATHER, handlers::date::format_datetime_russian, models::user::User,
    tools::config::CONFIG,
};
use chrono::Utc;
use string_format::string_format;

#[tracing::instrument]
async fn process_user(user: User, weather: String) {
    let response = AI
        .process(weather.clone())
        .await
        .unwrap_or(CONFIG.ai_msg_off.clone());

    let now = Utc::now();

    PLATFORM
        .clone()
        .send_message(
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
}

#[tracing::instrument]
pub async fn daily_message() {
    let weather = WEATHER
        .clone()
        .get_weather()
        .await
        .unwrap_or_else(|_| "Пасмурно".into());

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
