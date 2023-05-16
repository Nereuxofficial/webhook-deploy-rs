mod command;
mod config;

use crate::config::load_configs;
use actix_web::http::Uri;
use actix_web::{get, post, web, App, Error, HttpRequest, HttpResponse, HttpServer};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;

#[derive(Debug, Clone, Deserialize, Serialize)]
struct Application {
    name: String,
    script: String,
    secret: String,
}

#[get("/")]
async fn index() -> &'static str {
    "Hello world!"
}

/// Accept and validate the payload from github
#[post("/payload")]
async fn payload(
    req: HttpRequest,
    bytes: web::Bytes,
    data: web::Data<HashMap<Uri, Application>>,
) -> HttpResponse {
    // TODO: Fix unwraps
    let signature = req
        .headers()
        .get("X-Hub-Signature-256")
        .unwrap()
        .to_str()
        .unwrap();
    let body = String::from_utf8(bytes.to_vec()).unwrap();
    let json: Value = serde_json::from_str(&body).unwrap();
    HttpResponse::Ok().body("Successfully restarted process!")
}

#[tokio::main]
async fn main() -> color_eyre::Result<()> {
    tracing_subscriber::fmt::init();
    color_eyre::install()?;
    dotenvy::dotenv().ok();
    // Webserver using actix-web
    HttpServer::new(|| {
        App::new()
            .app_data(web::Data::new(|| async move { load_configs().await }))
            .service(index)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
    .unwrap();
    Ok(())
}
