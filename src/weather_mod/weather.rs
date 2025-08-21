use std::fmt::Debug;

use crate::{
    models_mod::weather_model::WeatherModel, traits_mod::create_traits::Create,
    types_mod::result_types::Res,
};

#[async_trait::async_trait]
pub trait WeatherModule: Send + Sync + Create + Debug {
    async fn get_weather(&self) -> Res<WeatherModel>;
}
