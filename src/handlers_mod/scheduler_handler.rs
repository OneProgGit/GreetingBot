use chrono::Utc;
use cron_tab::AsyncCron;

use crate::{
    handlers_mod::{daily_messages_handler::handle_daily_message, draw_handler::handle_draw},
    tools_mod::config_tools::CONFIG,
};

#[tracing::instrument]
pub async fn schedule_all_tasks() {
    let mut cron = AsyncCron::new(Utc);

    cron.add_fn(&CONFIG.greeting_date_cron, || async {
        handle_daily_message().await;
    })
    .await
    .expect("Failed to daily");

    cron.add_fn(&CONFIG.draw_date_cron, || async {
        handle_draw().await;
    })
    .await
    .expect("Failed to draw");

    cron.start().await;
}
