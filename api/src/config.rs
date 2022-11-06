use figment::{providers::Env, Figment};
use serde::{Deserialize, Serialize};

use crate::error::Result;

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Config {
    pub database_url: String,
    pub host: String,
    pub port: usize,
}

impl Config {
    pub fn init() -> Result<Config> {
        let config = Figment::new().merge(Env::prefixed("")).extract()?;

        Ok(config)
    }
}
