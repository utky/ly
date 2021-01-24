extern crate clap;
use clap::{App, Arg, SubCommand};
use crate::cli::TaskList;
use anyhow::{Result, bail};
use crate::core::Id;
use crate::core::current;
use std::convert::TryFrom;

mod core;
mod cli;
mod web;
mod sql;
mod public;

struct CleanupCurrent {
  session: sql::Session,
  current: current::Current
}

impl Drop for CleanupCurrent {
  fn drop(&mut self) {
    let _ = crate::core::current::complete(&mut self.session, &self.current);
  }
}

fn start_pomodoro(task_id: Id, duration_min: i64) -> Result<()> {
  let current: Result<current::Current> = {
    let mut session = sql::Session::connect()?;
    crate::core::current::start(&mut session, task_id, duration_min)
  };
  let session = sql::Session::connect()?;
  let mut cleanup = CleanupCurrent {session: session, current: current?};
  let duration_sec: u64 = u64::try_from(duration_min * 60).expect("failed to cast i64 to u64");
  std::thread::sleep(std::time::Duration::from_secs(duration_sec));
  Ok(())
}

#[tokio::main]
async fn main() -> Result<()> {
  let init = SubCommand::with_name("init").about("initialize database");
  let server = SubCommand::with_name("server").about("start server");
  let start = SubCommand::with_name("start").about("start pomodoro for the task")
    .arg(Arg::with_name("id")
      .long("id")
      .short("i")
      .value_name("ID")
      .takes_value(true)
      .required(true))
    .arg(Arg::with_name("duration")
      .long("duration")
      .short("d")
      .value_name("MINUTES")
      .takes_value(true)
      .required(false));
  let task_list = SubCommand::with_name("ls").about("list tasks")
    .arg(Arg::with_name("lane")
      .long("lane")
      .short("l")
      .value_name("LANE_NAME")
      .takes_value(true)
      .required(false));
  let task_add = SubCommand::with_name("add").about("add task")
    .arg(Arg::with_name("summary")
      .long("summary")
      .short("s")
      .value_name("TEXT")
      .takes_value(true)
      .required(true))
    .arg(Arg::with_name("lane")
      .long("lane")
      .short("l")
      .value_name("LANE_NAME")
      .takes_value(true)
      .required(false))
    .arg(Arg::with_name("priority")
      .long("priority")
      .short("p")
      .value_name("PRIORITY_NAME")
      .takes_value(true)
      .required(false))
    .arg(Arg::with_name("estimate")
      .long("estimate")
      .short("e")
      .value_name("NUM_OF_POMODORO")
      .takes_value(true)
      .required(true));
  let task_mod = SubCommand::with_name("mod").about("modify task")
    .arg(Arg::with_name("id")
      .long("id")
      .short("i")
      .value_name("ID")
      .takes_value(true)
      .required(true))
    .arg(Arg::with_name("lane")
      .long("lane")
      .short("l")
      .value_name("LANE_NAME")
      .takes_value(true)
      .required(false))
    .arg(Arg::with_name("priority")
      .long("priority")
      .short("p")
      .value_name("PRIORITY_NAME")
      .takes_value(true)
      .required(false))
    .arg(Arg::with_name("summary")
      .long("summary")
      .short("s")
      .value_name("TEXT")
      .takes_value(true)
      .required(false))
    .arg(Arg::with_name("estimate")
      .long("estimate")
      .short("e")
      .value_name("NUM_OF_POMODORO")
      .takes_value(true)
      .required(false));
  let task_rm = SubCommand::with_name("rm").about("remove task");
  let task = SubCommand::with_name("task")
      .about("manage task")
      .subcommand(task_list)
      .subcommand(task_add)
      .subcommand(task_mod)
      .subcommand(task_rm);
  let matches = App::new("ly")
      .version("1.0")
      .author("Yutaka Imamura")
      .about("Pomodoro time tracker")
      .subcommand(init)
      .subcommand(server)
      .subcommand(start)
      .subcommand(task)
      .get_matches();

  match matches.subcommand() {
    ("init", _) => {
      let mut session = sql::Session::connect()?;
      session.initialize()?;
      Ok(())
    },
    ("server", _) => Ok(web::start_server().await),
    ("start", Some(start_m)) => {
      let task_id = start_m.value_of("id").unwrap().parse::<i64>().expect("id should be integer");
      let duration_min = start_m.value_of("duration").unwrap_or("25").parse::<i64>().expect("duration should be integer");
      start_pomodoro(task_id, duration_min)
    },
    ("task", Some(task_m)) => match task_m.subcommand() {
        ("ls", task_ls_m) => {
          let mut session = sql::Session::connect()?;
          let lanes = core::lane::fetch_all_lanes(&mut session)?;
          let priorities = core::priority::fetch_all_priority(&mut session)?;
          let tasks = core::task::list_all_tasks(&mut session, task_ls_m.unwrap().value_of("lane").unwrap_or("backlog"))?;
          let task_list = TaskList::new(&lanes, &priorities, &tasks);
          println!("{}", task_list.output());
          Ok(())
        }
        ("add", Some(task_add_m)) => {
          let mut session = sql::Session::connect()?;
          core::task::add_task(
            &mut session,
            task_add_m.value_of("lane").unwrap_or("backlog"),
            task_add_m.value_of("priority").unwrap_or("n"),
            task_add_m.value_of("summary").unwrap(),
            task_add_m.value_of("estimate").unwrap().parse::<i64>().expect("estimate should be integer"),
          )?;
          Ok(())
        }
        ("mod", Some(task_mod_m)) => {
          let mut session = sql::Session::connect()?;
          core::task::mod_task(
            &mut session,
            task_mod_m.value_of("id").unwrap().parse::<i64>().expect("id should be integer"),
            task_mod_m.value_of("lane"),
            task_mod_m.value_of("priority"),
            task_mod_m.value_of("summary"),
            task_mod_m.value_of("estimate").map(|v| v.parse::<i64>().expect("estimate should be integer"))
          )?;
          Ok(())
        }
        // ("rm", _) => {
        //     rm_task().await.expect("rm task");
        // }
        _ => bail!("invalid options"),
    },
    _ => bail!("invalid options"),
  }
}
