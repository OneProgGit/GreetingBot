use std::env;

use meowbot_proto::generated::db::db_server::DbServer;
use tonic::transport::Server;

use crate::sqlite::Sqlite;

mod sqlite;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenvy::from_filename("bin/meowbot-db/.env").ok();

    let db_addr = env::var("DB_ADDR").expect("DB_ADDR must be set!");

    println!("Serving DB at {db_addr}...");

    let db_url = env::var("DB_URL").expect("DB_URL must be set!");

    let sqlite = Sqlite::new(&db_url).await?;
    Server::builder()
        .add_service(DbServer::new(sqlite))
        .serve(db_addr.parse()?)
        .await?;

    Ok(())
}
