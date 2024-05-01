use axum::{routing::get, Router};

use crate::{config::app_config::AppConfig, metrics::metrics::metrics_handler};

pub async fn start_http_listener(config: &AppConfig) {
    let app = Router::new()
        .route("/", get(|| async { "Hello, World!" }))
        .route("/metrics", get(|| async { metrics_handler() }));

    // run our app with hyper
    let socket = format!("{}:{}", config.app.http.host, config.app.http.port);
    println!("Launching web server at {}", socket);
    let listener = tokio::net::TcpListener::bind(socket).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}