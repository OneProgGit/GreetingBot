use crate::{
    infra::database::Database,
    models::{traits::Create, types::Res, user::User},
    tools::config::CONFIG,
};
use rusqlite::{Connection, Result, params};
use std::sync::{Arc, Mutex};

#[derive(Clone)]
pub struct SqliteDb {
    db_conn: Arc<Mutex<Connection>>,
}

impl Create for SqliteDb {
    fn new() -> Res<Arc<Self>> {
        log::info!("Create SQLite connection");
        let conn = Connection::open(CONFIG.db_url.clone())?;
        let db = Self {
            db_conn: Arc::new(Mutex::new(conn)),
        };
        log::info!("Created SQLite connection success");
        Ok(Arc::new(db))
    }
}

impl Database for SqliteDb {
    fn create_user(&self, user: User) -> Res<()> {
        log::info!(
            "Create new user id `{}` username `{}`",
            user.id,
            user.username
        );
        let conn = self.db_conn.lock().expect("Could not lock db_conn");
        conn.execute(
            "INSERT INTO users (id, username)
            VALUES (?1, ?2)
            ON CONFLICT(id) DO UPDATE SET
                username = excluded.username",
            params![user.id, user.username],
        )?;
        log::info!(
            "User id `{}` username `{}` created success",
            user.id,
            user.username
        );
        Ok(())
    }

    fn get_users(&self) -> Res<Vec<User>> {
        log::info!("Get users");
        let conn = self.db_conn.lock().expect("Could not lock db_conn");
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
        log::info!("Got users success");
        users
    }
}
