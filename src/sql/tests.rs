use super::lane::Fetch as LaneFetch;
use super::priority::Fetch as PriorityFetch;
use super::stats;
use super::task::{Add, Fetch as TaskFetch, Mod as TaskMod, Task};
use super::timer;
use super::todo;
use super::Session;
use crate::core::pomodoro;
use crate::core::stats::Fetch;
use crate::core::Id;
use anyhow::Result;
use chrono::{DateTime, TimeZone, Utc};
use rusqlite::Connection;

const TASK_SUMMARY: &str = "test1";

fn connect_memory() -> Result<Session> {
    let conn = Connection::open_in_memory()?;
    Ok(Session::new(conn))
}

fn get_initialized_session() -> Session {
    let mut session = connect_memory().expect("failed to aquire session");
    session.initialize().expect("failed to initialize session");
    session
}

fn fetch_first_created_task(session: &mut Session) -> Result<Task> {
    let task = session
        .fetch_task_by_id(1)?
        .expect("Could not found task by id 1");
    Ok(task)
}

fn add_test_task(session: &mut Session) -> Result<()> {
    let _ = session.add_task(1, 0, TASK_SUMMARY, 3)?;
    Ok(())
}

#[test]
fn test_fetch_lane() -> Result<()> {
    let mut session = get_initialized_session();
    let l = session
        .fetch_lane_by_name("backlog")?
        .expect("returned value should be Some");
    assert_eq!(l.name, "backlog");
    Ok(())
}

#[test]
fn test_insert_fetch_task_by_id() -> Result<()> {
    let mut session = get_initialized_session();
    add_test_task(&mut session)?;
    let t = fetch_first_created_task(&mut session)?;
    assert_eq!(t.lane_id, 1);
    assert_eq!(t.priority, 0);
    assert_eq!(t.summary, TASK_SUMMARY);
    assert_eq!(t.estimate, 3);
    Ok(())
}

#[test]
fn test_insert_fetch_all_tasks() -> Result<()> {
    let mut session = get_initialized_session();
    add_test_task(&mut session)?;
    let _ = session.add_task(1, 1, "test2", 3)?;
    let backlog = session.fetch_all_tasks("backlog")?;

    assert_eq!(backlog.len(), 2);
    assert_eq!(backlog[0].lane_id, 1);
    assert_eq!(backlog[0].priority, 1);
    assert_eq!(backlog[0].summary, "test2");
    assert_eq!(backlog[0].estimate, 3);
    assert_eq!(backlog[1].lane_id, 1);
    assert_eq!(backlog[1].priority, 0);
    assert_eq!(backlog[1].summary, TASK_SUMMARY);
    assert_eq!(backlog[1].estimate, 3);
    let todo = session.fetch_all_tasks("todo")?;
    assert_eq!(todo.len(), 0);
    Ok(())
}

#[test]
fn test_fetch_priority() -> Result<()> {
    let mut session = get_initialized_session();
    let h = session.fetch_priority_by_name("h")?;
    assert_eq!(h.name, "h");
    Ok(())
}

#[test]
fn test_mod_task_move_lane() -> Result<()> {
    let mut session = get_initialized_session();
    add_test_task(&mut session)?;
    let _ = session.mod_task(1, Some(&2), None, None, None);
    let t = fetch_first_created_task(&mut session)?;
    assert_eq!(t.lane_id, 2);
    Ok(())
}

#[test]
fn test_mod_task_higher_priority_and_new_summary() -> Result<()> {
    let mut session = get_initialized_session();
    add_test_task(&mut session)?;
    let _ = session.mod_task(1, None, Some(&3), Some("test1 new"), None)?;
    let t = fetch_first_created_task(&mut session)?;
    assert_eq!(t.priority, 3);
    assert_eq!(t.summary, "test1 new");
    Ok(())
}

#[test]
fn test_mod_plan_add_task() -> Result<()> {
    let first_task_id = 1;
    let mut session = get_initialized_session();
    add_test_task(&mut session)?;
    let d = Utc.ymd(2015, 3, 14).and_hms(0, 0, 0);
    let a = vec![first_task_id];
    let r = Vec::new();
    let _ = todo::mod_todo(&mut session, &d, &a, &r)?;
    let ts = todo::list_todo_tasks(&mut session, &d)?;
    assert_eq!(ts[0].priority, 0);
    assert_eq!(ts[0].estimate, 3);
    assert_eq!(ts[0].actual, 0);
    assert_eq!(ts[0].summary, TASK_SUMMARY);
    Ok(())
}

