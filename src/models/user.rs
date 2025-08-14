use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct User {
    pub id: String,
    pub username: String,
}
