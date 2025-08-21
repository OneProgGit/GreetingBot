use sqlx::{Row, SqlitePool, sqlite::SqlitePoolOptions};

use crate::{
    db_mod::database::Database, models_mod::user_model::UserModel, tools_mod::config_tools::CONFIG,
    traits_mod::create_traits::CreateAsync, types_mod::result_types::Res,
};

use std::sync::Arc;

#[derive(Clone, Debug)]
pub struct SqliteDatabase {
    pool: Arc<SqlitePool>,
}

#[async_trait::async_trait]
impl CreateAsync for SqliteDatabase {
    #[tracing::instrument]
    async fn new() -> Res<Arc<Self>> {
        let pool = SqlitePoolOptions::new()
            .max_connections(5)
            .connect(&CONFIG.db_url)
            .await?;
        let db = Self {
            pool: Arc::new(pool),
        };
        Ok(Arc::new(db))
    }
}

#[async_trait::async_trait]
impl Database for SqliteDatabase {
    #[tracing::instrument]
    async fn create_user(&self, user: UserModel) -> Res<()> {
        sqlx::query(
            "INSERT INTO users (id, username)
                VALUES (?1, ?2)
                ON CONFLICT(id) DO UPDATE SET
                    username = excluded.username",
        )
        .bind(user.id)
        .bind(user.username)
        .execute(&*self.pool)
        .await?;
        Ok(())
    }

    #[tracing::instrument]
    async fn get_users(&self) -> Res<Vec<UserModel>> {
        let rows = sqlx::query("SELECT id, username FROM users")
            .fetch_all(&*self.pool)
            .await?;
        let users = rows
            .into_iter()
            .map(|row| UserModel {
                id: row.get::<String, _>("id"),
                username: row.get::<String, _>("username"),
            })
            .collect();
        Ok(users)
    }
}
