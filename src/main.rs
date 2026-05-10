use std::sync::Arc;

use reqwest::Client;

use crate::model::{AppState, FsPath, LlamaModel};
use crate::{infra::run_server, model::HttpServer};

mod ai;
mod handler;
mod infra;
mod model;
mod services;

#[tokio::main]
async fn main() {
    let llama_config = Arc::new(LlamaModel::get_model());
    let axum_config = Arc::new(HttpServer::from_env());
    let system_config = Arc::new(FsPath::get_path());

    let http_client = Client::new();

    let state = AppState {
        llama: llama_config,
        client: http_client,
        fs: system_config,
    };

    run_server(axum_config, state).await;
}
