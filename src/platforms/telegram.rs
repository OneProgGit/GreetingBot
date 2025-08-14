use std::{collections::HashMap, sync::Arc};

use teloxide::{
    payloads::SendMessageSetters, prelude::Requester, types::{Message, ParseMode}, Bot
};
use tokio::sync::Mutex;

use crate::{
    models::{traits::Create, types::Res, user::User},
    platforms::platform::{Handler, Platform},
};

pub struct Telegram {
    bot: Arc<Bot>,
    bindings: Mutex<HashMap<String, Handler>>,
}

impl Telegram {
    async fn handle_message(self: Arc<Self>, user: User, msg: &str) {
        log::info!("Handle message");
        if let Some(handler) = self.bindings.lock().await.get(msg) {
            handler(user).await;
        } else {
            self.send_message(user, "Неизвестная команда")
                .await
                .expect("Failed to send message");
        }
    }
}

impl Create for Telegram {
    fn new() -> Res<Arc<Self>> {
        log::info!("Create Telegram instance");
        let bot = Bot::from_env();
        let tg = Self {
            bot: Arc::new(bot),
            bindings: Mutex::new(HashMap::new()),
        };

        Ok(Arc::new(tg))
    }
}

#[async_trait::async_trait]
impl Platform for Telegram {
    async fn run(self: Arc<Self>) {
        log::info!("Run Telegram instance");
        let tg = Arc::clone(&self);
        let bot = tg.bot.clone();
        log::info!("Start replying messages");
        teloxide::repl(bot, move |_bot: Arc<Bot>, msg: Message| {
            log::info!("Reply to message `{}`", msg.text().unwrap_or("No text"));
            let tg = tg.clone();
            let user = User {
                id: msg.chat.id.0.to_string(),
                username: msg
                    .chat
                    .username()
                    .unwrap_or(&format!("user {}", msg.chat.first_name().unwrap_or("user")))
                    .to_string(),
            };

            async move {
                tg.handle_message(user, msg.text().unwrap_or("")).await;
                Ok(())
            }
        })
        .await;
    }

    async fn send_message(self: Arc<Self>, user: User, msg: &str) -> Res<()> {
        log::info!("Send message `{msg}` to user @{}", user.username);
        self.bot
            .send_message(user.id, msg)
            .parse_mode(ParseMode::Html)
            .await?;
        log::info!("Sent message success for user @{}", user.username);
        Ok(())
    }

    async fn bind(self: Arc<Self>, cmd: &str, handler: Handler) {
        log::info!("Bind command `{cmd}`");
        let mut bindings = self.bindings.lock().await;

        bindings.insert(cmd.to_string(), handler);
    }
}
