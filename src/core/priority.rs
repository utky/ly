use anyhow::Result;
use chrono::{DateTime, Utc};
use super::common::{Id};

/// Task to be done.
#[derive(Debug)]
pub struct Priority {
  pub id: Id,
  pub name: String,
  pub created_at: DateTime<Utc>,
  pub updated_at: DateTime<Utc>
}

pub trait Fetch {
  fn fetch_priority_by_name(&mut self, name: &str) -> Result<Priority>;
  fn fetch_all_priority(&mut self) -> Result<Vec<Priority>>;
}

pub fn fetch_all_priority<R>(r: &mut R) -> Result<Vec<Priority>> where R: Fetch {
  r.fetch_all_priority()
}