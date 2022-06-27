#[macro_use]
extern crate log;
use crate::cli::TaskContext;
use crate::core::timer;
use crate::core::Id;
use anyhow::Result;
use chrono::{DateTime, FixedOffset, TimeZone, Utc};
use clap::{ArgEnum, Parser, Subcommand};
use std::convert::TryFrom;

mod cli;
mod config;
mod core;
mod public;
mod sql;
mod web;

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
struct Ly {
    #[clap(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    /// Initialize database on local file system
    Init {},
    /// Start ly server
    Server {
        /// Address to bind
        #[clap(short, long, default_value_t = String::from("0.0.0.0"))]
        address: String,
        /// Port number to listen
        #[clap(short, long)]
        port: u16,
    },
    /// Start pomodoro
    Start {
        /// Task ID
        #[clap(short, long)]
        id: i64,
        /// Pomodoro duration
        #[clap(short, long)]
        duration: i64,
    },
    Break {
        #[clap(arg_enum)]
        break_type: BreakType,
    },
    Task {
        #[clap(subcommand)]
        task_command: TaskCommand,
    },
    Todo {
        #[clap(subcommand)]
        todo_command: TodoCommand,
    },
}

#[derive(Subcommand)]
enum TaskCommand {
    Ls {
        #[clap(short, long, default_value_t = String::from("backlog"))]
        lane: String,
    },
    Add {
        #[clap(short, long)]
        summary: String,
        #[clap(short, long, default_value_t = String::from("backlog"))]
        lane: String,
        #[clap(short, long, default_value_t = String::from("n"))]
        priority: String,
        #[clap(short, long, default_value_t = 1)]
        estimate: i64,
    },
    Mod {
        #[clap(short, long)]
        id: i64,
        #[clap(short, long)]
        summary: Option<String>,
        #[clap(short, long)]
        lane: Option<String>,
        #[clap(short, long)]
        priority: Option<String>,
        #[clap(short, long)]
        estimate: Option<i64>,
    },
    Rm {
        #[clap(short, long)]
        id: i64,
    },
}

#[derive(Subcommand)]
enum TodoCommand {
    Ls {
        #[clap(short, long)]
        date: Option<String>,
    },
    Load {
        #[clap(short, long)]
        date: Option<String>,
    },
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ArgEnum)]
pub enum BreakType {
    Short,
    Long,
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

fn parse_line_as_task_ids<F>(input: F) -> Result<Vec<Id>>
where
    F: std::io::BufRead,
{
    let mut ids: Vec<Id> = Vec::new();
    for line in input.lines() {
        let l = line?;
        if l.starts_with('#') {
            debug!("skip line starts with hash: {:?}", l);
            continue;
        } else if let Some(id_str) = l.split('\t').next() {
            debug!("read as task_id {:?}", id_str);
            let id_int = id_str.parse::<i64>()?;
            ids.push(id_int);
        }
    }
    Ok(ids)
}

fn parse_or_today(timezone: &FixedOffset, input: Option<&str>) -> Result<DateTime<Utc>> {
    match input {
        Some(input) => {
            let mut dt = String::from(input);
            dt.push_str("T00:00:00");
            let parsed = timezone.datetime_from_str(&dt, "%Y-%m-%dT%H:%M:%S")?;
            Ok(parsed.with_timezone(&Utc))
        }
        None => Ok(core::todo::start_of_day_in_tz(Utc::now(), timezone).with_timezone(&Utc)),
    }
}

fn format_date(conf: &config::Config, date: DateTime<Utc>) -> String {
    date.with_timezone(&conf.timezone)
        .format("%Y-%m-%d")
        .to_string()
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

fn start_break(conf: &config::Config, break_type: BreakType) -> Result<()> {
    let (timer_type, duration_min) = match break_type {
        BreakType::Short => (core::timer::TimerType::ShortBreak, conf.short_break),
        BreakType::Long => (core::timer::TimerType::LongBreak, conf.long_break),
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
    let ly = Ly::parse();

    let conf = config::Config::from_env()?;
    match ly.command {
        Command::Init {} => {
            let mut session = sql::Session::connect(&conf)?;
            session.initialize()?;
            Ok(())
        }
        Command::Server { address, port } => web::start_server(conf, address, port).await,
        Command::Start { id, duration } => start_pomodoro(&conf, id, duration),
        Command::Break { break_type } => start_break(&conf, break_type),
        Command::Task { task_command } => match task_command {
            TaskCommand::Ls { lane } => {
                let mut session = sql::Session::connect(&conf)?;
                let tasks = core::task::list_all_tasks(&mut session, &lane)?;
                let lanes = core::lane::fetch_all_lanes(&mut session)?;
                let priorities = core::priority::fetch_all_priority(&mut session)?;
                let context = TaskContext::new(&lanes, &priorities);
                for t in tasks {
                    println!("{}", context.format(t));
                }
                Ok(())
            }
            TaskCommand::Add {
                summary,
                lane,
                priority,
                estimate,
            } => {
                let mut session = sql::Session::connect(&conf)?;
                core::task::add_task(&mut session, &lane, &priority, &summary, estimate)?;
                Ok(())
            }
            TaskCommand::Mod {
                id,
                summary,
                lane,
                priority,
                estimate,
            } => {
                let mut session = sql::Session::connect(&conf)?;
                core::task::mod_task(
                    &mut session,
                    id,
                    lane.as_deref(),
                    priority.as_deref(),
                    summary.as_deref(),
                    estimate,
                )?;
                Ok(())
            }
            TaskCommand::Rm { id } => {
                // TODO
                Ok(())
            }
        },
        Command::Todo { todo_command } => match todo_command {
            TodoCommand::Ls { date } => {
                let mut session = sql::Session::connect(&conf)?;
                let date = parse_or_today(&conf.timezone, date.as_deref())?;
                let tasks = core::todo::list_todo_tasks(&mut session, &date)?;

                let estimate = tasks.iter().fold(0, |s, t| s + t.estimate);
                let actual = tasks.iter().fold(0, |s, t| s + t.actual);
                let remaining = estimate - actual;
                println!(
                    "#date:{}\testimate:{}\tactual:{}\tremaining:{}",
                    format_date(&conf, date),
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
            TodoCommand::Load { date } => {
                let mut session = sql::Session::connect(&conf)?;
                let date = parse_or_today(&conf.timezone, date.as_deref())?;
                let stdin = std::io::stdin();
                let stdin = stdin.lock();
                let ids_to_load = parse_line_as_task_ids(stdin)?;
                let empty_removed = Vec::new();
                core::todo::clear_todo(&mut session, &date)?;
                core::todo::mod_todo(&mut session, &date, &ids_to_load, &empty_removed)?;
                println!("{}", format_date(&conf, date));
                Ok(())
            }
        },
    }
}

#[cfg(test)]
mod tests {
    use anyhow::Result;
    use chrono::{DateTime, FixedOffset, TimeZone, Utc};
    #[test]
    fn test_parse_or_today() -> Result<()> {
        let input = "2021-01-01";
        let jst = FixedOffset::east(9 * 3600);
        let parsed: DateTime<Utc> = super::parse_or_today(&jst, Some(input))?;
        let expected = Utc.datetime_from_str("2020-12-31 15:00:00", "%Y-%m-%d %H:%M:%S")?;
        assert_eq!(parsed, expected);
        Ok(())
    }
}
