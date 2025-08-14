use crate::{models::user::User, tools::config::CONFIG, DB, PLATFORM};
use string_format::string_format;

pub async fn handle_start(user: User) {
    log::info!("Handling a message from @{}...", user.username.clone());
    PLATFORM.clone().send_message(
        user.clone(),
        &string_format!(
            CONFIG.start_fmt.clone(),
            user.username.clone(),
            user.id.clone()
        ),
    )
    .await.expect("Failed to send message");
    DB.create_user(user)
        .expect("Error accessing to database");
}