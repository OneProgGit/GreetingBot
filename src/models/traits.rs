use std::sync::Arc;

use crate::models::types::Res;

pub trait Create {
    fn new() -> Res<Arc<Self>>
    where
        Self: Sized;
}

pub trait CreateAsync {
    async fn new() -> Res<Arc<Self>>
    where
        Self: Sized;
}
