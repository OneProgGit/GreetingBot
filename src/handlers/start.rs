use crate::{DB, PLATFORM, models::user::User, tools::config::CONFIG};
use string_format::string_format;

#[tracing::instrument]
pub async fn handle_start(user: User) {
    PLATFORM
        .clone()
        .send_message(
            user.clone(),
            &string_format!(
                CONFIG.start_fmt.clone(),
                user.username.clone(),
                user.id.clone()
            ),
        )
        .await
        .expect("Failed to send message");
    DB.create_user(user).expect("Error accessing to database");
}
