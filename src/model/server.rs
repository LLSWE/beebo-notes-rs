use std::env;

pub struct HttpServer {
    pub port: String,
}

impl HttpServer {
    pub fn from_env() -> Self {
        Self {
            port: env::var("AXUM_API_PORT").expect("[ERROR] Missing port"),
        }
    }
}
