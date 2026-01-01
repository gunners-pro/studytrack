use axum::Router;
use tokio::net::TcpListener;

use crate::{config::Config, http::routes};

pub fn build_app() -> Router<()> {
    Router::new().merge(routes::router())
}

pub async fn run() {
    let config = Config::from_env();

    let app = build_app();

    let listener = TcpListener::bind(config.addr())
        .await
        .expect("Failed to bind");

    println!("🚀 Server running at http://{}", config.addr());

    axum::serve(listener, app).await.unwrap();
}
