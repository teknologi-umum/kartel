use std::sync::LazyLock;

use configrs::config as configrs;
use serde::Deserialize;

pub fn config() -> &'static Config {
    &CONFIG
}

static CONFIG: LazyLock<Config> = LazyLock::new(|| {
    configrs::Config::new()
        .with_env_prefix(ENV_PREFIX)
        .build::<Config>()
        .expect("failed initializing config")
});

const ENV_PREFIX: &str = "KARTEL_";

#[derive(Debug, Clone, Deserialize)]
pub struct Config {
    #[serde(alias = "KARTEL_BOT_TOKEN", default)]
    pub bot_token: String,
}
