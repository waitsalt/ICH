mod ai;
mod comment;
mod ich;
mod post;
mod user;
mod util;

use axum::Router;
use tower_http::{
    cors::{Any, CorsLayer},
    trace,
};

pub fn init() -> Router {
    let ai_router = ai::init();
    let ich_router = ich::init();
    let post_router = post::init();
    let user_router = user::init();
    let util_router = util::init();
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);
    Router::new()
        .nest("/api/ai", ai_router)
        .nest("/api/ich", ich_router)
        .nest("/api/post", post_router)
        .nest("/api/user", user_router)
        .nest("/api/util", util_router)
        .layer(trace::TraceLayer::new_for_http())
        .layer(cors)
}
