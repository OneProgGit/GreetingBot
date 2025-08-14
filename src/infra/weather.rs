use std::error::Error;

pub trait WeatherHandler {
    async fn get_weather() -> Result<String, Box<dyn Error>>;
}