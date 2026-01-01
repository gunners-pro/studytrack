mod app;
mod config;
mod http;

#[tokio::main]
async fn main() {
    app::run().await;
}
