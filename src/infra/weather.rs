use std::error::Error;

use crate::models::traits::Create;

#[async_trait::async_trait]
pub trait WeatherHandler: Send + Sync + Create {
    async fn get_weather(&self) -> Result<String, Box<dyn Error>>;
}
