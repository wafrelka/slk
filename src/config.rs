use anyhow::Context as _;
use std::collections::HashMap;

#[derive(Debug, serde::Deserialize)]
pub struct Config {
    pub context: HashMap<String, Context>,
}

#[derive(Debug, serde::Deserialize)]
pub struct Context {
    pub webhook: String,
}

pub fn read_config() -> anyhow::Result<Config> {
    let config_file_path = xdg::BaseDirectories::with_prefix("slk")
        .map(|dirs| dirs.get_config_file("config.toml"))
        .context("could not locate xdg app data directory")?;
    let config_text = std::fs::read_to_string(&config_file_path)
        .context(format!("could not read config file from {:?}", &config_file_path))?;
    toml::from_str(&config_text).context("could not parse config file")
}
