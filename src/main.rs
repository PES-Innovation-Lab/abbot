use event_handler::event_handler::register_run_discord_client;
use http::http_server::start_http_listener;

use config::app_config::load_app_config;
use metrics::metrics::register_metrics;
use database::connection::init_client;

pub mod config;
pub mod event_handler;
pub mod http;
pub mod metrics;
pub mod database;
pub mod types;

#[tokio::main]
async fn main() {
    println!("Starting abbot server");
    let app_config = match load_app_config() {
        Ok(cfg) => cfg,
        Err(e) => { 
            eprintln!("Error while loading config: {}", e);
            return;
        }
    };

    register_metrics();
    if let Err(e) = init_client(&app_config).await {
        eprintln!("Failed to init DB client: {}", e);
        return;
    };

    let mut handles = vec![];
    handles.push(tokio::spawn(async move { start_http_listener(&app_config).await }));
    handles.push(tokio::spawn(async { register_run_discord_client().await }));
    
    futures::future::join_all(handles).await;
}