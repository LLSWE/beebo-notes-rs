use axum::extract::State;
use reqwest::StatusCode;

use crate::services::{send_git, write_beebo_notes};
use crate::{model::AppState, services::ask_beebo};

pub async fn send_beebo(State(state): State<AppState>) -> Result<String, StatusCode> {
    let resp = ask_beebo(&state.client, &state.llama.url, &state.llama.model)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(resp)
}

pub async fn write_beebo(State(state): State<AppState>) -> Result<String, StatusCode> {
    let resp = ask_beebo(&state.client, &state.llama.url, &state.llama.model)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    if let Err(e) = write_beebo_notes(&state.fs.fs_path, resp).await {
        eprint!("[ERROR] Failed to print beebos notes: {e}");
        return Err(StatusCode::INTERNAL_SERVER_ERROR);
    }

    if let Err(e) = send_git(&state.fs.fs_path).await {
        eprintln!("[ERROR] Failed to send messages to github: {e}");
        return Err(StatusCode::INTERNAL_SERVER_ERROR);
    }

    Ok("Beebos notes have been written & sent to github ✍️✍️✍️ ... \n".to_string())
}
