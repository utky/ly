use crate::config::Config;
use crate::core::lane;
use crate::core::pomodoro;
use crate::core::priority;
use crate::core::stats;
use crate::core::stats::DailySummary;
use crate::core::task;
use crate::core::timer;
use crate::core::todo;
use crate::core::Id;
use anyhow::{Context, Result};
use chrono::{DateTime, Duration, Utc};
use rusqlite::types::{ToSqlOutput, Value};
use rusqlite::{
    params, Connection, Error, OptionalExtension, Result as SqlResult, Row, ToSql, NO_PARAMS,
};
use std::convert::TryFrom;
use timer::TimerType;

pub mod ddl;

pub struct Session {
    conn: Connection,
}

impl Session {
    fn new(conn: Connection) -> Session {
        Session { conn }
    }
    pub fn connect(config: &Config) -> Result<Session> {
        let conn = Connection::open(&config.database)?;
        Ok(Session::new(conn))
    }

    pub fn initialize(&mut self) -> Result<()> {
        for stmt in ddl::STATEMENTS.iter() {
            self.conn
                .execute(stmt, NO_PARAMS)
                .with_context(|| format!("Failed to run statement {}", stmt))?;
        }
        Ok(())
    }
}

fn row_to_lane(row: &Row) -> SqlResult<lane::Lane> {
    Ok(lane::Lane {
        id: row.get(0)?,
        name: row.get(1)?,
        created_at: row.get(2)?,
        updated_at: row.get(3)?,
    })
}
static FETCH_LANE_BY_NAME: &str =
    "SELECT id, name, created_at, updated_at FROM lanes WHERE name = ?";
static FETCH_ALL_LANES: &str = "SELECT id, name, created_at, updated_at FROM lanes";
impl lane::Fetch for Session {
    fn fetch_lane_by_name(&mut self, name: &str) -> Result<Option<lane::Lane>> {
        let lane = self
            .conn
            .query_row_and_then(FETCH_LANE_BY_NAME, params![name], row_to_lane)
            .optional()?;
        Ok(lane)
    }
    fn fetch_all_lanes(&mut self) -> Result<Vec<lane::Lane>> {
        let mut stmt = self.conn.prepare(FETCH_ALL_LANES)?;
        let rows = stmt.query_map(params![], |row| Ok(row_to_lane(row)?))?;
        let mut results = Vec::new();
        for r in rows {
            results.push(r?);
        }
        Ok(results)
    }
}

fn row_to_priority(row: &Row) -> SqlResult<priority::Priority> {
    Ok(priority::Priority {
        id: row.get(0)?,
        name: row.get(1)?,
        created_at: row.get(2)?,
        updated_at: row.get(3)?,
    })
}
static FETCH_PRIORITY_BY_NAME: &str =
    "SELECT id, name, created_at, updated_at FROM priorities WHERE name = ?";
static FETCH_ALL_PRIORITY: &str = "SELECT id, name, created_at, updated_at FROM priorities";
impl priority::Fetch for Session {
    fn fetch_priority_by_name(&mut self, name: &str) -> Result<priority::Priority> {
        self.conn
            .query_row_and_then(FETCH_PRIORITY_BY_NAME, params![name], |row| {
                Ok(row_to_priority(row)?)
            })
    }
    fn fetch_all_priority(&mut self) -> Result<Vec<priority::Priority>> {
        let mut stmt = self.conn.prepare(FETCH_ALL_PRIORITY)?;
        let rows = stmt.query_map(params![], |row| Ok(row_to_priority(row)?))?;
        let mut results = Vec::new();
        for r in rows {
            results.push(r?);
        }
        Ok(results)
    }
}

static ADD_TASK: &str =
    "INSERT INTO tasks(lane_id, priority, summary, estimate) VALUES (?, ?, ?, ?)";
impl task::Add for Session {
    fn add_task(&mut self, lane_id: Id, priority: Id, summary: &str, estimate: i64) -> Result<()> {
        self.conn
            .execute(ADD_TASK, params![lane_id, priority, summary, estimate])?;
        Ok(())
    }
}

