use warp::reply::html;
use warp::Filter;
use super::public;

pub async fn start_server() {
    let routes = warp::any().map(|| html(public::index_html()));
    warp::serve(routes).run(([0, 0, 0, 0], 8081)).await;
}
