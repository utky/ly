use super::common::{Id, RepositoryError};
use super::lane;
use super::priority;
use anyhow::Result;
use chrono::serde::ts_milliseconds;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// Task to be done.
#[derive(Serialize, Deserialize, Debug)]
pub struct Task {
    pub id: Id,
    pub lane_id: Id,
    pub priority: Id,
    pub summary: String,
    pub estimate: i64,
    #[serde(with = "ts_milliseconds")]
    pub created_at: DateTime<Utc>,
    #[serde(with = "ts_milliseconds")]
    pub updated_at: DateTime<Utc>,
}

pub trait Add {
    fn add_task(&mut self, lane_id: Id, priority: Id, summary: &str, estimate: i64) -> Result<()>;
}

pub trait Fetch {
    fn fetch_task_by_id(&mut self, id: Id) -> Result<Option<Task>>;
    fn fetch_all_tasks(&mut self, lane_name: &str) -> Result<Vec<Task>>;
}

pub trait Mod {
    fn mod_task(
        &mut self,
        id: Id,
        lane_id: Option<&Id>,
        priority: Option<&Id>,
        summary: Option<&str>,
        estimate: Option<i64>,
    ) -> Result<()>;
}

pub fn add_task<R>(
    r: &mut R,
    lane_name: &str,
    priority_name: &str,
    summary: &str,
    estimate: i64,
) -> Result<()>
where
    R: Add + lane::Fetch + priority::Fetch,
{
    if let Some(lane) = r.fetch_lane_by_name(lane_name)? {
        let prio = r.fetch_priority_by_name(priority_name)?;
        r.add_task(lane.id, prio.id, summary, estimate)
    } else {
        Err(RepositoryError::NotFound.into())
    }
}

pub fn list_all_tasks<R>(r: &mut R, lane_name: &str) -> Result<Vec<Task>>
where
    R: Fetch,
{
    r.fetch_all_tasks(lane_name)
}

pub fn mod_task<R>(
    r: &mut R,
    id: Id,
    lane_name: Option<&str>,
    priority_name: Option<&str>,
    summary: Option<&str>,
    estimate: Option<i64>,
) -> Result<()>
where
    R: Mod + lane::Fetch + priority::Fetch,
{
    let lane = lane_name
        .and_then(|lane_name| r.fetch_lane_by_name(lane_name).ok().unwrap())
        .map(|l| l.id);
    let prio = priority_name
        .and_then(|priority_name| r.fetch_priority_by_name(priority_name).ok())
        .map(|p| p.id);
    r.mod_task(id, lane.as_ref(), prio.as_ref(), summary, estimate)
}
