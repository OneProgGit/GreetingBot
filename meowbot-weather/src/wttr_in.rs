use meowbot_proto::generated::weather::{WeatherModel, weather_server::Weather};
use reqwest::Client;
use serde::Deserialize;
use tonic::{Request, Response, Status};

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

pub struct WttrIn;

#[tonic::async_trait]
impl Weather for WttrIn {
    async fn get_weather(&self, _: Request<()>) -> Result<Response<WeatherModel>, Status> {
        let client = Client::new();
        let result = client
            .get("https://wttr.in/Moscow?format=j1&lang=ru")
            .send()
            .await
            .expect("Failed to get weather response")
            .json::<WttrInWeatherResponse>()
            .await
            .expect("Failed to parse weather response");

        let current_cond = &result.current_condition[0];
        let weather_today = &result.weather[0];
        let status = current_cond.weather_desc.first().map_or("?", |v| &v.value);

        Ok(Response::new(WeatherModel {
            temp_c: current_cond.temp_c.clone(),
            feels_like_c: current_cond.feels_like_c.clone(),
            wind_speed_kmph: current_cond.wind_speed_kmph.clone(),
            min_temp_c: weather_today.min_temp_c.clone(),
            max_temp_c: weather_today.max_temp_c.clone(),
            status: status.to_string(),
        }))
    }
}