#[test]
fn test_mod_plan_remove_task() -> Result<()> {
    let first_task_id = 1;
    let mut session = get_initialized_session();
    add_test_task(&mut session)?;
    let d = Utc.ymd(2015, 3, 14).and_hms(0, 0, 0);
    let include_task = vec![first_task_id];
    let empty = Vec::new();
    let _ = todo::mod_todo(&mut session, &d, &include_task, &empty)?;
    let _ = todo::mod_todo(&mut session, &d, &empty, &include_task)?;
    let ts = todo::list_todo_tasks(&mut session, &d)?;
    assert_eq!(ts.len(), 0);
    Ok(())
}

fn complete_pomodoro<R>(r: &mut R, task_id: Id, started_at: DateTime<Utc>) -> Result<()>
where
    R: pomodoro::Complete,
{
    r.complete_pomodoro(task_id, started_at)
}

fn fetch_by_task_id<R>(r: &mut R, task_id: Id) -> Result<Vec<pomodoro::Pomodoro>>
where
    R: pomodoro::Fetch,
{
    r.fetch_by_task_id(task_id)
}

#[test]
fn test_fetch_pomodoro_by_task_id() -> Result<()> {
    let first_task_id = 1;
    let mut session = get_initialized_session();
    add_test_task(&mut session)?;
    let now = Utc::now();
    complete_pomodoro(&mut session, first_task_id, now)?;
    let pomodoros = fetch_by_task_id(&mut session, first_task_id)?;
    assert_eq!(pomodoros.len(), 1, "count of pomodoro");
    assert_eq!(pomodoros[0].task_id, first_task_id, "task_od");
    assert_eq!(pomodoros[0].started_at, now, "started_at");
    Ok(())
}
#[test]
fn test_fetch_todo_task_with_pomodoro() -> Result<()> {
    let first_task_id = 1;
    let mut session = get_initialized_session();
    add_test_task(&mut session)?;
    let d = Utc.ymd(2015, 3, 14).and_hms(0, 0, 0);
    let a = vec![first_task_id];
    let r = Vec::new();
    let _ = todo::mod_todo(&mut session, &d, &a, &r)?;

    let started = Utc.ymd(2015, 3, 14).and_hms(1, 0, 0);
    complete_pomodoro(&mut session, first_task_id, started)?;

    let ts = todo::list_todo_tasks(&mut session, &d)?;
    assert_eq!(ts.len(), 1);
    assert_eq!(ts[0].priority, 0, "priority");
    assert_eq!(ts[0].estimate, 3, "estimate");
    assert_eq!(ts[0].summary, TASK_SUMMARY, "summary");
    assert_eq!(ts[0].actual, 1, "actual");
    Ok(())
}

#[test]
fn test_take_break() -> Result<()> {
    let duration_min = 5;
    let mut session = get_initialized_session();
    let _ = timer::take_break(&mut session, &timer::TimerType::ShortBreak, duration_min)?;
    let created_timer =
        timer::get_current_timer(&mut session)?.expect("short break timer not found");
    assert_eq!(created_timer.id, 0);
    assert_eq!(created_timer.timer_type, timer::TimerType::ShortBreak);
    assert_eq!(created_timer.duration_min, duration_min);
    assert_eq!(created_timer.label, "short break");
    Ok(())
}

#[test]
fn test_start_pomodoro() -> Result<()> {
    let duration_min = 5;
    let first_task_id = 1;
    let mut session = get_initialized_session();
    add_test_task(&mut session)?;
    let _ = timer::pomodoro(&mut session, first_task_id, duration_min)?;
    let created_timer = timer::get_current_timer(&mut session)?.expect("pomodoro timer not found");
    assert_eq!(created_timer.id, 0);
    assert_eq!(created_timer.timer_type, timer::TimerType::Pomodoro);
    assert_eq!(created_timer.duration_min, duration_min);
    assert_eq!(created_timer.label, TASK_SUMMARY);
    Ok(())
}

#[test]
fn test_fetch_daily_summary() -> Result<()> {
    let first_task_id = 1;
    let mut session = get_initialized_session();
    add_test_task(&mut session)?;
    let started = Utc.ymd(2015, 3, 14).and_hms(1, 0, 0);
    complete_pomodoro(&mut session, first_task_id, started)?;
    let range = stats::SummaryRange {
        start: Utc.ymd(2015, 3, 14).and_hms(1, 0, 0),
        end: Utc.ymd(2015, 3, 15).and_hms(1, 0, 0),
    };
    let summaries = session.fetch_daily_summary(&range)?;
    assert_eq!(summaries.len(), 1);
    assert_eq!(
        summaries[0].date,
        Utc.ymd(2015, 3, 14).and_hms(0, 0, 0),
        "date"
    );
    assert_eq!(summaries[0].task_id, first_task_id, "task_id");
    assert_eq!(summaries[0].pomodoro_count, 1, "pomodoro_count");
    assert_eq!(summaries[0].interruption_count, 0, "interruption_count");
    Ok(())
}
