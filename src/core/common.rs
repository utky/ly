use std::error::Error;
use std::fmt::{Display, Formatter, Result};
/// Autoincrement integer
pub type Id = i64;

#[derive(Debug, Clone)]
pub enum RepositoryError {
    NotFound,
}

impl Display for RepositoryError {
    fn fmt(&self, f: &mut Formatter) -> Result {
        match self {
            RepositoryError::NotFound => write!(f, "Entity not found"),
        }
    }
}

impl Error for RepositoryError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        None
    }
}
