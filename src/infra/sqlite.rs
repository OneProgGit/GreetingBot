use sqlx::{Executor, SqlitePool, pool::PoolOptions, sqlite::SqlitePoolOptions};

use crate::{
    infra::database::Database,
    models::{traits::CreateAsync, types::Res, user::User},
    tools::config::CONFIG,
};

use std::sync::{Arc, Mutex};

#[derive(Clone, Debug)]
pub struct SqliteDb {
    pool: Arc<Mutex<SqlitePool>>,
}

impl CreateAsync for SqliteDb {
    #[tracing::instrument]
    async fn new() -> Res<Arc<Self>> {
        let pool = SqlitePoolOptions::new()
            .max_connections(5)
            .connect(&CONFIG.db_url)
            .await?;
        let db = Self {
            pool: Arc::new(Mutex::new(pool)),
        };
        Ok(Arc::new(db))
    }
}

impl Database for SqliteDb {
    #[tracing::instrument]
    async fn create_user(&self, user: User) -> Res<()> {
        // TODO: Fix it
        let pool = self.pool.lock().expect("Could not lock db_conn").clone();
        pool.execute(
            sqlx::query(
                "INSERT INTO users (id, username)
            VALUES (?1, ?2)
            ON CONFLICT(id) DO UPDATE SET
                username = excluded.username",
            )
            .bind(user.id)
            .bind(user.username),
        )
        .await?;
        Ok(())
    }

    #[tracing::instrument]
    async fn get_users(&self) -> Res<Vec<User>> {
        let conn = self.pool.lock().expect("Could not lock db_conn");
        let mut stmt = conn.prepare("SELECT id, username FROM users")?;
        let user_iter = stmt.query_map([], |row| {
            Ok(User {
                id: row.get(0)?,
                username: row.get(1)?,
            })
        })?;
        let users: Res<Vec<User>> = user_iter
            .collect::<Result<Vec<User>, rusqlite::Error>>()
            .map_err(|e| Box::new(e) as Box<dyn std::error::Error>);
        users
    }
}
