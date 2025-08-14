use rand::random_range;

use crate::{DB, PLATFORM, models::user::User, tools::config::CONFIG};
use string_format::string_format;

#[tracing::instrument]
pub async fn draw() {
    let users = DB.clone().get_users().expect("Error while getting users");
    let mut ind = random_range(0..users.len());
    let mut choice = &users[ind];

    let mut it = 0;

    while choice.id == CONFIG.admin {
        ind = random_range(0..users.len());
        choice = &users[ind];
        it += 1;
        if it == 10000000 {
            panic!("Unluckly, can't choose the winner");
        }
    }

    PLATFORM
        .clone()
        .send_message(
            choice.clone(),
            &string_format!(CONFIG.draw_win_fmt.clone(), choice.username.clone()),
        )
        .await
        .expect("Send message failed");

    let admin = User {
        id: CONFIG.clone().admin,
        username: "admin".into(),
    };

    PLATFORM
        .clone()
        .send_message(
            admin,
            &string_format!(CONFIG.draw_admin_fmt.clone(), choice.username.clone()),
        )
        .await
        .expect("Send message failed");
}
