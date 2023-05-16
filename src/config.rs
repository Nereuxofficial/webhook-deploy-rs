use crate::Application;
use actix_web::dev::Url;
use actix_web::http::Uri;
use std::collections::HashMap;
use std::str::FromStr;
use tokio::fs;

async fn load_configs() -> HashMap<Uri, Application> {
    let mut files = fs::read_dir("configs").await.unwrap();
    let mut configs = HashMap::new();
    while let Some(file) = files.next_entry().await.unwrap() {
        let contents = fs::read_to_string(file.path()).await.unwrap();
        let config: Application = toml::from_str(&contents).unwrap();
        configs.insert(Uri::from_str(&config.name).unwrap(), config);
    }
    configs
}

#[cfg(test)]
mod tests {
    use super::*;
    #[tokio::test]
    async fn test_load_configs() {
        let configs = load_configs().await;
        assert_eq!(configs.len(), 1);
    }
}
