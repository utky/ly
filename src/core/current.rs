use serde::{Deserialize, Serialize};
use anyhow::Result;
use chrono::{DateTime, Utc};
use chrono::serde::ts_milliseconds;
use super::common::{Id, RepositoryError};
use super::task;
use super::pomodoro;

#[derive(Debug, Serialize, Deserialize)]
pub struct Current {
  pub id: Id,
  pub task_id: Id,
  #[serde(with = "ts_milliseconds")]
  pub started_at: DateTime<Utc>
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CurrentTask {
  pub id: Id,
  pub task: task::Task,
  #[serde(with = "ts_milliseconds")]
  pub started_at: DateTime<Utc>
}

pub trait Lifecycle {
  fn start(&mut self, task_id: Id) -> Result<Current>;
  fn complete(&mut self) -> Result<()>;
}

pub trait Get {
  fn get(&mut self) -> Result<Current>;
}

pub fn start<R>(r: &mut R, task_id: Id) -> Result<Current>
  where R: Lifecycle + task::Fetch {
  let t = r.fetch_task_by_id(task_id)?.ok_or(RepositoryError::NotFound)?;
  let c = r.start(t.id)?;
  Ok(c)
}

pub fn complete<R>(r: &mut R, current: &Current) -> Result<()>
  where R: Lifecycle + task::Fetch + pomodoro::Complete {
  let t = r.fetch_task_by_id(current.task_id)?.ok_or(RepositoryError::NotFound)?;
  r.complete_pomodoro(t.id, current.started_at)?;
  r.complete()
}

pub fn get_current_task<R>(r: &mut R) -> Result<CurrentTask>
  where R: Get + task::Fetch {
  let c = r.get()?;
  let t = r.fetch_task_by_id(c.task_id)?.ok_or(RepositoryError::NotFound)?;
  Ok(CurrentTask {id: c.id, task: t, started_at: c.started_at})
}
