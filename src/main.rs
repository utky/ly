extern crate clap;
use clap::{Arg, App, SubCommand};
use rusqlite::{params, Connection, Result, NO_PARAMS};
use warp::Filter;

mod ddl;
mod public;

struct Task {
  id: u32,
  uuid: String,
  summary: String,

}

fn connect() -> Result<Connection> {
  let path = "./ly.db";
  Connection::open(&path)
}

async fn list_task() -> Result<Vec<(u32, String)>> {
  let conn = connect()?;
  let mut stmt = conn.prepare("SELECT id, uuid, lane_id, summary, created_at, updated_at FROM tasks WHERE lane_id = 1")?;
  let rows = stmt.query_map(NO_PARAMS, |row| {
    let i = row.get(0)?;
    let s = row.get(1)?;
    Ok((i, s))
  })?;
  let mut results = Vec::new();
  for r in rows {
    results.push(r?);
  }
  Ok(results)
}

async fn add_task() -> Result<()> {
  let conn = connect()?;
  conn.execute("INSERT INTO tasks(uuid, lane_id, summary) VALUES ('uuid', 1, 'test tasks')", NO_PARAMS)?;
  Ok(())
}

async fn rm_task() -> Result<()> {
  let conn = connect()?;
  conn.execute("DELETE FROM tasks WHERE uuid = 'uuid'", params![])?;
  Ok(())
}

async fn start_server() {
  let routes = warp::any().map(|| public::index_html());
  warp::serve(routes).run(([127, 0, 0, 1], 8080)).await;
}

async fn initialize() -> Result<()> {
  let conn = connect()?;

  for s in ddl::STATEMENTS.iter() {
    conn.execute(s, params![])?;
  }

  Ok(())
}

#[tokio::main]
async fn main() {
  let init = SubCommand::with_name("init").about("initialize database");
  let server = SubCommand::with_name("server").about("start server");
  let task_list = SubCommand::with_name("ls").about("list tasks");
  let task_add = SubCommand::with_name("add").about("add task");
  let task_rm = SubCommand::with_name("rm").about("remove task");
  let task = SubCommand::with_name("task").about("manage task")
    .subcommand(task_list)
    .subcommand(task_add)
    .subcommand(task_rm);
  let matches = App::new("ly")
    .version("1.0")
    .author("Yutaka Imamura")
    .about("Pomodoro time tracker")
    .subcommand(init)
    .subcommand(server)
    .subcommand(task)
    .get_matches();

  match matches.subcommand() {
    ("init", _) => {
      initialize().await;
    }
    ("server", _) => {
      start_server().await
    }
    ("task", Some(task_m)) => {
      match task_m.subcommand() {
        ("ls", _) => {
          for r in list_task().await.expect("ls tasks") {
            println!("{:?}", r)
          }
        },
        ("add", _) => {
          add_task().await.expect("add test");
        },
        ("rm", _) => {
          rm_task().await.expect("rm task");
        },
        _ => panic!("invalid options")
      }
    }
    _ => panic!("invalid options")
  }
}
