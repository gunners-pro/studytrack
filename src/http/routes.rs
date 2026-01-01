use axum::{Router, routing::get};

use crate::state::AppState;

pub fn router() -> Router<AppState> {
    Router::new().route("/health", get(health))
}

async fn health() -> &'static str {
    "ok"
}
