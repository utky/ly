extern crate clap;
use clap::{App, Arg, SubCommand};
use crate::cli::TaskList;

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
        .required(false));
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
        .subcommand(task)
        .get_matches();

    match matches.subcommand() {
        ("init", _) => {
            let mut session = sql::Session::connect().expect("connect database");
            session.initialize().expect("failed to initialize");
        }
        ("server", _) => http::start_server().await,
        ("task", Some(task_m)) => match task_m.subcommand() {
            ("ls", task_ls_m) => {
              let mut session = sql::Session::connect().expect("connect database");
              let lanes = core::lane::fetch_all_lanes(&mut session).expect("fetch all lanes");
              let priorities = core::priority::fetch_all_priority(&mut session).expect("fetch all priorities");
              let tasks = core::task::list_all_tasks( &mut session, task_ls_m.unwrap().value_of("lane").unwrap_or("backlog"),).expect("ls tasks");
              let task_list = TaskList::new(&lanes, &priorities, &tasks);
              println!("{}", task_list.output());
            }
            ("add", Some(task_add_m)) => {
              let mut session = sql::Session::connect().expect("connect database");
              core::task::add_task(
                &mut session,
                task_add_m.value_of("lane").unwrap_or("backlog"),
                task_add_m.value_of("priority").unwrap_or("n"),
                task_add_m.value_of("summary").unwrap(),
              ).expect("add test");
            }
            ("mod", Some(task_mod_m)) => {
              let mut session = sql::Session::connect().expect("connect database");
              core::task::mod_task(
                &mut session,
                task_mod_m.value_of("id").unwrap().parse::<i64>().expect("id should be integer"),
                task_mod_m.value_of("lane"),
                task_mod_m.value_of("priority"),
                task_mod_m.value_of("summary"),
              ).expect("mod test");
            }
            // ("rm", _) => {
            //     rm_task().await.expect("rm task");
            // }
            _ => panic!("invalid options"),
        },
        _ => panic!("invalid options"),
    }
}
