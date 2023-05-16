
#[tokio::main]
async fn main() -> color_eyre::Result<()>{
    tracing_subscriber::fmt::init();
    color_eyre::install()?;
    dotenvy::dotenv().ok();
    // Webserver using actix-web

    Ok(())
}