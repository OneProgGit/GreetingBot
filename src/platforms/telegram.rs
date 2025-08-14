use std::{
    collections::HashMap,
    pin::Pin, sync::Arc,
};

use teloxide::{Bot, prelude::Requester, types::Message};
use tokio::sync::Mutex;

use crate::{
    models::{traits::Create, types::Res, user::User},
    platforms::platform::Platform,
};

type Handler = Box<dyn Fn(User) -> Pin<Box<dyn Future<Output = ()> + Send + Sync>> + Send + Sync>;

pub struct Telegram {
    bot: Arc<Bot>,
    bindings: Mutex<HashMap<String, Handler>>,
}

impl Telegram {
    async fn handle_message(self: Arc<Self>, user: User, msg: &str) {
        if let Some(handler) = self.bindings.lock().await.get(msg) {
            handler(user).await;
        } else {
            self.send_message(user, "Неизвестная команда").await;
        }
    }
}

impl Create for Telegram {
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
impl Platform for Telegram {
    async fn run(self: Arc<Self>) {
        let tg = Arc::clone(&self);
        let bot = tg.bot.clone();
        tokio::spawn(async move {
            teloxide::repl(bot, move |_: Bot, msg: Message| {
                let tg = tg.clone();
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
    }

    async fn send_message(self: Arc<Self>, user: User, msg: &str) -> Res<()> {
        self.bot.send_message(user.id, msg).await?;
        Ok(())
    }

    async fn bind(self: Arc<Self>, cmd: &str, handler: Box<dyn Fn(User) -> Pin<Box<dyn Future<Output = ()> + Send + Sync>> + Send + Sync>){
        let mut bindings = self.bindings.lock().await;

        bindings.insert(cmd.to_string(), Box::new(move |usr| Box::pin(handler(usr))));
    }
}
