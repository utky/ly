extern crate clap;
use clap::{App, Arg, SubCommand};
use uuid;

mod core;
mod http;
mod sql;
mod public;


#[tokio::main]
async fn main() {
    let init = SubCommand::with_name("init").about("initialize database");
    let server = SubCommand::with_name("server").about("start server");
    let task_list = SubCommand::with_name("ls").about("list tasks");
    let task_add = SubCommand::with_name("add").about("add task");
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
            sql::initialize();
        }
        ("server", _) => http::start_server().await,
        ("task", Some(task_m)) => match task_m.subcommand() {
            ("ls", _) => {
              let mut session = sql::connect().expect("connect database");
              for r in core::task::list_all_tasks(&mut session).expect("ls tasks") {
                  println!("{:?}", r)
              }
            }
            ("add", _) => {
              let mut session = sql::connect().expect("connect database");
              let u = uuid::Uuid::new_v4();
              core::task::add_task(&mut session, &u, "backlog", "test").expect("add test");
            }
            // ("rm", _) => {
            //     rm_task().await.expect("rm task");
            // }
            _ => panic!("invalid options"),
        },
        _ => panic!("invalid options"),
    }
}
