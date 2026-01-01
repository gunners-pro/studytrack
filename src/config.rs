use std::env;

pub struct Config {
    pub host: String,
    pub port: u16,
    pub database_url: String,
}

impl Config {
    pub fn from_env() -> Self {
        let host = env::var("APP_HOST").unwrap_or_else(|_| "0.0.0.0".into());
        let port = env::var("APP_PORT")
            .ok()
            .and_then(|p| p.parse().ok())
            .unwrap_or(3000);

        let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

        Self {
            host,
            port,
            database_url,
        }
    }

    pub fn addr(&self) -> String {
        format!("{}:{}", self.host, self.port)
    }
}
