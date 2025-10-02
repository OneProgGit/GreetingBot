use std::env;

use meowbot_proto::generated::weather::weather_server::WeatherServer;
use tonic::transport::Server;

use crate::wttr_in::WttrIn;

mod wttr_in;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenvy::from_filename("bin/meowbot-weather/.env").ok();

    let weather_addr = env::var("WEATHER_ADDR").expect("WEATHER_ADDR must be set!");
    let wttr_in = WttrIn;

    println!("Serving WEATHER at {weather_addr}...");

    Server::builder()
        .add_service(WeatherServer::new(wttr_in))
        .serve(weather_addr.parse()?)
        .await?;

    Ok(())
}