fn row_to_task(row: &Row) -> SqlResult<task::Task> {
    Ok(task::Task {
        id: row.get(0)?,
        lane_id: row.get(1)?,
        priority: row.get(2)?,
        summary: row.get(3)?,
        estimate: row.get(4)?,
        created_at: row.get(5)?,
        updated_at: row.get(6)?,
    })
}

static FETCH_TASK_BY_ID: &str = "SELECT id, lane_id, priority, summary, estimate, created_at, updated_at FROM tasks WHERE id = ?";
static FETCH_ALL_TASKS: &str = "SELECT id, lane_id, priority, summary, estimate, created_at, updated_at FROM tasks WHERE EXISTS (SELECT id FROM lanes WHERE name = ? AND lanes.id = tasks.lane_id) ORDER BY priority DESC";
impl task::Fetch for Session {
    fn fetch_task_by_id(&mut self, id: Id) -> Result<Option<task::Task>> {
        let t = self
            .conn
            .query_row(FETCH_TASK_BY_ID, params![id], row_to_task)
            .optional()?;
        Ok(t)
    }
    fn fetch_all_tasks(&mut self, lane_name: &str) -> Result<Vec<task::Task>> {
        let mut stmt = self.conn.prepare(FETCH_ALL_TASKS)?;
        let rows = stmt.query_map(params![lane_name], |row| {
            let t = row_to_task(row)?;
            Ok(t)
        })?;
        let mut results = Vec::new();
        for r in rows {
            results.push(r?);
        }
        Ok(results)
    }
}

static MOD_TASK: &str = "UPDATE tasks SET lane_id = ?, priority = ?, summary = ?, estimate = ?, updated_at = datetime('now') WHERE id = ?";
impl task::Mod for Session {
    fn mod_task(
        &mut self,
        id: Id,
        lane_id: Option<&Id>,
        priority: Option<&Id>,
        summary: Option<&str>,
        estimate: Option<i64>,
    ) -> Result<()> {
        let old = self
            .conn
            .query_row_and_then(FETCH_TASK_BY_ID, params![id], |row| row_to_task(row))?;
        let set_lane_id = lane_id.unwrap_or(&old.lane_id);
        let set_priority = priority.unwrap_or(&old.priority);
        let set_summary = summary.unwrap_or(&old.summary);
        let set_estimate = estimate.unwrap_or(old.estimate);
        self.conn.execute(
            MOD_TASK,
            params![set_lane_id, set_priority, set_summary, set_estimate, id],
        )?;
        Ok(())
    }
}

fn row_to_timer(row: &Row) -> SqlResult<timer::Timer> {
    Ok(timer::Timer {
        id: row.get(0)?,
        timer_type: {
            let int_val: u8 = row.get(1)?;
            let v = TimerType::try_from(int_val);
            v.map_err(|e| {
                Error::FromSqlConversionFailure(8, rusqlite::types::Type::Integer, Box::new(e))
            })?
        },
        label: row.get(2)?,
        started_at: row.get(3)?,
        duration_min: row.get(4)?,
    })
}

impl ToSql for timer::TimerType {
    fn to_sql(&self) -> rusqlite::Result<ToSqlOutput> {
        Ok(ToSqlOutput::Owned(Value::Integer(*self as i64)))
    }
}

static INSERT_TIMER_TASK: &str = "INSERT INTO timer_tasks(timer_id, task_id) VALUES (0, ?)";
static DELETE_TIMER_TASK: &str = "DELETE FROM timer_tasks WHERE timer_id = 0";
static GET_TIMER_TASK: &str = "SELECT timer_id, task_id FROM timer_tasks WHERE timer_id = 0";
impl timer::TimerTaskAdd for Session {
    fn add_timer_task(&mut self, task_id: &Id) -> Result<()> {
        self.conn.execute(INSERT_TIMER_TASK, params![task_id])?;
        Ok(())
    }
}

impl timer::TimerTaskRemove for Session {
    fn remove_timer_task(&mut self) -> Result<()> {
        self.conn.execute(DELETE_TIMER_TASK, NO_PARAMS)?;
        Ok(())
    }
}

