use crate::core::stats::Fetch;

use super::config;
use super::core::stats;
use super::core::timer;
use super::public;
use anyhow::Result;
use warp::filters::path::end;
use warp::http::Response;
use warp::reply::{html, json};
use warp::{path, query, Filter};

fn with_config(
    conf: config::Config,
) -> impl Filter<Extract = (config::Config,), Error = std::convert::Infallible> + Clone {
    warp::any().map(move || conf.clone())
}

async fn get_timer(conf: config::Config) -> Result<impl warp::Reply, warp::Rejection> {
    let result: Result<Option<timer::Timer>> = {
        let session = crate::sql::Session::connect(&conf);
        session.and_then(|s| {
            let mut ms = s;
            timer::get_current_timer(&mut ms)
        })
    };
    match result {
        Ok(Some(c)) => Ok(json(&c)),
        Ok(None) => Err(warp::reject::not_found()),
        Err(_e) => Err(warp::reject()),
    }
}

async fn fetch_daily_summary(
    conf: config::Config,
    range: stats::SummaryRange,
) -> Result<impl warp::Reply, warp::Rejection> {
    match crate::sql::Session::connect(&conf).and_then(|session| {
        let mut s = session;
        s.fetch_daily_summary(&range)
    }) {
        Ok(summaries) => Ok(json(&summaries)),
        Err(_e) => Err(warp::reject()),
    }
}

pub async fn start_server(conf: config::Config) {
    let routes = path!("index.js")
        .map(|| {
            Response::builder()
                .header("Content-Type", "text/javascript")
                .body(public::index_js())
        })
        .or(path!("alarm.mp3").map(|| {
            Response::builder()
                .header("Content-Type", "audio/mpeg")
                .body(public::ALARM_MP3)
        }))
        .or(path!("api" / "timer")
            .and(warp::get())
            .and(with_config(conf.clone()))
            .and_then(get_timer))
        .or(path!("api" / "daily_stats")
            .and(warp::get())
            .and(with_config(conf.clone()))
            .and(query::query())
            .and_then(fetch_daily_summary))
        .or(end().map(|| html(public::index_html())));
    warp::serve(routes).run(([0, 0, 0, 0], 8081)).await;
}
