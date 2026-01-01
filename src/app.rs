use axum::Router;
use sqlx::PgPool;
use tokio::net::TcpListener;

use crate::{config::Config, http::routes, state::AppState};

pub fn build_app() -> Router<AppState> {
    Router::new().merge(routes::router())
}

pub async fn run() {
    let config = Config::from_env();
    let db = PgPool::connect(&config.database_url)
        .await
        .expect("Failed to connect to database");
    let state = AppState { db };

    let app = build_app().with_state(state);

    let listener = TcpListener::bind(config.addr())
        .await
        .expect("Failed to bind");

    println!("🚀 Server running at http://{}", config.addr());

    axum::serve(listener, app).await.unwrap();
}
