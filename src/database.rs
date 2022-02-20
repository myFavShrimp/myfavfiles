use futures::executor::block_on;
use sea_orm::{ConnectOptions, Database, DatabaseConnection};

use crate::config::Config;

pub fn get_connection_pool() -> DatabaseConnection {
    let mut opt = ConnectOptions::new(Config::default().database_url);
    block_on(Database::connect(opt)).expect("DATABASE CONNECTION")
}
