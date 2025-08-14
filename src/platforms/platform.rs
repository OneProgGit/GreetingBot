use crate::models::{
    types::{Res, ThreadSafeRes},
    user::User,
};

pub trait Platform {
    async fn new() -> Self;
    async fn send_message(&self, user: User, msg: &str) -> Res<()>;
    async fn bind<F, Fut>(&mut self, cmd: &str, handler: F)
    where
        F: Fn(User) -> Fut + Send + Sync + 'static,
        Fut: Future<Output = ()> + Send + Sync + 'static;
}
