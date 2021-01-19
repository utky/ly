use anyhow::{Result};
use warp::reply::{html, json};
use warp::{Filter, path};
use warp::filters::path::{end};
use warp::http::{Response};
use rusqlite::Error;
use super::public;
use super::core::current;

async fn get_current() -> Result<impl warp::Reply, warp::Rejection> {
  let result: Result<current::CurrentTask> = {
    let session = crate::sql::Session::connect();
    session.and_then(|s| {
      let mut ms = s;
      current::get_current_task(&mut ms)
    })
  };
  match result {
    Ok(c) => Ok(json(&c)),
    Err(e) =>  {
      match e.downcast_ref::<&Error>() {
        Some(Error::QueryReturnedNoRows) => {
          println!("row not found");
          Err(warp::reject::not_found())
        },
        _ => {
          println!("something wrong happend on fetching current");
          Err(warp::reject())
        }
      }
    }
  }
}

pub async fn start_server() {
    let routes =
      path!("index.js").map(|| {
        Response::builder().header("Content-Type", "text/javascript").body(public::index_js())
      })
      .or(path!("api" / "current").and(warp::get()).and_then(get_current))
      .or(end().map(|| html(public::index_html())));
    warp::serve(routes).run(([0, 0, 0, 0], 8081)).await;
}
