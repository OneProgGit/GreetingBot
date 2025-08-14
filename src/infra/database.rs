use crate::models::{traits::Create, types::Res, user::User};

#[async_trait::async_trait]
pub trait Database: Send + Sync + Create {
    fn create_user(&self, user: User) -> Res<()>;
    fn get_users(&self) -> Res<Vec<User>>;
}
