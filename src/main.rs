use axum::{Router, routing::get, serve};
use tokio::net::TcpListener;

#[tokio::main]
async fn main() {
    let app: Router<()> = Router::new().route("/", get(|| async { "Hello World!" }));

    let listener = TcpListener::bind("0.0.0.0:3000").await.unwrap();

    serve(listener, app).await.unwrap();
}
