use anyhow::Result;
use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use chrono::serde::ts_milliseconds;
use super::common::Id;

/// Lane of task list.
#[derive(Debug, Serialize, Deserialize)]
pub struct Lane {
  pub id: Id,
  pub name: String,
  #[serde(with = "ts_milliseconds")]
  pub created_at: DateTime<Utc>,
  #[serde(with = "ts_milliseconds")]
  pub updated_at: DateTime<Utc>
}

pub trait Fetch {
  fn fetch_lane_by_name(&mut self, name: &str) -> Result<Option<Lane>>;
  fn fetch_all_lanes(&mut self) -> Result<Vec<Lane>>;
}

pub fn fetch_all_lanes<R>(r: &mut R) -> Result<Vec<Lane>> where R: Fetch {
  r.fetch_all_lanes()
}
