use crate::{handlers::start::handle_start, models::user::User, PLATFORM};

pub fn bind_all_commands() {
    PLATFORM.clone().bind(
        "/start", 
        |user: User| Box::pin(handle_start(user))
    );
}