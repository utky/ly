use std::fmt::Display;

use super::config;
use super::core::meter;
use super::core::meter::MeterQuery;
use super::core::timer;
use super::public;
use super::sql::Session;
use actix_web::{get, web, App, HttpResponse, HttpResponseBuilder, HttpServer, Responder};
use anyhow::{Error, Result};
use tokio::sync::Mutex;

struct State {
    session: Mutex<Session>,
}

#[derive(Debug)]
enum WebApiError {
    TimerNotFound,
    InternalError,
}

impl Display for WebApiError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            WebApiError::TimerNotFound => {
                f.write_str("Timer not found").unwrap();
            }
            WebApiError::InternalError => {
                f.write_str("Serious Problem").unwrap();
            }
        };
        Ok(())
    }
}

impl actix_web::error::ResponseError for WebApiError {
    fn status_code(&self) -> http::StatusCode {
        match *self {
            WebApiError::TimerNotFound => http::StatusCode::NOT_FOUND,
            WebApiError::InternalError => http::StatusCode::INTERNAL_SERVER_ERROR,
        }
    }

    fn error_response(&self) -> HttpResponse<actix_web::body::BoxBody> {
        let mut res = HttpResponseBuilder::new(self.status_code());
        match *self {
            WebApiError::TimerNotFound => res.append_header(("Content-Type", "text/plain")),
            WebApiError::InternalError => res.append_header(("Content-Type", "text/plain")),
        };
        let body = actix_web::body::BoxBody::new(format!("{}", self));
        res.body(body)
    }
}

#[get("/timer")]
async fn get_timer(data: web::Data<State>) -> impl Responder {
    let mut session = data.session.lock().await;
    match timer::get_current_timer(&mut *session) {
        Ok(Some(c)) => Ok(web::Json(c)),
        Ok(None) => Err(WebApiError::TimerNotFound),
        Err(_e) => Err(WebApiError::InternalError),
    }
}

#[get("/pomodoro_daily")]
async fn query_pomodoro_daily(
    data: web::Data<State>,
    range: web::Query<meter::TimeRange>,
) -> impl Responder {
    let mut session = data.session.lock().await;
    match session.query_pomodoro_daily(&range) {
        Ok(summaries) => Ok(web::Json(summaries)),
        Err(_e) => Err(WebApiError::InternalError),
    }
}

pub async fn start_server(conf: config::Config, address: String, port: u16) -> Result<()> {
    let session = crate::sql::Session::connect(&conf)?;

    let state = State {
        session: Mutex::new(session),
    };
    let data = web::Data::new(state);
    HttpServer::new(move || {
        App::new()
            .app_data(data.clone())
            .route(
                "/",
                web::get().to(|| async {
                    HttpResponse::Ok()
                        .append_header(("Content-Type", "text/html"))
                        .body(public::index_html())
                }),
            )
            .route(
                "/index.js",
                web::get().to(|| async {
                    HttpResponse::Ok()
                        .append_header(("Content-Type", "text/javascript"))
                        .body(public::index_js())
                }),
            )
            .route(
                "/alarm.mp3",
                web::get().to(|| async {
                    HttpResponse::Ok()
                        .append_header(("Content-Type", "audio/mpeg"))
                        .body(public::ALARM_MP3)
                }),
            )
            .service(
                web::scope("/api")
                    .service(get_timer)
                    .service(query_pomodoro_daily),
            )
    })
    .bind((address, port))?
    .run()
    .await
    .map_err(Error::new)
}
