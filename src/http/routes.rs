use axum::{Router, routing::get};

use crate::{http::handlers, state::AppState};

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/health", get(health))
        .nest("/users", handlers::users::router())
}

async fn health() -> &'static str {
    "ok"
}
