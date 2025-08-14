use std::{collections::HashMap, pin::Pin, sync::Arc};

use teloxide::{Bot, prelude::Requester, types::Message};

use crate::{
    models::{types::Res, user::User},
    platforms::platform::Platform,
};

type Handler = Box<dyn Fn(User) -> Pin<Box<dyn Future<Output = ()> + Send + Sync>> + Send + Sync>;

pub struct Telegram {
    bot: Arc<Bot>,
    bindings: HashMap<String, Handler>,
}

impl Telegram {
    async fn handle_message(&self, user: User, msg: &str) {
        if let Some(handler) = self.bindings.get(msg) {
            handler(user).await;
        } else {
            self.send_message(user, "Неизвестная команда").await;
        }
    }
}

impl Platform for Telegram {
    async fn new() -> Self {
        let bot = Bot::from_env();
        let tg = Self {
            bot: Arc::new(bot),
            bindings: HashMap::new(),
        };
        tokio::spawn(async {
            teloxide::repl(bot, move |bot: Bot, msg: Message| {
                let bot = bot.clone();
                let user = User {
                    id: msg.chat.id.0.to_string(),
                    username: msg
                        .chat
                        .username()
                        .unwrap_or(&format!(
                            "user {}",
                            msg.chat.first_name().unwrap_or("user").to_string()
                        ))
                        .to_string(),
                };

                async move {
                    tg.handle_message(user, msg.text().unwrap_or("")).await;
                    Ok(())
                }
            })
            .await;
        });

        tg
    }

    async fn send_message(&self, user: User, msg: &str) -> Res<()> {
        self.bot.send_message(user.id, msg).await?;
        Ok(())
    }

    async fn bind<F, Fut>(&mut self, cmd: &str, handler: F)
    where
        F: Fn(User) -> Fut + Send + Sync + 'static,
        Fut: Future<Output = ()> + Send + Sync + 'static,
    {
        self.bindings
            .insert(cmd.to_string(), Box::new(move |usr| Box::pin(handler(usr))));
    }
}