impl timer::TimerTaskGet for Session {
    fn get_timer_task(&mut self) -> Result<Option<timer::TimerTask>> {
        let timer_task = self
            .conn
            .query_row(GET_TIMER_TASK, NO_PARAMS, |row| {
                Ok(timer::TimerTask {
                    timer_id: row.get(0)?,
                    task_id: row.get(1)?,
                })
            })
            .optional()?;
        Ok(timer_task)
    }
}

static START: &str = "INSERT INTO timers(id, timer_type, label, duration_min) VALUES (0, ?, ?, ?)";
static COMPLETE: &str = "DELETE FROM timers WHERE id = 0";
static GET_TIMER: &str =
    "SELECT id, timer_type, label, started_at, duration_min FROM timers WHERE id = 0";
impl timer::Lifecycle for Session {
    fn start(
        &mut self,
        timer_type: &timer::TimerType,
        label: &str,
        duration_min: i64,
    ) -> Result<timer::Timer> {
        self.conn
            .execute(START, params![timer_type, label, duration_min])?;
        let c = self.conn.query_row(GET_TIMER, NO_PARAMS, row_to_timer)?;
        Ok(c)
    }
    fn complete(&mut self) -> Result<()> {
        self.conn.execute(COMPLETE, NO_PARAMS)?;
        Ok(())
    }
}

impl timer::Get for Session {
    fn get(&mut self) -> Result<Option<timer::Timer>> {
        let c = self
            .conn
            .query_row(GET_TIMER, NO_PARAMS, row_to_timer)
            .optional()?;
        Ok(c)
    }
}

fn row_to_pomodoro(row: &Row) -> SqlResult<pomodoro::Pomodoro> {
    Ok(pomodoro::Pomodoro {
        id: row.get(0)?,
        task_id: row.get(1)?,
        started_at: row.get(2)?,
        finished_at: row.get(3)?,
    })
}

static ADD_POMODORO: &str = "INSERT INTO pomodoros(task_id, started_at) VALUES (?, ?)";
impl pomodoro::Complete for Session {
    fn complete_pomodoro(&mut self, task_id: Id, started_at: DateTime<Utc>) -> Result<()> {
        self.conn
            .execute(ADD_POMODORO, params![task_id, started_at])?;
        Ok(())
    }
}

static FETCH_POMODOROS_BY_TASK_ID: &str = "SELECT id, task_id, started_at, finished_at FROM pomodoros WHERE task_id = ? ORDER BY started_at";
impl pomodoro::Fetch for Session {
    fn fetch_by_task_id(&mut self, task_id: Id) -> Result<Vec<pomodoro::Pomodoro>> {
        let mut stmt = self.conn.prepare(FETCH_POMODOROS_BY_TASK_ID)?;
        let rows = stmt.query_map(params![task_id], |row| row_to_pomodoro(row))?;
        let mut results = Vec::new();
        for r in rows {
            results.push(r?);
        }
        Ok(results)
    }
}

/* ---------------------------------------------------------------
 * todo
 * ---------------------------------------------------------------
 */
static INSERT_TODO: &str = "INSERT INTO todo(date, note) VALUES (?, ?)";
impl todo::Add for Session {
    fn add_todo(&mut self, date: &todo::TodoDate, note: &str) -> Result<()> {
        self.conn.execute(INSERT_TODO, params![date, note])?;
        Ok(())
    }
}
fn row_to_todo(row: &Row) -> SqlResult<todo::Todo> {
    Ok(todo::Todo {
        date: row.get(0)?,
        note: row.get(1)?,
        created_at: row.get(2)?,
        updated_at: row.get(3)?,
    })
}
fn row_to_todo_task(row: &Row) -> SqlResult<todo::TodoTask> {
    Ok(todo::TodoTask {
        date: row.get(0)?,
        task_id: row.get(1)?,
        lane_id: row.get(2)?,
        priority: row.get(3)?,
        summary: row.get(4)?,
        estimate: row.get(5)?,
        actual: row.get(6)?,
    })
}
static FETCH_TODO_BY_DATE: &str =
    "SELECT date, note, created_at, updated_at FROM todo WHERE date = ?";
