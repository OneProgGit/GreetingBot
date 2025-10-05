use std::sync::Arc;

use meowbot_proto::generated::{
    db::{GetUserMessage, db_server::Db},
    models::{User, UsersList},
};
use sqlx::{Row, SqlitePool, sqlite::SqlitePoolOptions};
use tonic::{Request, Response, Status};

#[derive(sqlx::FromRow, Debug)]
struct DbUser {
    id: String,
    username: String,
    city: String,
    areas_of_interest: String,
}

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
        let user = user.get_ref().to_owned();
        println!("Creating user {user:?}...");

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

    async fn get_user(&self, request: Request<GetUserMessage>) -> Result<Response<User>, Status> {
        let request = request.get_ref().to_owned();
        println!("Getting user by id {}...", request.id);

        let db_user = sqlx::query_as::<_, DbUser>("SELECT * FROM users WHERE id = ?1")
            .bind(request.id)
            .fetch_one(&*self.pool)
            .await
            .expect("Failed to get user");

        Ok(Response::new(User {
            id: db_user.id,
            username: db_user.username,
            city: db_user.city,
            areas_of_interest: db_user.areas_of_interest,
        }))
    }

    async fn update_user(&self, user: Request<User>) -> Result<Response<()>, Status> {
        let user = user.get_ref().to_owned();
        println!("Updating user {user:?}...");

        sqlx::query(
            "UPDATE users SET username = ?1, city = ?2, areas_of_interest = ?3 WHERE id = ?4",
        )
        .bind(user.username)
        .bind(user.city)
        .bind(user.areas_of_interest)
        .bind(user.id)
        .execute(&*self.pool)
        .await
        .expect("Failed to update user");

        Ok(Response::new(()))
    }

    async fn get_users(&self, _request: Request<()>) -> Result<Response<UsersList>, Status> {
        println!("Getting users...");

        let rows = sqlx::query("SELECT * FROM users")
            .fetch_all(&*self.pool)
            .await
            .expect("Failed to get users");
        let users = rows
            .into_iter()
            .map(|row| User {
                id: row.get::<String, _>("id"),
                username: row.get::<String, _>("username"),
                city: row.get::<String, _>("city"),
                areas_of_interest: row.get::<String, _>("areas_of_interest"),
            })
            .collect();
        Ok(Response::new(UsersList { users }))
    }
}
