use chrono::Utc;
use cron_tab::AsyncCron;

use crate::{
    handlers_mod::{daily_messages::daily_message, draw::draw},
    tools_mod::config_tools::CONFIG,
};

#[tracing::instrument]
pub async fn schedule_all_tasks() {
    let mut cron = AsyncCron::new(Utc);

    cron.add_fn(&CONFIG.greeting_date_cron, || async {
        daily_message().await;
    })
    .await
    .expect("Failed to daily");

    cron.add_fn(&CONFIG.draw_date_cron, || async {
        draw().await;
    })
    .await
    .expect("Failed to draw");

    cron.start().await;
}