static FETCH_TODO_TASKS: &str = "SELECT
    todo.date AS date,
    task.id AS task_id,
    task.lane_id AS lane_id,
    task.priority AS priority,
    task.summary AS summary,
    task.estimate AS estimate,
    CASE WHEN result.actual IS NULL THEN 0 ELSE result.actual END AS actual
FROM tasks task
JOIN todo_tasks todo ON task.id = todo.task_id
LEFT JOIN (
    SELECT
        task_id AS task_id,
        COUNT(task_id) AS actual
    FROM pomodoros
    WHERE (started_at >= ? AND started_at < ?)
    GROUP BY task_id
) result ON task.id = result.task_id
WHERE todo.date = ?
ORDER BY todo.todo_order
";
impl todo::Fetch for Session {
    fn fetch_by_date(&mut self, date: &todo::TodoDate) -> Result<Option<todo::Todo>> {
        let result = self
            .conn
            .query_row(FETCH_TODO_BY_DATE, params![date], row_to_todo)
            .optional()?;
        Ok(result)
    }
    fn fetch_todo_tasks(&mut self, date: &todo::TodoDate) -> Result<Vec<todo::TodoTask>> {
        let start_time = *date;
        let end_time = start_time + Duration::days(1);
        let mut stmt = self.conn.prepare(FETCH_TODO_TASKS)?;
        let rows = stmt.query_map(params![start_time, end_time, date], |r| row_to_todo_task(r))?;
        let mut results = Vec::new();
        for r in rows {
            results.push(r?);
        }
        Ok(results)
    }
}

static INSERT_TODO_TASK: &str =
    "INSERT INTO todo_tasks(date, task_id, todo_order) VALUES (?, ?, ?)";
static DELETE_TODO_TASK: &str = "DELETE FROM todo_tasks WHERE date = ? AND task_id = ?";
impl todo::Mod for Session {
    fn add_todo_task(
        &mut self,
        date: &todo::TodoDate,
        task_id: &Id,
        todo_order: usize,
    ) -> Result<()> {
        self.conn.execute(
            INSERT_TODO_TASK,
            params![date, task_id, (todo_order as i64)],
        )?;
        Ok(())
    }
    fn remove_todo_task(&mut self, date: &todo::TodoDate, task_id: &Id) -> Result<()> {
        self.conn
            .execute(DELETE_TODO_TASK, params![date, task_id])?;
        Ok(())
    }
}

// TODO: timezone offset as parameters
static FETCH_DAILY_SUMMARY: &str = "
SELECT
    datetime(date(time, '+09:00')),
    task_id,
    SUM(CASE WHEN item_type = 'task' THEN 1 ELSE 0 END) AS pomodoro_count,
    SUM(CASE WHEN item_type = 'intr' THEN 1 ELSE 0 END) AS interruption_count 
FROM (
    SELECT
        started_at AS time,
        task_id,
        'task' AS item_type
    FROM
        pomodoros
    WHERE ? <= started_at AND started_at < ?
    UNION ALL
    SELECT
        created_at AS time,
        task_id,
        'intr' AS item_type
    FROM
        interruptions
    WHERE ? <= created_at AND created_at < ?
) summary_items 
GROUP BY datetime(date(time, '+09:00')), task_id";
fn row_to_daily_summary(row: &Row) -> SqlResult<stats::DailySummary> {
    Ok(stats::DailySummary {
        date: row.get(0)?,
        task_id: row.get(1)?,
        pomodoro_count: row.get(2)?,
        interruption_count: row.get(3)?,
    })
}
impl stats::Fetch for Session {
    fn fetch_daily_summary(&mut self, range: &stats::SummaryRange) -> Result<Vec<DailySummary>> {
        let mut stmt = self.conn.prepare(FETCH_DAILY_SUMMARY)?;
        let rows = stmt.query_map(
            params![range.start, range.end, range.start, range.end],
            |row| Ok(row_to_daily_summary(row)?),
        )?;
        let mut results = Vec::new();
        for r in rows {
            results.push(r?);
        }
        Ok(results)
    }
}

#[cfg(test)]
mod tests;
