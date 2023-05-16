mod command;
mod config;

use actix_web::{get, post, web, App, Error, HttpRequest, HttpResponse, HttpServer};
use serde::{Deserialize, Serialize};

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
async fn payload(req: HttpRequest, bytes: web::Bytes) -> HttpResponse {
    // TODO: Fix unwraps
    let signature = req
        .headers()
        .get("X-Hub-Signature-256")
        .unwrap()
        .to_str()
        .unwrap();
    let body = String::from_utf8(bytes.to_vec()).unwrap();
    todo!("Parse body and validate signature by comparing against Config Files");
    HttpResponse::Ok().body("Successfully restarted process!")
}

#[tokio::main]
async fn main() -> color_eyre::Result<()> {
    tracing_subscriber::fmt::init();
    color_eyre::install()?;
    dotenvy::dotenv().ok();
    // Webserver using actix-web
    HttpServer::new(|| App::new().service(index))
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
        .unwrap();
    Ok(())
}
