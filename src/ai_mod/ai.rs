use std::fmt::Debug;

use crate::{traits_mod::create_traits::Create, types_mod::result_types::Res};

#[async_trait::async_trait]
pub trait AiModule: Send + Sync + Create + Debug {
    async fn process(&self, weather: String) -> Res<String>;
}
