use serde::Deserialize;
use std::net::SocketAddr;

#[derive(Deserialize)]
pub struct Config {
    host_name: String,
    port: u16,
    pub database_url: String,
    pub frontend_path: String,
    pub jwt_secret: String,
}

impl Config {
    pub fn address(&self) -> SocketAddr {
        format!("{}:{}", self.host_name, self.port)
            .parse()
            .expect("address")
    }
}

impl Default for Config {
    fn default() -> Self {
        dotenv::dotenv().ok();
        envy::from_env::<Config>().expect("load config")
    }
}
