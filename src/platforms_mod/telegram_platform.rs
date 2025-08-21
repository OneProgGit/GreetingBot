use std::{collections::HashMap, sync::Arc};

use teloxide::{
    Bot,
    payloads::SendMessageSetters,
    prelude::Requester,
    types::{Message, ParseMode},
};
use tokio::sync::Mutex;

use crate::{
    models_mod::user_model::UserModel,
    platforms_mod::platform::{Handler, Platform},
    traits_mod::create_traits::Create,
    types_mod::result_types::Res,
};

#[derive(Debug)]
pub struct TelegramPlatform {
    bot: Arc<Bot>,
    bindings: Mutex<HashMap<String, Handler>>,
}

impl TelegramPlatform {
    #[tracing::instrument]
    async fn handle_message(self: Arc<Self>, user: UserModel, msg: &str) {
        if let Some(handler) = self.bindings.lock().await.get(msg) {
            handler(user).await;
        } else {
            self.send_message(user, "Неизвестная команда")
                .await
                .expect("Failed to send message");
        }
    }
}

impl Create for TelegramPlatform {
    #[tracing::instrument]
    fn new() -> Res<Arc<Self>> {
        let bot = Bot::from_env();
        let tg = Self {
            bot: Arc::new(bot),
            bindings: Mutex::new(HashMap::new()),
        };

        Ok(Arc::new(tg))
    }
}

#[async_trait::async_trait]
impl Platform for TelegramPlatform {
    #[tracing::instrument]
    async fn run(self: Arc<Self>) {
        let tg = Arc::clone(&self);
        let bot = tg.bot.clone();
        teloxide::repl(bot, move |_bot: Arc<Bot>, msg: Message| {
            let tg = tg.clone();
            let user = UserModel {
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

    #[tracing::instrument]
    async fn send_message(self: Arc<Self>, user: UserModel, msg: &str) -> Res<()> {
        self.bot
            .send_message(user.id, msg)
            .parse_mode(ParseMode::Html)
            .await?;
        Ok(())
    }

    #[tracing::instrument]
    async fn bind(self: Arc<Self>, cmd: &str, handler: Handler) {
        let mut bindings = self.bindings.lock().await;

        bindings.insert(cmd.to_string(), handler);
    }
}
