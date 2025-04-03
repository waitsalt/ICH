use axum::{
    Router,
    routing::{delete, get, post, put},
};

use crate::service;

pub fn init() -> Router {
    Router::new()
        .route("/", post(service::post::create))
        .route("/{post_id}", get(service::post::info))
        .route("/{post_id}", delete(service::post::delete))
        .route("/{post_id}", put(service::post::update))
        .route("/search", get(service::post::search))
}
