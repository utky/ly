use anyhow::{Result, Context};
use rusqlite::{params, Connection, NO_PARAMS, Row, Result as SqlResult};
use crate::core::{Id};
use crate::core::lane;
use crate::core::task;
use crate::core::priority;

pub mod ddl;


pub struct Session {
  conn: Connection
}

impl Session {
  fn new(conn: Connection) -> Session {
    Session { conn: conn }
  }
  pub fn connect() -> Result<Session> {
    let path = "./ly.db";
    let conn = Connection::open(&path)?;
    Ok(Session::new(conn))
  }

   pub fn initialize(&mut self) -> Result<()> {
    for stmt in ddl::STATEMENTS.iter() {
      self.conn.execute(stmt, NO_PARAMS).with_context(|| format!("Failed to run statement {}", stmt))?;
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
static FETCH_LANE_BY_NAME: &str = "SELECT id, name, created_at, updated_at FROM lanes WHERE name = ?";
static FETCH_ALL_LANES: &str = "SELECT id, name, created_at, updated_at FROM lanes";
impl lane::Fetch for Session {
  fn fetch_lane_by_name(&mut self, name: &str) -> Result<Option<lane::Lane>> {
    self.conn.query_row_and_then(FETCH_LANE_BY_NAME, params![name], |row| {
      Ok(Some(row_to_lane(row)?))
    })
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
static FETCH_PRIORITY_BY_NAME: &str = "SELECT id, name, created_at, updated_at FROM priorities WHERE name = ?";
static FETCH_ALL_PRIORITY: &str = "SELECT id, name, created_at, updated_at FROM priorities";
impl priority::Fetch for Session {
  fn fetch_priority_by_name(&mut self, name: &str) -> Result<priority::Priority> {
    self.conn.query_row_and_then(FETCH_PRIORITY_BY_NAME, params![name], |row| {
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

static ADD_TASK: &str = "INSERT INTO tasks(lane_id, priority, summary) VALUES (?, ?, ?)";
impl task::Add for Session {
  fn add_task(&mut self, lane_id: Id, priority: Id, summary: &str) -> Result<()> {
    self.conn.execute(ADD_TASK, params![lane_id, priority, summary])?;
    Ok(())
  }
}

fn row_to_task(row: &Row) -> SqlResult<task::Task> {
  Ok(task::Task {id: row.get(0)?, lane_id: row.get(1)?, priority: row.get(2)?, summary: row.get(3)?, created_at: row.get(4)?, updated_at: row.get(5)?})
}

static FETCH_TASK_BY_ID: &str = "SELECT id, lane_id, priority, summary, created_at, updated_at FROM tasks WHERE id = ?";
static FETCH_ALL_TASKS: &str = "SELECT id, lane_id, priority, summary, created_at, updated_at FROM tasks WHERE EXISTS (SELECT id FROM lanes WHERE name = ? AND lanes.id = tasks.lane_id) ORDER BY priority DESC";
impl task::Fetch for Session {
  fn fetch_task_by_id(&mut self, id: Id) -> Result<Option<task::Task>> {
    self.conn.query_row_and_then(FETCH_TASK_BY_ID, params![id], |row| {
      let t = row_to_task(row)?;
      Ok(Some(t))
    })
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

static MOD_TASK: &str = "UPDATE tasks SET lane_id = ?, priority = ?, summary = ?, updated_at = datetime('now') WHERE id = ?";
impl task::Mod for Session {
  fn mod_task(&mut self, id: Id, lane_id: Option<&Id>, priority: Option<&Id>, summary: Option<&str>) -> Result<()> {
    let old = self.conn.query_row_and_then(FETCH_TASK_BY_ID, params![id], |row| row_to_task(row))?;
    let set_lane_id = lane_id.unwrap_or(&old.lane_id);
    let set_priority = priority.unwrap_or(&old.priority);
    let set_summary = summary.unwrap_or(&old.summary);
    self.conn.execute(MOD_TASK, params![set_lane_id, set_priority, set_summary, id])?;
    Ok(())
  }
}

#[cfg(test)]
mod tests {
  use anyhow::Result;
  use rusqlite::Connection;
  use super::Session;
  use super::lane::{Fetch as LaneFetch};
  use super::task::{Add, Fetch as TaskFetch, Mod as TaskMod};
  use super::priority::{Fetch as PriorityFetch};
  
  fn connect_memory() -> Result<Session> {
    let conn = Connection::open_in_memory()?;
    Ok(Session::new(conn))
  }

  fn get_initialized_session() -> Session {
    let mut session = connect_memory().expect("failed to aquire session");
    session.initialize().expect("failed to initialize session");
    session
  }

  #[test]
  fn test_fetch_lane() {
    let mut session = get_initialized_session();
    let l = session.fetch_lane_by_name("backlog").expect("failed to fetch backlog lane").expect("returned value should be Some");
    assert_eq!(l.name, "backlog");
  }

  #[test]
  fn test_insert_fetch_task_by_id() {
    let mut session = get_initialized_session();
    let _ = session.add_task(2, 0, "test").expect("adding task should not failed");
    let t = session.fetch_task_by_id(1).expect("could not fetch task by id 1").expect("returned value should be Some");
    assert_eq!(t.lane_id, 2);
    assert_eq!(t.priority, 0);
    assert_eq!(t.summary, "test");
  }

  #[test]
  fn test_insert_fetch_all_tasks() {
    let mut session = get_initialized_session();
    let _ = session.add_task(1, 0, "test1").expect("adding task should not failed");
    let _ = session.add_task(1, 1, "test2").expect("adding task should not failed");
    let backlog = session.fetch_all_tasks("backlog").expect("failed to fech backlog tasks");
    assert_eq!(backlog.len(), 2);
    assert_eq!(backlog[0].lane_id, 1);
    assert_eq!(backlog[0].priority, 1);
    assert_eq!(backlog[0].summary, "test2");
    assert_eq!(backlog[1].lane_id, 1);
    assert_eq!(backlog[1].priority, 0);
    assert_eq!(backlog[1].summary, "test1");
    let todo = session.fetch_all_tasks("todo").expect("failed to fech backlog tasks");
    assert_eq!(todo.len(), 0);
  }

  #[test]
  fn test_fetch_priority() {
    let mut session = get_initialized_session();
    let h = session.fetch_priority_by_name("h").expect("failed to fetch high priority");
    assert_eq!(h.name, "h");
  }

  #[test]
  fn test_mod_task_move_lane() {
    let mut session = get_initialized_session();
    let _ = session.add_task(1, 0, "test1").expect("adding task test1 should not failed");
    let _ = session.mod_task(1, Some(&2), None, None).expect("modify task 1");
    let t = session.fetch_task_by_id(1).expect("could not fetch task by id 1").expect("returned value should be Some");
    assert_eq!(t.lane_id, 2);
  }

  #[test]
  fn test_mod_task_higher_priority_and_new_summary() {
    let mut session = get_initialized_session();
    let _ = session.add_task(1, 0, "test1").expect("adding task test1 should not failed");
    let _ = session.mod_task(1, None, Some(&3), Some("test1 new")).expect("modify task 1");
    let t = session.fetch_task_by_id(1).expect("could not fetch task by id 1").expect("returned value should be Some");
    assert_eq!(t.priority, 3);
    assert_eq!(t.summary, "test1 new");
  }
}
