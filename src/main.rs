use actix_web::{App, HttpServer};
#[get("/")]
async fn index() -> &'static str {
    "Hello world!"
}

#[tokio::main]
async fn main() -> color_eyre::Result<()>{
    tracing_subscriber::fmt::init();
    color_eyre::install()?;
    dotenvy::dotenv().ok();
    // Webserver using actix-web
    HttpServer::new(|| {
        App::new()
            .service(index)
    })
        .bind(("127.0.0.1", 8080))?
        .run()
        .await;
    Ok(())
}