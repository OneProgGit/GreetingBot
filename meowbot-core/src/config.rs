use std::path::Path;

use config::{Config, ConfigError, File};
use serde::Deserialize;

#[derive(Clone, Deserialize, Debug)]
pub struct Configuration {
    pub weather_fmt: String,
    pub ai_model: String,
    pub ai_prompt: String,
    pub ai_msg_off: String,
    pub greeting_date_cron: String,
    pub greeting_fmt: String,
    pub start_fmt: String,
    pub draw_date_cron: String,
    pub draw_win_fmt: String,
    pub admin: String,
    pub draw_results_fmt: String,
    pub channel: String,
    pub skip_channel: bool,
    pub changed_city_fmt: String,
    pub changed_city_failed_fmt: String,
    pub changed_areas_of_interest_fmt: String,
}

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
        let path = "test.toml";
        let res = load_config(path).expect("Failed to load config");
        assert_eq!(res.start_fmt, "Hello world!");
    }
}
