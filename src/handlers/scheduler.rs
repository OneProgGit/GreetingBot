use chrono::Utc;
use cron_tab::AsyncCron;

use crate::{handlers::{daily_messages::daily_message, draw::draw}, tools::config::CONFIG};

pub async fn schedule_all_tasks() {
    let mut cron = AsyncCron::new(Utc);

    log::info!("Creating daily message task...");

    cron
        .add_fn(&CONFIG.greeting_date_cron, || async {
            log::info!("Daily msg");
            daily_message().await;
        })
        .await
        .expect("Failed to daily");

    log::info!("Creating draw task...");

    cron
        .add_fn(&CONFIG.draw_date_cron, || async {
            log::info!("Draw");
            draw().await;
        })
        .await
        .expect("Failed to draw");

    log::info!("Starting tasks...");

    cron.start().await;
}