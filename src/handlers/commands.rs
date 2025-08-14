use chrono::Utc;

use crate::{models::user::User, tools::config::CONFIG};

async fn process_user(user: User, weather: String) {
    log::info!(
        "Handling user @{}...",
        user.username,
    );
    let response = ai::process_ollama(weather.clone())
        .await
        .unwrap_or(CONFIG.ai_msg_off.clone());
    log::info!(
        "Ai's response for user @{} is `{response}`",
        user.username,
    );
    let now = Utc::now();

    
}

async fn daily_message() {
    log::info!("Daily message time!");

    log::info!("Getting weather...");

    let weather = get_weather().await.unwrap_or_else(|err| {
        log::error!("Could not get weather: `{err}`");
        "Пасмурно".into()
    });

    log::info!("Handling all users...");

    let users = DB.clone().get_users().expect("Error while getting users");

    for user in users {
        tokio::spawn(process_user(user, weather.clone()));
    }

    let channel = Chat {
        id: CONFIG.channel,
        username: "oneprogofficial".into(),
        full_name: "OneProg".into(),
    };

    tokio::spawn(process_chat(channel, weather.clone()));
}

async fn draw() {
    log::info!("Draw time!");
    log::info!("Getting a random user...");

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

    log::info!(
        "The winner is @{} (full name {})!",
        choice.username,
        choice.full_name
    );

    BOT.send_message(
        UserId(choice.id),
        string_format!(CONFIG.draw_win_fmt.clone(), choice.username.clone()),
    )
    .parse_mode(ParseMode::Html)
    .await
    .expect("Send message failed");

    BOT.send_message(
        UserId(CONFIG.admin),
        string_format!(
            CONFIG.draw_admin_fmt.clone(),
            choice.username.clone(),
            choice.full_name.clone()
        ),
    )
    .parse_mode(ParseMode::Html)
    .await
    .expect("Send message failed");

    log::info!(
        "Message sent success to user @{} (full name {})",
        choice.username,
        choice.full_name
    );
}


pub fn bind_all_commands() {

}