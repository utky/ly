use anyhow::Result;
use std::env;
use std::path::PathBuf;

#[derive(Debug, Clone)]
pub struct Config {
    pub database: PathBuf,
    pub short_break: i64,
    pub long_break: i64,
}

impl Config {
    pub fn from_env() -> Result<Self> {
        let home = env::var("HOME")?;
        Ok(Config {
            database: PathBuf::from(format!("{}/.ly.db", home)),
            short_break: 5,
            long_break: 15,
        })
    }
}
