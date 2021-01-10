use anyhow::Result;
use uuid::Uuid;
use chrono::{DateTime, Utc};
use super::common::{Id, RepositoryError};
use super::lane;

/// Task to be done.
#[derive(Debug)]
pub struct Task {
  pub id: Id,
  pub uuid: Uuid,
  pub lane_id: Id,
  pub summary: String,
  pub created_at: DateTime<Utc>,
  pub updated_at: DateTime<Utc>
}

pub trait Add {
  fn add_task(&mut self, uuid: &Uuid, lane_id: Id, summary: &str) -> Result<()>;
}

pub trait Fetch {
  fn fetch_task_by_uuid(&mut self, uuid: &Uuid) -> Result<Option<Task>>;
  fn fetch_task_by_id(&mut self, id: Id) -> Result<Option<Task>>;
  fn fetch_all_tasks(&mut self) -> Result<Vec<Task>>;
}

pub fn add_task<R>(r: &mut R, uuid: &Uuid, lane_name: &str, summary: &str) -> Result<()>
  where R: Add + lane::Fetch  {
  if let Some(lane) = r.fetch_lane_by_name(lane_name)? {
    r.add_task(uuid, lane.id, summary)
  }
  else {
    Err(RepositoryError::NotFound.into())
  }
}

pub fn list_all_tasks<R>(r: &mut R) -> Result<Vec<Task>>
  where R: Fetch {
  r.fetch_all_tasks()
}


