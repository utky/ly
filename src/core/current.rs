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
  pub started_at: DateTime<Utc>,
  pub duration_min: i64
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CurrentTask {
  pub id: Id,
  pub task: task::Task,
  #[serde(with = "ts_milliseconds")]
  pub started_at: DateTime<Utc>,
  pub duration_min: i64
}

pub trait Lifecycle {
  fn start(&mut self, task_id: Id, duration_min: i64) -> Result<Current>;
  fn complete(&mut self) -> Result<()>;
}

pub trait Get {
  fn get(&mut self) -> Result<Option<Current>>;
}

pub fn start<R>(r: &mut R, task_id: Id, duration_min: i64) -> Result<Current>
  where R: Lifecycle + task::Fetch {
  let t = r.fetch_task_by_id(task_id)?.ok_or(RepositoryError::NotFound)?;
  let c = r.start(t.id, duration_min)?;
  Ok(c)
}

pub fn complete<R>(r: &mut R, current: &Current) -> Result<()>
  where R: Lifecycle + task::Fetch + pomodoro::Complete {
  let t = r.fetch_task_by_id(current.task_id)?.ok_or(RepositoryError::NotFound)?;
  r.complete_pomodoro(t.id, current.started_at)?;
  r.complete()
}

pub fn get_current_task<R>(r: &mut R) -> Result<Option<CurrentTask>>
  where R: Get + task::Fetch {
  let c = r.get()?;
  match c {
    Some(c) => {
      let t = r.fetch_task_by_id(c.task_id)?.ok_or(RepositoryError::NotFound)?;
      Ok(Some(CurrentTask {id: c.id, task: t, started_at: c.started_at, duration_min: c.duration_min}))
    },
    None => Ok(None)
  }
}
