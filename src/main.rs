#[macro_use]
extern crate log;
extern crate clap;
use crate::cli::TaskContext;
use crate::core::timer;
use crate::core::Id;
use anyhow::{bail, Result};
use chrono::{DateTime, TimeZone, Utc};
use clap::{arg_enum, value_t, App, Arg, SubCommand};
use std::convert::TryFrom;

mod cli;
mod config;
mod core;
mod public;
mod sql;
mod web;

const VERSION: &str = env!("CARGO_PKG_VERSION");

arg_enum! {
    #[derive(Debug)]
    pub enum Break {
        Short,
        Long,
    }
}

struct CleanupCurrent {
    session: sql::Session,
    timer: timer::Timer,
}

impl Drop for CleanupCurrent {
    fn drop(&mut self) {
        let _ = crate::core::timer::complete(&mut self.session, &self.timer);
        debug!("completed timer started_at: {}", &self.timer.started_at);
    }
}

fn parse_or_today(conf: &config::Config, input: Option<&str>) -> Result<DateTime<Utc>> {
    match input {
        Some(input) => {
            let parsed = conf.timezone.datetime_from_str(input, "%Y-%m-%d")?;
            Ok(parsed.with_timezone(&Utc))
        }
        None => Ok(core::todo::start_of_day_in_tz(Utc::now(), &conf.timezone).with_timezone(&Utc)),
    }
}

fn start_pomodoro(conf: &config::Config, task_id: Id, duration_min: i64) -> Result<()> {
    let timer: Result<timer::Timer> = {
        let mut session = sql::Session::connect(conf)?;
        crate::core::timer::pomodoro(&mut session, task_id, duration_min)
    };
    let session = sql::Session::connect(conf)?;
    let mut _cleanup = CleanupCurrent {
        session,
        timer: timer?,
    };
    let duration_sec: u64 = u64::try_from(duration_min * 60).expect("failed to cast i64 to u64");
    debug!("starting to sleep: {}", duration_sec);
    std::thread::sleep(std::time::Duration::from_secs(duration_sec));
    debug!("sleep finished");
    Ok(())
}

fn start_break(conf: &config::Config, break_type: Break) -> Result<()> {
    let (timer_type, duration_min) = match break_type {
        Break::Short => (core::timer::TimerType::ShortBreak, conf.short_break),
        Break::Long => (core::timer::TimerType::LongBreak, conf.long_break),
    };
    let timer: Result<timer::Timer> = {
        let mut session = sql::Session::connect(conf)?;
        crate::core::timer::take_break(&mut session, &timer_type, duration_min)
    };
    let session = sql::Session::connect(conf)?;
    let mut _cleanup = CleanupCurrent {
        session,
        timer: timer?,
    };
    let duration_sec: u64 = u64::try_from(duration_min * 60).expect("failed to cast i64 to u64");
    std::thread::sleep(std::time::Duration::from_secs(duration_sec));
    Ok(())
}

