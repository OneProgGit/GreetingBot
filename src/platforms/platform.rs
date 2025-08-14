use std::{pin::Pin, sync::Arc};

use crate::models::{traits::Create, types::Res, user::User};

#[async_trait::async_trait]
pub trait Platform: Send + Sync + Create {
    async fn run(self: Arc<Self>);
    async fn send_message(self: Arc<Self>, user: User, msg: &str) -> Res<()>;
    async fn bind(self: Arc<Self>, cmd: &str, handler: Box<dyn Fn(User) -> Pin<Box<dyn Future<Output = ()> + Send + Sync>> + Send + Sync>);
}
