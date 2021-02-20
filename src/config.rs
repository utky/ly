use anyhow::Result;
use std::path::{PathBuf, Path};
use std::env;

#[derive(Debug, Clone)]
pub struct Config {
  pub database: PathBuf
}

impl Config {
  pub fn from_env() -> Result<Self> {
    let home = env::var("HOME")?;
    Ok(Config { database: PathBuf::from(format!("{}/.ly.db", home)) })
  }
  pub fn from_user(path: &Path) -> Result<Self> {
    Ok(Config { database: path.to_path_buf()})
  }
}
