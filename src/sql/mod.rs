use anyhow::{Result, Context};
use rusqlite::{params, Connection, NO_PARAMS, Row, Result as SqlResult};
use crate::core::{Id};
use crate::core::lane;
use crate::core::task;

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

impl lane::Fetch for Session {
  fn fetch_lane_by_name(&mut self, name: &str) -> Result<Option<lane::Lane>> {
    self.conn.query_row_and_then("SELECT id, name, created_at, updated_at FROM lanes WHERE name = ?", params![name], |row| {
      Ok(Some(lane::Lane {
        id: row.get(0)?,
        name: row.get(1)?,
        created_at: row.get(2)?,
        updated_at: row.get(3)?,
      }))
    })
  }
}

impl task::Add for Session {
  fn add_task(&mut self, lane_id: Id, summary: &str) -> Result<()> {
    self.conn.execute("INSERT INTO tasks(lane_id, summary) VALUES (?, ?)", params![lane_id, summary])?;
    Ok(())
  }
}

fn row_to_task(row: &Row) -> SqlResult<task::Task> {
  Ok(task::Task {id: row.get(0)?, lane_id: row.get(1)?, summary: row.get(2)?, created_at: row.get(3)?, updated_at: row.get(4)?})
}

impl task::Fetch for Session {
  fn fetch_task_by_id(&mut self, id: Id) -> Result<Option<task::Task>> {
    self.conn.query_row_and_then("SELECT id, lane_id, summary, created_at, updated_at FROM tasks WHERE id = ?", params![id], |row| {
      let t = row_to_task(row)?;
      Ok(Some(t))
    })
  }
  fn fetch_all_tasks(&mut self, lane_name: &str) -> Result<Vec<task::Task>> {
    let mut stmt = self.conn.prepare("SELECT id, lane_id, summary, created_at, updated_at FROM tasks WHERE EXISTS (SELECT id FROM lanes WHERE name = ? AND lanes.id = tasks.lane_id)")?;
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

#[cfg(test)]
mod tests {
  use anyhow::Result;
  use rusqlite::Connection;
  use super::Session;
  use super::lane::{Fetch as LaneFetch};
  use super::task::{Add, Fetch as TaskFetch};
  
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
    let _ = session.add_task(2, "test").expect("adding task should not failed");
    let t = session.fetch_task_by_id(1).expect("could not fetch task by id 1").expect("returned value should be Some");
    assert_eq!(t.lane_id, 2);
    assert_eq!(t.summary, "test");
  }
  #[test]
  fn test_insert_fetch_all_tasks() {
    let mut session = get_initialized_session();
    let _ = session.add_task(1, "test1").expect("adding task should not failed");
    let backlog = session.fetch_all_tasks("backlog").expect("failed to fech backlog tasks");
    assert_eq!(backlog.len(), 1);
    assert_eq!(backlog[0].lane_id, 1);
    assert_eq!(backlog[0].summary, "test1");
    let todo = session.fetch_all_tasks("todo").expect("failed to fech backlog tasks");
    assert_eq!(todo.len(), 0);
  }
}
