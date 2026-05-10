use std::sync::Arc;

use crate::model::{FsPath, LlamaModel};

#[derive(Clone)]
pub struct AppState {
    pub llama: Arc<LlamaModel>,
    pub client: reqwest::Client,
    pub fs: Arc<FsPath>,
}
