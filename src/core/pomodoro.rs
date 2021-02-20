use super::common::Id;
use anyhow::Result;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Pomodoro {
    pub id: Id,
    pub task_id: Id,
    pub started_at: DateTime<Utc>,
    pub finished_at: DateTime<Utc>,
}

pub trait Complete {
    fn complete_pomodoro(&mut self, task_id: Id, started_at: DateTime<Utc>) -> Result<()>;
}

pub trait Fetch {
    fn fetch_by_task_id(&mut self, task_id: Id) -> Result<Vec<Pomodoro>>;
}

// unused
// pub fn complete_pomodoro<R>(r: &mut R, task_id: Id, started_at: DateTime<Utc>) -> Result<()> where R: Complete {
//   r.complete_pomodoro(task_id, started_at)
// }
//
// pub fn fetch_by_task_id<R>(r: &mut R, task_id: Id) -> Result<Vec<Pomodoro>> where R: Fetch {
//   r.fetch_by_task_id(task_id)
// }
