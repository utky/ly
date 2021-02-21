use super::config;
use super::core::current;
use super::public;
use anyhow::Result;
use warp::filters::path::end;
use warp::http::Response;
use warp::reply::{html, json};
use warp::{path, Filter};

fn with_config(
    conf: config::Config,
) -> impl Filter<Extract = (config::Config,), Error = std::convert::Infallible> + Clone {
    warp::any().map(move || conf.clone())
}

async fn get_current(conf: config::Config) -> Result<impl warp::Reply, warp::Rejection> {
    let result: Result<Option<current::CurrentTask>> = {
        let session = crate::sql::Session::connect(&conf);
        session.and_then(|s| {
            let mut ms = s;
            current::get_current_task(&mut ms)
        })
    };
    match result {
        Ok(Some(c)) => Ok(json(&c)),
        Ok(None) => Err(warp::reject::not_found()),
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
        .or(path!("api" / "current")
            .and(warp::get())
            .and(with_config(conf.clone()))
            .and_then(get_current))
        .or(end().map(|| html(public::index_html())));
    warp::serve(routes).run(([0, 0, 0, 0], 8081)).await;
}
