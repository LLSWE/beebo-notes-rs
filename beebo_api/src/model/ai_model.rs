use serde::Deserialize;
use std::sync::Arc;

pub struct LlamaModel {
    pub model: String,
    pub url: String,
}

impl LlamaModel {
    pub fn get_model() -> Self {
        Self {
            model: std::env::var("LLAMA_MODEL").expect("[ERROR] Missing Llama Model"),
            url: std::env::var("LLAMA_URL").expect("[ERROR] Missing Llama Url"),
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct LlamaResponse {
    pub choices: Vec<Choice>,
}

#[derive(Debug, Deserialize)]
pub struct Choice {
    pub message: Message,
}

#[derive(Debug, Deserialize)]
pub struct Message {
    pub content: String,
}
