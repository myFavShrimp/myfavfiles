use sea_orm::{ConnectOptions, Database, DatabaseConnection};

use crate::config::Config;

pub async fn get_connection_pool() -> DatabaseConnection {
    let opt = ConnectOptions::new(Config::default().database_url);
    Database::connect(opt).await.expect("DATABASE CONNECTION")
}
