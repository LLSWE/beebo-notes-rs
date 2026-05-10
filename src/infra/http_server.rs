use std::sync::Arc;

use axum::Router;
use axum::routing::{get, post};
use reqwest::Client;

use crate::handler::{send_beebo, write_beebo};
use crate::model::{AppState, HttpServer};

pub async fn run_server(axum_config: Arc<HttpServer>, state: AppState) {
    let app = Router::new()
        .route("/", get(|| async { "SERVER IS RUNNING\n" }))
        .route("/beebo/send", get(write_beebo).with_state(state));

    let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{}", &axum_config.port))
        .await
        .expect("[ERROR] Failed to bind tcp port");

    println!(
        "Server is running on http://localhost:{}",
        &axum_config.port
    );

    axum::serve(listener, app)
        .await
        .expect("[ERROR] Failed to run server")
}
