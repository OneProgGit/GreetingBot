use std::{error::Error, fmt::Debug};

use crate::{models_mod::weather_model::WeatherModel, traits_mod::create_traits::Create};

#[async_trait::async_trait]
pub trait Weather: Send + Sync + Create + Debug {
    async fn get_weather(&self) -> Result<WeatherModel, Box<dyn Error>>;
}
