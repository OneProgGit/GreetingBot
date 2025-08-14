use crate::models::{traits::Create, types::Res};

#[async_trait::async_trait]
pub trait AiProvider: Send + Sync + Create {
    async fn process(&self, weather: String) -> Res<String>;
}