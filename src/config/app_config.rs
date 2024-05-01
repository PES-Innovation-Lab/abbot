use figment::{providers::{Format, Yaml}, Figment};
use serde::Deserialize;


#[derive(Deserialize)]
pub struct Http {
    pub host: String,
    pub port: i32
}

#[derive(Deserialize)]
pub struct Collections {
    pub candidates: String,
    pub panels: String
}

#[derive(Deserialize)]
pub struct Database {
    pub atlas_host: String,
    pub certificate_path: String,
    pub db_name: String,
    pub collections: Collections
}

#[derive(Deserialize)]
pub struct App {
    pub http: Http,
    pub database: Database
}

#[derive(Deserialize)]
pub struct AppConfig {
    pub app: App
}

pub fn load_app_config() -> Result<AppConfig, figment::Error> {
    let result = Figment::new()
    .merge(Yaml::file("./config.yaml")).
    extract()?;
    Ok(result)
}