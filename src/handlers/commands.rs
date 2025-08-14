use crate::{PLATFORM, handlers::start::handle_start, models::user::User};

#[tracing::instrument]
pub async fn bind_all_commands() {
    PLATFORM
        .clone()
        .bind("/start", |user: User| Box::pin(handle_start(user)))
        .await;
}
