use std::fmt::Display;

use super::config;
use super::core::meter;
use super::core::meter::MeterQuery;
use super::core::timer;
use super::public;
use actix_web::{get, web, App, HttpResponse, HttpResponseBuilder, HttpServer, Responder};
use anyhow::Result;

struct State {
    conf: config::Config,
}

#[derive(Debug)]
enum WebApiError {
    TimerNotFound,
    InternalError,
}

impl Display for WebApiError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            &WebApiError::TimerNotFound => {
                f.write_str("Timer not found").unwrap();
            }
            &WebApiError::InternalError => {
                f.write_str("Serious Problem").unwrap();
            }
        };
        Ok(())
    }
}

impl actix_web::error::ResponseError for WebApiError {
    fn status_code(&self) -> http::StatusCode {
        match self {
            &WebApiError::TimerNotFound => http::StatusCode::NOT_FOUND,
            &WebApiError::InternalError => http::StatusCode::INTERNAL_SERVER_ERROR,
        }
    }

    fn error_response(&self) -> HttpResponse<actix_web::body::BoxBody> {
        let mut res = HttpResponseBuilder::new(self.status_code());
        match self {
            &WebApiError::TimerNotFound => res.append_header(("Content-Type", "text/plain")),
            &WebApiError::InternalError => res.append_header(("Content-Type", "text/plain")),
        };
        let body = actix_web::body::BoxBody::new(format!("{}", self));
        res.body(body)
    }
}

#[get("/timer")]
async fn get_timer(data: web::Data<State>) -> impl Responder {
    let result: Result<Option<timer::Timer>> = {
        let session = crate::sql::Session::connect(&data.conf);
        session.and_then(|s| {
            let mut ms = s;
            timer::get_current_timer(&mut ms)
        })
    };
    match result {
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
    match crate::sql::Session::connect(&data.conf).and_then(|session| {
        let mut s = session;
        s.query_pomodoro_daily(&range)
    }) {
        Ok(summaries) => Ok(web::Json(summaries)),
        Err(_e) => Err(WebApiError::InternalError),
    }
}

pub async fn start_server(conf: config::Config, port: u16) -> std::io::Result<()> {
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(State { conf: conf.clone() }))
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
    .bind(("0.0.0.0", port))?
    .run()
    .await
}
