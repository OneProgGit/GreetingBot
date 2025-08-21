use crate::{DB, PLATFORM, models_mod::user_model::UserModel, tools_mod::config_tools::CONFIG};
use string_format::string_format;

#[tracing::instrument]
pub async fn handle_start(user: UserModel) {
    PLATFORM
        .get()
        .expect("Failed to get platform instance")
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
    DB.get()
        .expect("Failed to get DB instance")
        .create_user(user)
        .await
        .expect("Error accessing to database");
}
