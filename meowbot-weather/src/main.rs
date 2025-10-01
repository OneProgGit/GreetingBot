use std::env;

use dotenvy::dotenv;
use meowbot_proto::generated::weather::weather_server::WeatherServer;
use tonic::transport::Server;

use crate::wttr_in::WttrIn;

mod wttr_in;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();

    let weather_addr = env::var("WEATHER_ADDR").expect("WEATHER_ADDR must be set!");
    let wttr_in = WttrIn;

    Server::builder()
        .add_service(WeatherServer::new(wttr_in))
        .serve(weather_addr.parse()?)
        .await?;

    Ok(())
}
