extern crate clap;
use clap::{App, Arg, SubCommand};
use crate::cli::Row;

mod core;
mod cli;
mod http;
mod sql;
mod public;


#[tokio::main]
async fn main() {
    let init = SubCommand::with_name("init").about("initialize database");
    let server = SubCommand::with_name("server").about("start server");
    let task_list = SubCommand::with_name("ls").about("list tasks")
      .arg(Arg::with_name("lane")
        .long("lane")
        .short("l")
        .value_name("LANE_NAME")
        .takes_value(true));
    let task_add = SubCommand::with_name("add").about("add task")
      .arg(Arg::with_name("lane")
        .long("lane")
        .short("l")
        .value_name("LANE_NAME")
        .takes_value(true)
        .required(true))
      .arg(Arg::with_name("summary")
        .long("summary")
        .short("s")
        .value_name("TEXT")
        .takes_value(true)
        .required(true));
    let task_rm = SubCommand::with_name("rm").about("remove task");
    let task = SubCommand::with_name("task")
        .about("manage task")
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
            match sql::initialize() {
              Err(error) => println!("Error: {}", error),
              Ok(_) => ()
            }
        }
        ("server", _) => http::start_server().await,
        ("task", Some(task_m)) => match task_m.subcommand() {
            ("ls", task_ls_m) => {
              let mut session = sql::connect().expect("connect database");
              for r in core::task::list_all_tasks(
                &mut session,
                task_ls_m.unwrap().value_of("lane").unwrap_or("backlog"),
              ).expect("ls tasks") {
                  println!("{}", r.as_row())
              }
            }
            ("add", task_add_m) => {
              let mut session = sql::connect().expect("connect database");
              core::task::add_task(
                &mut session,
                task_add_m.unwrap().value_of("lane").unwrap(),
                task_add_m.unwrap().value_of("summary").unwrap(),
              ).expect("add test");
            }
            // ("rm", _) => {
            //     rm_task().await.expect("rm task");
            // }
            _ => panic!("invalid options"),
        },
        _ => panic!("invalid options"),
    }
}
