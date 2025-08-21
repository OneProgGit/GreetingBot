use std::{fmt::Debug, pin::Pin, sync::Arc};

use crate::{
    models_mod::user_model::UserModel, traits_mod::create_traits::Create,
    types_mod::result_types::Res,
};

pub type Handler = fn(UserModel) -> Pin<Box<dyn Future<Output = ()> + Send>>;

#[async_trait::async_trait]
pub trait Platform: Send + Sync + Create + Debug {
    async fn run(self: Arc<Self>);
    async fn send_message(self: Arc<Self>, user: UserModel, msg: &str) -> Res<()>;
    async fn bind(self: Arc<Self>, cmd: &str, handler: Handler);
}
