use std::{error::Error, sync::Arc};

use reqwest::Client;
use serde::Deserialize;
use string_format::string_format;

use crate::{
    infra::weather::WeatherHandler,
    models::{traits::Create, types::Res},
    tools::config::CONFIG,
};

#[derive(Debug, Deserialize)]
struct WttrInWeatherResponse {
    current_condition: Vec<WttrInCondition>,
    weather: Vec<WttrInWeather>,
}

#[derive(Debug, Deserialize)]
struct WttrInCondition {
    #[serde(rename = "temp_C")]
    temp_c: String,
    #[serde(rename = "FeelsLikeC")]
    feels_like_c: String,
    #[serde(rename = "windspeedKmph")]
    wind_speed_kmph: String,
    #[serde(rename = "lang_ru")]
    weather_desc: Vec<WttrInLangValue>,
}

#[derive(Debug, Deserialize)]
struct WttrInWeather {
    #[serde(rename = "date")]
    _date: String,
    #[serde(rename = "maxtempC")]
    max_temp_c: String,
    #[serde(rename = "mintempC")]
    min_temp_c: String,
}

#[derive(Debug, Deserialize)]
struct WttrInLangValue {
    value: String,
}

pub struct WttrInWetherHandler;

impl Create for WttrInWetherHandler {
    fn new() -> Res<Arc<Self>> {
        Ok(Arc::new(WttrInWetherHandler))
    }
}

#[async_trait::async_trait]
impl WeatherHandler for WttrInWetherHandler {
    async fn get_weather(&self) -> Result<String, Box<dyn Error>> {
        log::info!("Get response from weather server `{}`", CONFIG.weather_url);
        let client = Client::new();
        let result = client
            .get(CONFIG.weather_url.clone())
            .send()
            .await?
            .json::<WttrInWeatherResponse>()
            .await?;
        log::info!("Parse weather response");
        let current = &result.current_condition[0];
        let today = &result.weather[0];
        let status = current.weather_desc.first().map_or("?", |v| &v.value);
        let status_char = weather_to_emoji(status);
        log::info!("Weather is done");
        Ok(string_format!(
            CONFIG.weather_fmt.clone(),
            current.temp_c.clone(),
            current.feels_like_c.clone(),
            current.wind_speed_kmph.clone(),
            status_char,
            status.into(),
            today.min_temp_c.clone(),
            today.max_temp_c.clone()
        ))
    }
}

// TODO: Move this stuff to handlers.
fn weather_to_emoji(desc: &str) -> String {
    let desc_lower = desc.to_lowercase();

    let patterns = vec![
        ("замерзающий", "🧊"),
        ("переменная", "☀️"),
        ("гроза", "⛈️"),
        ("дождь", "🌧️"),
        ("снег", "❄️"),
        ("слякоть", "🌨️"),
        ("град", "🌨️"),
        ("туман", "🌫️"),
        ("дымка", "🌫️"),
        ("ясно", "☀️"),
        ("облачно", "☁️"),
        ("пасмурно", "🌥️"),
    ];

    let mut ans = String::new();

    for (pattern, emoji) in &patterns {
        if desc_lower.contains(pattern) {
            ans += emoji;
        }
    }

    ans
}

#[cfg(test)]
mod weather_tests {
    use crate::infra::wttr_in::weather_to_emoji;

    #[test]
    fn test_weather_emoji_one_condition_lowercase() {
        assert_eq!(weather_to_emoji("гроза").as_str(), "⛈️");
        assert_eq!(weather_to_emoji("дождь").as_str(), "🌧️");
        assert_eq!(weather_to_emoji("снег").as_str(), "❄️");
        assert_eq!(weather_to_emoji("слякоть").as_str(), "🌨️");
        assert_eq!(weather_to_emoji("град").as_str(), "🌨️");
        assert_eq!(weather_to_emoji("туман").as_str(), "🌫️");
        assert_eq!(weather_to_emoji("дымка").as_str(), "🌫️");
        assert_eq!(weather_to_emoji("ясно").as_str(), "☀️");
        assert_eq!(weather_to_emoji("облачно").as_str(), "☁️");
        assert_eq!(weather_to_emoji("пасмурно").as_str(), "🌥️");
    }

    #[test]
    fn test_weather_emoji_one_condition_uppercase() {
        assert_eq!(weather_to_emoji("Гроза"), "⛈️");
        assert_eq!(weather_to_emoji("Дождь"), "🌧️");
        assert_eq!(weather_to_emoji("Снег"), "❄️");
        assert_eq!(weather_to_emoji("Слякоть"), "🌨️");
        assert_eq!(weather_to_emoji("Град"), "🌨️");
        assert_eq!(weather_to_emoji("Туман"), "🌫️");
        assert_eq!(weather_to_emoji("Дымка"), "🌫️");
        assert_eq!(weather_to_emoji("Ясно"), "☀️");
        assert_eq!(weather_to_emoji("Облачно"), "☁️");
        assert_eq!(weather_to_emoji("Пасмурно"), "🌥️");
    }

    #[test]
    fn test_weather_emoji_one_condition_dot() {
        assert_eq!(weather_to_emoji("гроза."), "⛈️");
        assert_eq!(weather_to_emoji("дождь."), "🌧️");
        assert_eq!(weather_to_emoji("снег."), "❄️");
        assert_eq!(weather_to_emoji("слякоть."), "🌨️");
        assert_eq!(weather_to_emoji("град."), "🌨️");
        assert_eq!(weather_to_emoji("туман."), "🌫️");
        assert_eq!(weather_to_emoji("дымка."), "🌫️");
        assert_eq!(weather_to_emoji("ясно."), "☀️");
        assert_eq!(weather_to_emoji("облачно."), "☁️");
        assert_eq!(weather_to_emoji("пасмурно."), "🌥️");
    }

    #[test]
    fn test_weather_emoji_several_conditions() {
        assert_eq!(weather_to_emoji("пасмурно, снег"), "❄️🌥️");
        assert_eq!(weather_to_emoji("снег, пасмурно"), "❄️🌥️");
        assert_eq!(weather_to_emoji("ясно, замерзающий дождь"), "🧊🌧️☀️");
        assert_eq!(weather_to_emoji("переменная облачность").as_str(), "☀️☁️");
    }
}
