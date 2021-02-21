use anyhow::Result;
use std::env;
use std::path::PathBuf;

#[derive(Debug, Clone)]
pub struct Config {
    pub database: PathBuf,
}

impl Config {
    pub fn from_env() -> Result<Self> {
        let home = env::var("HOME")?;
        Ok(Config {
            database: PathBuf::from(format!("{}/.ly.db", home)),
        })
    }
}
