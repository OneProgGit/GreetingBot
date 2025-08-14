use crate::models::{types::Res, user::User};

pub trait Database {
    fn new() -> Res<Box<Self>>;
    fn create_user(&self, user: User) -> Res<()>;
    fn get_users(&self) -> Res<Vec<User>>;
}
