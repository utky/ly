use anyhow::Result;
use rusqlite::{params, Connection, NO_PARAMS};
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
}

pub fn connect() -> Result<Session> {
  let path = "./ly.db";
  let conn = Connection::open(&path)?;
  Ok(Session::new(conn))
}

pub fn connect_memory() -> Result<Session> {
  let conn = Connection::open_in_memory()?;
  Ok(Session::new(conn))
}

pub fn initialize() -> Result<()> {
    let s = connect()?;
    for stmt in ddl::STATEMENTS.iter() {
      s.conn.execute(stmt, NO_PARAMS)?;
    }
    Ok(())
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

impl task::Fetch for Session {
  fn fetch_task_by_id(&mut self, id: Id) -> Result<Option<task::Task>> {
    self.conn.query_row_and_then("SELECT id, lane_id, summary, created_at, updated_at FROM tasks WHERE id = ?", params![id], |row| {
      Ok(Some(task::Task {
        id: row.get(0)?,
        lane_id: row.get(1)?,
        summary: row.get(2)?,
        created_at: row.get(3)?,
        updated_at: row.get(4)?,
      }))
    })
  }
  fn fetch_all_tasks(&mut self, lane_name: &str) -> Result<Vec<task::Task>> {
    let mut stmt = self.conn.prepare("SELECT id, lane_id, summary, created_at, updated_at FROM tasks WHERE EXISTS (SELECT id FROM lanes WHERE name = ? AND lanes.id = tasks.lane_id)")?;
    let rows = stmt.query_map(params![lane_name], |row| {
      Ok(task::Task {
        id: row.get(0)?,
        lane_id: row.get(1)?,
        summary: row.get(2)?,
        created_at: row.get(3)?,
        updated_at: row.get(4)?,
      })
    })?;
    let mut results = Vec::new();
    for r in rows {
      results.push(r?);
    }
    Ok(results)
  }
}
