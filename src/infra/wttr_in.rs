use std::{error::Error, sync::Arc};

use reqwest::Client;
use serde::Deserialize;

use crate::{
    infra::weather::WeatherHandler,
    models::{traits::Create, types::Res, weather::Weather},
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

#[derive(Debug)]
pub struct WttrInWetherHandler;

impl Create for WttrInWetherHandler {
    #[tracing::instrument]
    fn new() -> Res<Arc<Self>> {
        Ok(Arc::new(WttrInWetherHandler))
    }
}

#[async_trait::async_trait]
impl WeatherHandler for WttrInWetherHandler {
    #[tracing::instrument]
    async fn get_weather(&self) -> Result<Weather, Box<dyn Error>> {
        let client = Client::new();
        let result = client
            .get(CONFIG.weather_url.clone())
            .send()
            .await?
            .json::<WttrInWeatherResponse>()
            .await?;
        let current = &result.current_condition[0];
        let today = &result.weather[0];
        let status = current.weather_desc.first().map_or("?", |v| &v.value);
        Ok(Weather {
            temp_c: current.temp_c.clone(),
            feels_like_c: current.feels_like_c.clone(),
            wind_speed_kmph: current.wind_speed_kmph.clone(),
            min_temp_c: today.min_temp_c.clone(),
            max_temp_c: today.max_temp_c.clone(),
            status: status.to_string(),
        })
    }
}
