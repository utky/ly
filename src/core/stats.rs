use super::common::Id;
use anyhow::Result;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use chrono::serde::ts_milliseconds;

#[derive(Debug, Serialize, Deserialize)]
pub struct DailyStats {
    pub id: Id,
    pub task_id: Id,
    #[serde(with = "ts_milliseconds")]
    pub started_at: DateTime<Utc>,
    #[serde(with = "ts_milliseconds")]
    pub finished_at: DateTime<Utc>,
}

pub trait Fetch {
    fn fetch_pomodoro_daily_stats(&mut self, start: DateTime<Utc>, end: DateTime<Utc>) -> Result<Vec<Pomodoro>>;
}
