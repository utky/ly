extern crate clap;
use clap::{Arg, App, SubCommand};
use rusqlite::{params, Connection, Result};
use warp::Filter;
use tokio::sync::oneshot;

async fn server() {
  let routes = warp::any().map(|| "Hello, World!");
  warp::serve(routes).run(([127, 0, 0, 1], 8080)).await;
}

async fn init() -> Result<()> {
  let path = "./ly.db";
  let conn = Connection::open(&path)?;
  println!("{}", conn.is_autocommit());
  conn.execute(
   "CREATE TABLE lanes (
       id tinyint,
       name varchar NOT NULL UNIQUE,
       created_at timestamp NOT NULL DEFAULT CURRENT_TIMESTAMP,
       updated_at timestamp NOT NULL DEFAULT CURRENT_TIMESTAMP,
       PRIMARY KEY (id)
     )",
      params![],
  )?;
  Ok(())
}

#[tokio::main]
async fn main() {
  let matches = App::new("ly")
    .version("1.0")
    .author("Yutaka Imamura")
    .about("Pomodoro time tracker")
    .subcommand(SubCommand::with_name("init")
      .about("initialize database"))
    .subcommand(SubCommand::with_name("server")
      .about("start server"))
    .get_matches();

  match matches.subcommand_name() {
    Some("init") => {
      init().await;
    }
    Some("server") => {
      server().await
    }
    _ => {}
  }
}
