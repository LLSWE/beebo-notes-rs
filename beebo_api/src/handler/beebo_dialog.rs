use std::sync::Arc;

use axum::extract::State;
use reqwest::StatusCode;

use crate::model::AppState;
use crate::services::exec_beebo;

// pub async fn send_beebo(State(state): State<AppState>) -> Result<String, StatusCode> {
//     let resp = ask_beebo(&state.client, &state.llama.url, &state.llama.model)
//         .await
//         .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
//
//     Ok(resp)
// }

pub async fn write_beebo(State(state): State<Arc<AppState>>) -> Result<String, StatusCode> {
    exec_beebo(&state)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)
}
