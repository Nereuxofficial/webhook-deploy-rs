mod command;
mod config;

use crate::config::load_configs;
use actix_web::http::header::HeaderValue;
use actix_web::http::Uri;
use actix_web::{get, post, web, App, Error, HttpRequest, HttpResponse, HttpServer};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;
use std::env;
use tracing::info;

// TODO: If we process the first payload we can inform about new webhooks connected to the server
// TODO: Admin Panel to add new webhooks, remove old ones and restart them
// TODO: Admin Panel to display logs, and show the status of the scripts

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
    if req
        .headers()
        .get("X-GitHub-Event")
        .unwrap_or(&HeaderValue::from_str("").unwrap())
        != HeaderValue::from_str("push").unwrap()
    {
        return HttpResponse::Ok().body("Not a push event!");
    }
    // TODO: Handle unwraps
    // Get repository URL
    let body = String::from_utf8(bytes.to_vec()).unwrap();
    info!("Payload: {}", body);
    let json: Value = serde_json::from_str(&body).unwrap();
    let repo_url: String = json.get("html_url").unwrap().to_string();
    info!("Repo URL: {}", repo_url);
    let signature = req
        .headers()
        .get("X-Hub-Signature-256")
        .unwrap()
        .to_str()
        .unwrap();

    HttpResponse::Ok().body("Successfully restarted process!")
}

#[tokio::main]
async fn main() -> color_eyre::Result<()> {
    tracing_subscriber::fmt::init();
    color_eyre::install()?;
    dotenvy::dotenv().ok();
    // Webserver using actix-web
    info!("Starting webserver");
    HttpServer::new(|| {
        App::new()
            .app_data(web::Data::new(|| async move { load_configs().await }))
            .service(index)
            .service(payload)
    })
    .bind(env::var("BIND_IP").unwrap())?
    .run()
    .await
    .unwrap();
    Ok(())
}
