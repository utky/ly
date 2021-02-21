use super::common::Id;
use anyhow::Result;
use chrono::NaiveDate;
use chrono::{serde::ts_milliseconds, Datelike};
use chrono::{DateTime, FixedOffset, TimeZone, Utc};
use serde::{Deserialize, Serialize};

pub type TodoDate = NaiveDate;

pub fn timestamp_at_start_of_todo_date(date: &TodoDate) -> DateTime<Utc> {
    FixedOffset::east(9 * 3600)
        .ymd(date.year(), date.month(), date.day())
        .and_hms(0, 0, 0)
        .with_timezone(&Utc)
}

/// Task to be done.
#[derive(Serialize, Deserialize, Debug)]
pub struct Todo {
    pub date: TodoDate,
    pub note: String,
    #[serde(with = "ts_milliseconds")]
    pub created_at: DateTime<Utc>,
    #[serde(with = "ts_milliseconds")]
    pub updated_at: DateTime<Utc>,
}

/// Task connected to specific todo
#[derive(Serialize, Deserialize, Debug)]
pub struct TodoTask {
    pub date: TodoDate,
    pub task_id: Id,
    pub lane_id: Id,
    pub priority: Id,
    pub summary: String,
    pub estimate: i64,
    pub actual: i64,
}

pub trait Add {
    fn add_todo(&mut self, date: &TodoDate, note: &str) -> Result<()>;
}

pub trait Fetch {
    fn fetch_by_date(&mut self, date: &TodoDate) -> Result<Option<Todo>>;
    fn fetch_todo_tasks(&mut self, date: &TodoDate) -> Result<Vec<TodoTask>>;
}

pub trait Mod {
    fn add_todo_task(&mut self, date: &TodoDate, task_id: &Id, todo_order: usize) -> Result<()>;
    fn remove_todo_task(&mut self, date: &TodoDate, task_id: &Id) -> Result<()>;
}

pub fn list_todo_tasks<R>(r: &mut R, date: &TodoDate) -> Result<Vec<TodoTask>>
where
    R: Fetch,
{
    r.fetch_todo_tasks(date)
}

pub fn mod_todo<R>(r: &mut R, date: &TodoDate, add_tasks: &[Id], remove_tasks: &[Id]) -> Result<()>
where
    R: Fetch + Mod + Add,
{
    let plan = match r.fetch_by_date(date)? {
        Some(plan) => plan,
        None => {
            // otherwise create
            r.add_todo(date, "")?;
            let msg = format!("could not find inserted plan on date: {:?}", date);
            r.fetch_by_date(date).map(|o| o.expect(&msg))?
        }
    };
    for (i, t) in add_tasks.iter().enumerate() {
        r.add_todo_task(&plan.date, t, i)?;
    }
    for i in remove_tasks {
        r.remove_todo_task(&plan.date, i)?;
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_timestamp_at_start_of_date_in_jst() {
        let jst_date = NaiveDate::from_ymd(2020, 02, 29);
        assert_eq!(
            timestamp_at_start_of_todo_date(&jst_date).timestamp(),
            1582902000
        );
    }
}
