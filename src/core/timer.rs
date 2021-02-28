use super::common::{Id, RepositoryError};
use super::pomodoro;
use super::task;
use anyhow::{bail, Result};
use chrono::serde::ts_milliseconds;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::convert::TryFrom;
use std::fmt::{Display, Formatter, Result as FmtResult};

const TIMER_TYPE_POMODORO: u8 = 0;
const TIMER_TYPE_SHORT_BREAK: u8 = 1;
const TIMER_TYPE_LONG_BREAK: u8 = 2;

#[derive(Serialize, Deserialize, PartialEq, Debug, Copy, Clone)]
#[serde(into = "u8", try_from = "u8")]
pub enum TimerType {
    Pomodoro,
    ShortBreak,
    LongBreak,
}

#[derive(Debug)]
pub struct TimerTypeFromIntError {
    value: u8,
}

impl Display for TimerTypeFromIntError {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "Failed to convert TimerType from")
    }
}

impl std::error::Error for TimerTypeFromIntError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        None
    }
}

impl Into<u8> for TimerType {
    fn into(self) -> u8 {
        match self {
            TimerType::ShortBreak => TIMER_TYPE_SHORT_BREAK,
            TimerType::LongBreak => TIMER_TYPE_LONG_BREAK,
            TimerType::Pomodoro => TIMER_TYPE_POMODORO,
        }
    }
}

impl TryFrom<u8> for TimerType {
    type Error = TimerTypeFromIntError;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            TIMER_TYPE_POMODORO => Ok(TimerType::Pomodoro),
            TIMER_TYPE_SHORT_BREAK => Ok(TimerType::ShortBreak),
            TIMER_TYPE_LONG_BREAK => Ok(TimerType::LongBreak),
            _ => Err(TimerTypeFromIntError { value }),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Timer {
    pub id: Id,
    pub timer_type: TimerType,
    pub label: String,
    #[serde(with = "ts_milliseconds")]
    pub started_at: DateTime<Utc>,
    pub duration_min: i64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TimerTask {
    pub timer_id: Id,
    pub task_id: Id,
}

pub trait TimerTaskAdd {
    fn add_timer_task(&mut self, task_id: &Id) -> Result<()>;
}

pub trait TimerTaskRemove {
    fn remove_timer_task(&mut self) -> Result<()>;
}

pub trait TimerTaskGet {
    fn get_timer_task(&mut self) -> Result<Option<TimerTask>>;
}

pub trait Lifecycle {
    /// Start timer
    fn start(&mut self, timer_type: &TimerType, label: &str, duration_min: i64) -> Result<Timer>;
    /// Clear timer
    fn complete(&mut self) -> Result<()>;
}

pub trait Get {
    fn get(&mut self) -> Result<Option<Timer>>;
}

pub fn pomodoro<R>(r: &mut R, task_id: Id, duration_min: i64) -> Result<Timer>
where
    R: Lifecycle + TimerTaskAdd + task::Fetch,
{
    let task = r
        .fetch_task_by_id(task_id)?
        .ok_or(RepositoryError::NotFound)?;
    let timer = r.start(&TimerType::Pomodoro, &task.summary, duration_min)?;
    r.add_timer_task(&task.id)?;
    Ok(timer)
}

pub fn take_break<R>(r: &mut R, timer_type: &TimerType, duration_min: i64) -> Result<Timer>
where
    R: Lifecycle,
{
    let label: Result<&str, anyhow::Error> = match timer_type {
        TimerType::ShortBreak => Ok("short break"),
        TimerType::LongBreak => Ok("long break"),
        _ => bail!("illegal timer type {:?}", timer_type),
    };
    let timer = r.start(timer_type, label?, duration_min)?;
    Ok(timer)
}

pub fn complete<R>(r: &mut R, timer: &Timer) -> Result<()>
where
    R: Lifecycle + TimerTaskRemove + TimerTaskGet + task::Fetch + pomodoro::Complete,
{
    match timer.timer_type {
        TimerType::ShortBreak => {}
        TimerType::LongBreak => {}
        TimerType::Pomodoro => match r.get_timer_task()? {
            Some(timer_task) => {
                let task = r
                    .fetch_task_by_id(timer_task.task_id)?
                    .ok_or(RepositoryError::NotFound)?;
                r.complete_pomodoro(task.id, timer.started_at)?;
            }
            None => bail!("timer_task was not found"),
        },
    };
    r.remove_timer_task()?;
    r.complete()
}

pub fn get_current_timer<R>(r: &mut R) -> Result<Option<Timer>>
where
    R: Get + task::Fetch,
{
    let timer = r.get()?;
    Ok(timer)
}
