use meowbot_proto::generated::{
    ai::ai_client::AiClient,
    db::{db_client::DbClient, platform_client::PlatformClient},
    weather::weather_client::WeatherClient,
};
use tonic::transport::Channel;

use crate::config::Configuration;

pub struct App {
    ai_client: AiClient<Channel>,
    db_client: DbClient<Channel>,
    platform_client: PlatformClient<Channel>,
    weather_client: WeatherClient<Channel>,
    config: Configuration,
}

impl App {
    pub fn new(
        ai_client: AiClient<Channel>,
        db_client: DbClient<Channel>,
        platform_client: PlatformClient<Channel>,
        weather_client: WeatherClient<Channel>,
        config: Configuration,
    ) -> Self {
        Self {
            ai_client,
            db_client,
            platform_client,
            weather_client,
            config,
        }
    }
}
