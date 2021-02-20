use super::common::Id;
use super::task;
use anyhow::Result;
use chrono::serde::ts_milliseconds;
use chrono::NaiveDate;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// Task to be done.
#[derive(Serialize, Deserialize, Debug)]
pub struct Plan {
    pub date: NaiveDate,
    pub note: String,
    #[serde(with = "ts_milliseconds")]
    pub created_at: DateTime<Utc>,
    #[serde(with = "ts_milliseconds")]
    pub updated_at: DateTime<Utc>,
}

pub trait Add {
    fn add_plan(&mut self, date: &NaiveDate, note: &str) -> Result<()>;
}

pub trait Fetch {
    fn fetch_by_date(&mut self, date: &NaiveDate) -> Result<Option<Plan>>;
    fn fetch_planned_tasks(&mut self, date: &NaiveDate) -> Result<Vec<task::Task>>;
}

pub trait Mod {
    fn add_planned_task(&mut self, date: &NaiveDate, task_id: &Id) -> Result<()>;
    fn remove_planned_task(&mut self, date: &NaiveDate, task_id: &Id) -> Result<()>;
}

pub fn list_planned_tasks<R>(r: &mut R, date: &NaiveDate) -> Result<Vec<task::Task>>
where
    R: Fetch,
{
    r.fetch_planned_tasks(date)
}

pub fn mod_plan<R>(r: &mut R, date: &NaiveDate, add_tasks: &[Id], remove_tasks: &[Id]) -> Result<()>
where
    R: Fetch + Mod + Add,
{
    let plan = match r.fetch_by_date(date)? {
        Some(plan) => plan,
        None => {
            // otherwise create
            r.add_plan(date, "")?;
            let msg = format!("could not find inserted plan on date: {:?}", date);
            r.fetch_by_date(date).map(|o| o.expect(&msg))?
        }
    };
    for i in add_tasks {
        r.add_planned_task(&plan.date, i)?;
    }
    for i in remove_tasks {
        r.remove_planned_task(&plan.date, i)?;
    }
    Ok(())
}
