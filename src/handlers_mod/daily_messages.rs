use crate::{
    AI, DB, PLATFORM, WEATHER,
    handlers_mod::{date::format_datetime_russian, formats::weather_to_emoji},
    models_mod::user_model::UserModel,
    tools_mod::config_tools::CONFIG,
};
use chrono::Utc;
use string_format::string_format;

#[tracing::instrument]
async fn process_user(user: UserModel, weather: String) {
    let response = AI
        .get()
        .expect("Failed to get AI instance")
        .clone()
        .process(weather.clone())
        .await
        .unwrap_or_else(|_| CONFIG.ai_msg_off.clone());

    let now = Utc::now();

    PLATFORM
        .get()
        .expect("Failed to get platform instance")
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
    let weather_struct = WEATHER
        .get()
        .expect("Failed to get weather instance")
        .clone()
        .get_weather()
        .await
        .expect("Failed to get weather");

    let formatted_weather = string_format!(
        CONFIG.weather_fmt.clone(),
        weather_struct.temp_c,
        weather_struct.feels_like_c,
        weather_struct.wind_speed_kmph,
        weather_struct.min_temp_c,
        weather_struct.max_temp_c,
        weather_to_emoji(&weather_struct.status),
        weather_struct.status
    );

    let users = DB
        .get()
        .expect("Failed to get DB instance")
        .clone()
        .get_users()
        .await
        .expect("Error while getting users");

    for user in users {
        tokio::spawn(process_user(user, formatted_weather.clone()));
    }

    let channel = UserModel {
        id: CONFIG.channel.clone(),
        username: "oneprogofficial".into(),
    };

    tokio::spawn(process_user(channel, formatted_weather.clone()));
}
