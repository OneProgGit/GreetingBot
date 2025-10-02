use std::env;

use meowbot_proto::generated::ai::ai_server::AiServer;
use tonic::transport::Server;

use crate::ollama::Ollama;

mod ollama;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenvy::from_filename("bin/meowbot-ai/.env").ok();

    let ai_addr = env::var("AI_ADDR").expect("AI_ADDR mus be set!");
    let ollama = Ollama;

    println!("Serving AI at {ai_addr}...");

    Server::builder()
        .add_service(AiServer::new(ollama))
        .serve(ai_addr.parse()?)
        .await?;

    Ok(())
}
