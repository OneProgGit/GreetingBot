use std::sync::Arc;

use meowbot_proto::generated::{
    db::db_server::Db,
    models::{User, UsersList},
};
use sqlx::{Row, SqlitePool, sqlite::SqlitePoolOptions};
use tonic::{Request, Response, Status};

pub struct Sqlite {
    pool: Arc<SqlitePool>,
}

impl Sqlite {
    pub async fn new(url: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let pool = SqlitePoolOptions::new()
            .max_connections(10)
            .connect(url)
            .await?;
        Ok(Self {
            pool: Arc::new(pool),
        })
    }
}

#[tonic::async_trait]
impl Db for Sqlite {
    async fn create_user(&self, user: Request<User>) -> Result<Response<()>, Status> {
        println!("Creating user {user:?}...");

        let user = user.get_ref().to_owned();

        sqlx::query(
            "INSERT INTO users (id, username)
                VALUES (?1, ?2)
                ON CONFLICT(id) DO UPDATE SET
                    username = excluded.username",
        )
        .bind(user.id)
        .bind(user.username)
        .execute(&*self.pool)
        .await
        .expect("Failed to create user");
        Ok(Response::new(()))
    }

    async fn get_users(&self, _request: Request<()>) -> Result<Response<UsersList>, Status> {
        let rows = sqlx::query("SELECT id, username FROM users")
            .fetch_all(&*self.pool)
            .await
            .expect("Failed to get users");
        let users = rows
            .into_iter()
            .map(|row| User {
                id: row.get::<String, _>("id"),
                username: row.get::<String, _>("username"),
            })
            .collect();
        Ok(Response::new(UsersList { users }))
    }
}
