//! Used for configuration stuff. Don't change config in runtime.
//! # Examples
//! Let's define `foo` in our config file equals to 5.
//! ```
//! let bar = config::CONFIG.foo;
//! assert_eq!(bar, 5);
//! ```

use std::{env, path::Path, sync::LazyLock};

use config::{Config, ConfigError, File};
use serde::Deserialize;

/// Defines all configuration stuff
#[derive(Clone, Deserialize)]
pub struct Configuration {
    pub weather_url: String,
    pub weather_fmt: String,
    pub ai_model: String,
    pub ai_prompt: String,
    pub ai_msg_off: String,
    pub greeting_date_cron: String,
    pub greeting_fmt: String,
    pub start_fmt: String,
    pub db_url: String,
    pub draw_date_cron: String,
    pub draw_win_fmt: String,
    pub admin: u64,
    pub draw_admin_fmt: String,
    pub channel: i64,
}

pub static CONFIG: LazyLock<Configuration> = LazyLock::new(|| {
    load_config(&env::var("CONFIG_PATH").expect("CONFIG_PATH must be set!"))
        .expect("Failed to load config")
});

/// Loads config from the path and deserializes it to Configuration struct
pub fn load_config(path: &str) -> Result<Configuration, ConfigError> {
    Config::builder()
        .add_source(File::from(Path::new(path)))
        .build()?
        .try_deserialize()
}

#[cfg(test)]
mod config_test {
    use crate::config::load_config;

    #[test]
    fn test_load_config() {
        let path = "test.toml"; // Be sure this file exists to pass test
        let res = load_config(path).expect("Failed to load config");
        assert_eq!(res.start_fmt, "Hello world!");
    }
}
