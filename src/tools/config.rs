use std::{env, path::Path, sync::LazyLock};

use config::{Config, ConfigError, File};
use serde::Deserialize;

#[derive(Clone, Deserialize, Debug)]
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
    pub admin: String,
    pub draw_admin_fmt: String,
    pub channel: String,
}

pub static CONFIG: LazyLock<Configuration> = LazyLock::new(|| {
    load_config(&env::var("CONFIG_PATH").expect("CONFIG_PATH must be set!"))
        .expect("Failed to load config")
});

pub fn load_config(path: &str) -> Result<Configuration, ConfigError> {
    log::info!("Load config from `{}`", path);
    let cfg = Config::builder()
        .add_source(File::from(Path::new(path)))
        .build()?
        .try_deserialize();
    if let Err(e) = cfg {
        log::error!("Failed to load config from `{}`: {}", path, e);
        Err(e)
    } else {
        log::info!("Got config `{:?}`", cfg);
        cfg
    }
}

#[cfg(test)]
mod config_test {
    use crate::tools::config::load_config;

    #[test]
    fn test_load_config() {
        let path = "test.toml";
        let res = load_config(path).expect("Failed to load config");
        assert_eq!(res.start_fmt, "Hello world!");
    }
}
