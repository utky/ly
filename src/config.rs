use anyhow::Result;
use chrono::FixedOffset;
use std::env;
use std::path::PathBuf;

#[derive(Debug, Clone)]
pub struct Config {
    pub database: PathBuf,
    pub short_break: i64,
    pub long_break: i64,
    pub timezone: FixedOffset,
}

impl Config {
    pub fn from_env() -> Result<Self> {
        let home = env::var("HOME")?;
        Ok(Config {
            database: PathBuf::from(format!("{}/.ly.db", home)),
            short_break: 5,
            long_break: 15,
            timezone: FixedOffset::east(9 * 3600),
        })
    }
}
