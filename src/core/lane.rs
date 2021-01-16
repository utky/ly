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
  fn fetch_all_lanes(&mut self) -> Result<Vec<Lane>>;
}

pub fn fetch_all_lanes<R>(r: &mut R) -> Result<Vec<Lane>> where R: Fetch {
  r.fetch_all_lanes()
}
