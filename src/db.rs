//! Used for database requests using rusqlite.

use std::sync::{Arc, Mutex};

use rusqlite::{Connection, Result, params};

use crate::config::CONFIG;

/// Defines database api. Works using rusqlite.
#[derive(Clone)]
pub struct Database {
    db_conn: Arc<Mutex<Connection>>,
}

/// Defines user
#[allow(dead_code)]
#[derive(Debug)]
pub struct User {
    pub id: u64,
    pub username: String,
    pub full_name: String,
}

/// Defines chat
#[allow(dead_code)]
#[derive(Debug)]
pub struct Chat {
    pub id: i64,
    pub username: String,
    pub full_name: String,
}

impl Database {
    /// Creates a new database based on url
    pub fn new(url: &str) -> Result<Self> {
        let conn = Connection::open(url)?;
        let db = Self {
            db_conn: Arc::new(Mutex::new(conn)),
        };
        Ok(db)
    }

    /// Create a new database from config
    pub fn from_config() -> Result<Self> {
        Self::new(&CONFIG.db_url)
    }

    /// Inserts a new user into the `users` table
    pub fn create_user(self, id: i64, username: &str, full_name: &str) -> Result<()> {
        let conn = self.db_conn.lock().expect("Could not lock db_conn");
        conn.execute(
            "INSERT INTO users (id, username, full_name)
            VALUES (?1, ?2, ?3)
            ON CONFLICT(id) DO UPDATE SET
                username = excluded.username,
                full_name = excluded.full_name",
            params![id, username, full_name],
        )?;
        Ok(())
    }

    /// Gets all users from the `users` table
    pub fn get_users(self) -> Result<Vec<User>> {
        let conn = self.db_conn.lock().expect("Could not lock db_conn");
        let mut stmt = conn.prepare("SELECT id, username, full_name FROM users")?;
        let user_iter = stmt.query_map([], |row| {
            Ok(User {
                id: row.get(0)?,
                username: row.get(1)?,
                full_name: row.get(2)?,
            })
        })?;

        let users: Result<Vec<User>> = user_iter.collect();
        users
    }
}
