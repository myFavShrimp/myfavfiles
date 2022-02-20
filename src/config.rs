use serde_derive::Deserialize;

#[derive(Deserialize)]
pub struct Config {
    host: String,
    port: u16,
}

impl Config {
    pub fn address(&self) -> String {
        format!("{}:{}", self.host, self.port)
    }
}

impl Default for Config {
    fn default() -> Self {
        envy::from_env::<Config>().expect("load config")
    }
}
