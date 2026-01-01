mod app;
mod config;
mod http;
mod models;
mod services;
mod state;

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();
    app::run().await;
}
