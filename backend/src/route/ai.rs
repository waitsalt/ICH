use axum::{Router, routing::post};

use crate::service;

pub fn init() -> Router {
    Router::new().route("/chat", post(service::ai::chat))
}
