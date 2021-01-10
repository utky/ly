use anyhow::Result;
use chrono::{DateTime, Utc};
use super::common::Id;

/// Lane of task list.
#[derive(Debug)]
pub struct Lane {
  pub id: Id,
  pub name: String,
  pub created_at: DateTime<Utc>,
  pub updated_at: DateTime<Utc>
}

pub trait Fetch {
  fn fetch_lane_by_name(&mut self, name: &str) -> Result<Option<Lane>>;
}
