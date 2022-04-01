use sqlx::{PgPool, Pool, Postgres};

use crate::config::Config;

pub type DbPool = Pool<Postgres>;

pub async fn get_connection_pool() -> DbPool {
    PgPool::connect(&Config::default().database_url).await.expect("DATABASE CONNECTION")
}
