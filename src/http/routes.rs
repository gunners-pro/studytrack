use axum::{Router, routing::get};

pub fn router() -> Router<()> {
    Router::new().route("/", get(health))
}

async fn health() -> &'static str {
    "Hello world"
}