#[tokio::main]
async fn main() -> Result<()> {
    env_logger::init();
    let init = SubCommand::with_name("init").about("initialize database");
    let server = SubCommand::with_name("server").about("start server");
    let start = SubCommand::with_name("start")
        .about("start pomodoro for the task")
        .arg(
            Arg::with_name("id")
                .long("id")
                .short("i")
                .value_name("ID")
                .takes_value(true)
                .required(true),
        )
        .arg(
            Arg::with_name("duration")
                .long("duration")
                .short("d")
                .value_name("MINUTES")
                .takes_value(true)
                .required(false),
        );
    let take_break = SubCommand::with_name("break").about("take a break").arg(
        Arg::with_name("type")
            .long("type")
            .short("t")
            .value_name("TYPE")
            .takes_value(true)
            .required(false),
    );
    let task_list = SubCommand::with_name("ls").about("list tasks").arg(
        Arg::with_name("lane")
            .long("lane")
            .short("l")
            .value_name("LANE_NAME")
            .takes_value(true)
            .required(false),
    );
    let task_add = SubCommand::with_name("add")
        .about("add task")
        .arg(
            Arg::with_name("summary")
                .long("summary")
                .short("s")
                .value_name("TEXT")
                .takes_value(true)
                .required(true),
        )
        .arg(
            Arg::with_name("lane")
                .long("lane")
                .short("l")
                .value_name("LANE_NAME")
                .takes_value(true)
                .required(false),
        )
        .arg(
            Arg::with_name("priority")
                .long("priority")
                .short("p")
                .value_name("PRIORITY_NAME")
                .takes_value(true)
                .required(false),
        )
        .arg(
            Arg::with_name("estimate")
                .long("estimate")
                .short("e")
                .value_name("NUM_OF_POMODORO")
                .takes_value(true)
                .required(true),
        );
    let task_mod = SubCommand::with_name("mod")
        .about("modify task")
        .arg(
            Arg::with_name("id")
                .long("id")
                .short("i")
                .value_name("ID")
                .takes_value(true)
                .required(true),
        )
        .arg(
            Arg::with_name("lane")
                .long("lane")
                .short("l")
                .value_name("LANE_NAME")
                .takes_value(true)
                .required(false),
        )
        .arg(
            Arg::with_name("priority")
                .long("priority")
                .short("p")
                .value_name("PRIORITY_NAME")
                .takes_value(true)
                .required(false),
        )
        .arg(
            Arg::with_name("summary")
                .long("summary")
                .short("s")
                .value_name("TEXT")
                .takes_value(true)
                .required(false),
        )
        .arg(
            Arg::with_name("estimate")
                .long("estimate")
                .short("e")
                .value_name("NUM_OF_POMODORO")
                .takes_value(true)
                .required(false),
        );
    let task_rm = SubCommand::with_name("rm").about("remove task");
    let task = SubCommand::with_name("task")
        .about("task related")
        .alias("t")
        .subcommand(task_list)
        .subcommand(task_add)
        .subcommand(task_mod)
        .subcommand(task_rm);
    let todo_list = SubCommand::with_name("ls").about("list todo tasks").arg(
        Arg::with_name("date")
            .long("date")
            .short("d")
            .value_name("YYYY-MM-DD")
            .takes_value(true)
            .required(false),
    );
    let todo_mod = SubCommand::with_name("mod")
        .about("modify todo")
        .arg(
            Arg::with_name("date")
                .long("date")
                .short("d")
                .value_name("YYYY-MM-DD")
                .takes_value(true)
                .required(false),
        )
        .arg(
            Arg::with_name("add")
                .long("add")
                .short("a")
                .value_name("TASK_ID")
                .takes_value(true)
                .multiple(true)
                .number_of_values(1),
        )
        .arg(
            Arg::with_name("remove")
                .long("rm")
                .short("r")
                .value_name("TASK_ID")
                .takes_value(true)
                .multiple(true)
                .number_of_values(1),
        );
    let todo = SubCommand::with_name("todo")
        .about("todo related operations")
        .subcommand(todo_list)
        .subcommand(todo_mod);
    let matches = App::new("ly")
        .version(VERSION)
        .author("Yutaka Imamura")
        .about("Pomodoro time tracker")
        .subcommand(init)
        .subcommand(server)
        .subcommand(start)
        .subcommand(take_break)
        .subcommand(task)
        .subcommand(todo)
        .get_matches();

    let conf = config::Config::from_env()?;
    match matches.subcommand() {
        ("init", _) => {
            let mut session = sql::Session::connect(&conf)?;
            session.initialize()?;
            Ok(())
        }
        ("server", _) => {
            web::start_server(conf).await;
            Ok(())
        }
        ("start", Some(start_m)) => {
            let task_id = start_m
                .value_of("id")
                .unwrap()
                .parse::<i64>()
                .expect("id should be integer");
            let duration_min = start_m
                .value_of("duration")
                .unwrap_or("25")
                .parse::<i64>()
                .expect("duration should be integer");
            start_pomodoro(&conf, task_id, duration_min)
        }
        ("break", Some(break_m)) => {
            let break_type = value_t!(break_m, "type", Break).unwrap_or_else(|e| e.exit());
            start_break(&conf, break_type)
        }
        ("task", Some(task_m)) => match task_m.subcommand() {
            ("ls", task_ls_m) => {
                let mut session = sql::Session::connect(&conf)?;
                let tasks = core::task::list_all_tasks(
                    &mut session,
                    task_ls_m.unwrap().value_of("lane").unwrap_or("backlog"),
                )?;
                let lanes = core::lane::fetch_all_lanes(&mut session)?;
                let priorities = core::priority::fetch_all_priority(&mut session)?;
                let context = TaskContext::new(&lanes, &priorities);
                for t in tasks {
                    println!("{}", context.format(t));
                }
                Ok(())
            }
            ("add", Some(task_add_m)) => {
                let mut session = sql::Session::connect(&conf)?;
                core::task::add_task(
                    &mut session,
                    task_add_m.value_of("lane").unwrap_or("backlog"),
                    task_add_m.value_of("priority").unwrap_or("n"),
                    task_add_m.value_of("summary").unwrap(),
                    task_add_m
                        .value_of("estimate")
                        .unwrap()
                        .parse::<i64>()
                        .expect("estimate should be integer"),
                )?;
                Ok(())
            }
            ("mod", Some(task_mod_m)) => {
                let mut session = sql::Session::connect(&conf)?;
                core::task::mod_task(
                    &mut session,
                    task_mod_m
                        .value_of("id")
                        .unwrap()
                        .parse::<i64>()
                        .expect("id should be integer"),
                    task_mod_m.value_of("lane"),
                    task_mod_m.value_of("priority"),
                    task_mod_m.value_of("summary"),
                    task_mod_m
                        .value_of("estimate")
                        .map(|v| v.parse::<i64>().expect("estimate should be integer")),
                )?;
                Ok(())
            }
            // ("rm", _) => {
            //     rm_task().await.expect("rm task");
            // }
            _ => bail!("invalid options"),
        },
        ("todo", Some(todo_m)) => match todo_m.subcommand() {
            ("ls", Some(todo_ls_m)) => {
                let mut session = sql::Session::connect(&conf)?;
                let date = parse_or_today(&conf, todo_ls_m.value_of("date"))?;
                let tasks = core::todo::list_todo_tasks(&mut session, &date)?;

                let estimate = tasks.iter().fold(0, |s, t| s + t.estimate);
                let actual = tasks.iter().fold(0, |s, t| s + t.actual);
                let remaining = estimate - actual;
                println!(
                    "date:{}\testimate:{}\tactual:{}\tremaining:{}",
                    date.with_timezone(&conf.timezone).format("%Y-%m-%d"),
                    estimate,
                    actual,
                    remaining
                );

                let lanes = core::lane::fetch_all_lanes(&mut session)?;
                let priorities = core::priority::fetch_all_priority(&mut session)?;
                let context = TaskContext::new(&lanes, &priorities);
                for t in tasks {
                    println!("{}", context.format(t));
                }
                Ok(())
            }
            ("mod", Some(plan_mod_m)) => {
                let mut session = sql::Session::connect(&conf)?;
                let date = parse_or_today(&conf, plan_mod_m.value_of("date"))?;
                let added: Vec<Id> = plan_mod_m
                    .values_of("add")
                    .unwrap_or_default()
                    .map(|s| s.parse::<i64>().expect("estimate should be integer"))
                    .collect();
                let removed: Vec<Id> = plan_mod_m
                    .values_of("remove")
                    .unwrap_or_default()
                    .map(|s| s.parse::<i64>().expect("estimate should be integer"))
                    .collect();
                core::todo::mod_todo(&mut session, &date, &added, &removed)?;
                println!("{}", date.with_timezone(&conf.timezone).format("%Y-%m-%d"));
                Ok(())
            }
            _ => bail!("invalid options"),
        },
        _ => bail!("invalid options"),
    }
}
