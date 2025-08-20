use crate::models::{traits::CreateAsync, types::Res, user::User};

#[async_trait::async_trait]
pub trait Database: Send + Sync + CreateAsync {
    async fn create_user(&self, user: User) -> Res<()>;
    async fn get_users(&self) -> Res<Vec<User>>;
}
