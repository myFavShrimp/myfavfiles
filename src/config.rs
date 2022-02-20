use serde_derive::Deserialize;

#[derive(Deserialize)]
pub struct Config {
    host_name: String,
    port: u16,
    pub database_url: String,
}

impl Config {
    pub fn address(&self) -> String {
        format!("{}:{}", self.host_name, self.port)
    }
}

impl Default for Config {
    fn default() -> Self {
        envy::from_env::<Config>().expect("load config")
    }
}
