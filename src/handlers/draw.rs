use rand::random_range;

use crate::{DB, PLATFORM, models::user::User, tools::config::CONFIG};
use string_format::string_format;

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

    let results_fmt = &string_format!(CONFIG.draw_results_fmt.clone(), choice.username.clone());

    let admin = User {
        id: CONFIG.clone().admin,
        username: "admin".into(),
    };

    PLATFORM
        .clone()
        .send_message(admin, results_fmt)
        .await
        .expect("Send message failed");

    let channel: User = User {
        id: CONFIG.clone().channel,
        username: "channel".into(),
    };

    PLATFORM
        .clone()
        .send_message(channel, results_fmt)
        .await
        .expect("Send message failed");
}
