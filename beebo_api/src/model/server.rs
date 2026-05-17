use std::env;

pub struct HttpServer {
    pub port: String,
    pub api_key: String,
}

impl HttpServer {
    pub fn from_env() -> Self {
        Self {
            port: env::var("AXUM_API_PORT").expect("[ERROR] Missing port"),
            api_key: env::var("AXUM_API_KEY").expect("[ERROR] Missing api key"),
        }
    }
}
