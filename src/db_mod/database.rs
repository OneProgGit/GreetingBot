use std::fmt::Debug;

use crate::{
    models_mod::user_model::UserModel, traits_mod::create_traits::CreateAsync,
    types_mod::result_types::Res,
};

#[async_trait::async_trait]
pub trait Database: Send + Sync + CreateAsync + Debug {
    async fn create_user(&self, user: UserModel) -> Res<()>;
    async fn get_users(&self) -> Res<Vec<UserModel>>;
}
