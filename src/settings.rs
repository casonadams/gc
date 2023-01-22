use colorizer::Colors;
use serde_derive::Deserialize;
use config::{Config, ConfigError, Environment, File};
use std::env;

#[derive(Debug, Deserialize)]
#[allow(unused)]
pub struct Highlight {
    pub regex: String,
    pub name: String,
    pub color: Colors,
}

#[derive(Debug, Deserialize)]
#[allow(unused)]
pub struct Settings {
    pub highlight: Vec<Highlight>,
}

impl Settings {
    pub fn new() -> Result<Self, ConfigError> {
        let _run_mode = env::var("RUN_MODE").unwrap_or_else(|_| "development".into());

        let s = Config::builder()
            .add_source(File::with_name("Settings"))
            .add_source(Environment::with_prefix("app"))
            .build()?;

        s.try_deserialize()
    }
}
