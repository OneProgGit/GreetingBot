use crate::{
    PLATFORM, handlers_mod::start_handler::handle_start, models_mod::user_model::UserModel,
};

#[tracing::instrument]
pub async fn bind_all_commands() {
    let platform = PLATFORM
        .get()
        .expect("Failed to get platform instance")
        .clone();

    platform
        .clone()
        .bind("/start", |user: UserModel| Box::pin(handle_start(user)))
        .await;
}
