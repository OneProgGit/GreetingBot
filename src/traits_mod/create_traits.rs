use std::sync::Arc;

use crate::types_mod::result_types::Res;

pub trait Create {
    fn new() -> Res<Arc<Self>>
    where
        Self: Sized;
}

#[async_trait::async_trait]
pub trait CreateAsync {
    async fn new() -> Res<Arc<Self>>
    where
        Self: Sized;
}
