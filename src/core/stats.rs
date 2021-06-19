use super::common::Id;
use anyhow::Result;
use chrono::serde::ts_milliseconds;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct SummaryRange {
    #[serde(with = "ts_milliseconds")]
    pub start: DateTime<Utc>,
    #[serde(with = "ts_milliseconds")]
    pub end: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DailySummary {
    #[serde(with = "ts_milliseconds")]
    pub date: DateTime<Utc>,
    pub task_id: Id,
    pub pomodoro_count: i64,
    pub interruption_count: i64,
}

pub trait Fetch {
    fn fetch_daily_summary(&mut self, range: &SummaryRange) -> Result<Vec<DailySummary>>;
}
