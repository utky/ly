use warp::reply::html;
use warp::{Filter, path};
use warp::filters::path::{end};
use warp::http::{Response};
use super::public;

pub async fn start_server() {
    let routes =
      path!("index.js").map(|| {
        Response::builder().header("Content-Type", "text/javascript").body(public::index_js())
      })
      .or(end().map(|| html(public::index_html())));
    warp::serve(routes).run(([0, 0, 0, 0], 8081)).await;
}
