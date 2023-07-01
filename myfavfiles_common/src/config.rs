use serde::Deserialize;
use serde_env::from_env;
use std::net::SocketAddr;
use uuid::Uuid;

#[derive(Deserialize)]
pub struct Config {
    host_name: String,
    port: u16,
    pub database_url: String,
    pub frontend_path: String,
    pub jwt_secret: String,
    pub force_session: Option<Uuid>,
    pub bcrypt_cost: u32,
    #[serde(default)]
    pub tracing_level: TracingLevel,
}

#[derive(Deserialize, Clone)]
#[serde(rename_all = "lowercase")]
pub enum TracingLevel {
    Error,
    Warn,
    Info,
    Debug,
    Trace,
}

impl Default for TracingLevel {
    fn default() -> Self {
        TracingLevel::Warn
    }
}

impl From<TracingLevel> for tracing::level_filters::LevelFilter {
    fn from(value: TracingLevel) -> Self {
        match value {
            TracingLevel::Error => tracing::level_filters::LevelFilter::ERROR,
            TracingLevel::Warn => tracing::level_filters::LevelFilter::WARN,
            TracingLevel::Info => tracing::level_filters::LevelFilter::INFO,
            TracingLevel::Debug => tracing::level_filters::LevelFilter::DEBUG,
            TracingLevel::Trace => tracing::level_filters::LevelFilter::TRACE,
        }
    }
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
        from_env::<Config>().expect("load config")
    }
}
