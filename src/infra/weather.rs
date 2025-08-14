use std::error::Error;

use crate::models::{traits::Create, weather::Weather};

#[async_trait::async_trait]
pub trait WeatherHandler: Send + Sync + Create {
    async fn get_weather(&self) -> Result<Weather, Box<dyn Error>>;
}