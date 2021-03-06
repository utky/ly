use super::common::Id;
use anyhow::Result;
use chrono::serde::ts_milliseconds;
use chrono::{DateTime, Datelike, TimeZone, Utc};
use serde::{Deserialize, Serialize};

//pub type TodoDate = NaiveDate;
pub type TodoDate = DateTime<Utc>;

/// Returns timestamp of start of day in specified timezone from specified time point in (maybe) other timezone.
pub fn start_of_day_in_tz<FromZone: TimeZone, ToZone: TimeZone>(
    ts: DateTime<FromZone>,
    timezone: &ToZone,
) -> DateTime<ToZone> {
    let now_tz = ts.with_timezone(timezone);
    timezone
        .ymd(now_tz.year(), now_tz.month(), now_tz.day())
        .and_hms(0, 0, 0)
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
    use chrono::{FixedOffset, TimeZone, Utc};

    #[test]
    fn test_start_of_day_in_tz() {
        let jst = FixedOffset::east(9 * 3600);
        let timestamp_utc = Utc.ymd(2021, 3, 6).and_hms(23, 10, 33);
        let start_of_day = super::start_of_day_in_tz(timestamp_utc, &jst);
        assert_eq!(
            start_of_day.format("%Y-%m-%d %H:%M:%S").to_string(),
            "2021-03-07 00:00:00"
        );
    }
}
