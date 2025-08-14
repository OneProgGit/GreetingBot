use crate::{DB, PLATFORM, models::user::User, tools::config::CONFIG};
use string_format::string_format;

pub async fn handle_start(user: User) {
    log::info!("Handle a message from @{}", user.username.clone());
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
    log::info!("Create user for @{}", user.username.clone());
    DB.create_user(user).expect("Error accessing to database");
}
