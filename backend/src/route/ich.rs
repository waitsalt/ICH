use axum::{
    Router,
    routing::{delete, get, post, put},
};

use crate::service;

pub fn init() -> Router {
    Router::new()
        .route("/", post(service::ich::create))
        .route("/{ich_id}", get(service::ich::info))
        .route("/{ich_id}", delete(service::ich::delete))
        .route("/{ich_id}", put(service::ich::update))
        .route("/search", get(service::ich::search))
}
