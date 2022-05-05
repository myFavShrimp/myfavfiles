use sqlx::{migrate::MigrateDatabase, PgPool, Pool, Postgres};

use crate::config::Config;

#[macro_use]
pub mod entities;
pub mod loaders;

pub type DbPool = Pool<Postgres>;

pub const DATABASE_CONNECTION_ERROR_MESSAGE: &str = "Could not connect to the database.";
pub const DATABASE_MIGRATION_ERROR_MESSAGE: &str = "Could not apply database migrations.";
pub const DATABASE_CREATION_ERROR_MESSAGE: &str = "Could not create the database.";

pub async fn get_connection_pool() -> DbPool {
    PgPool::connect(&Config::default().database_url)
        .await
        .expect(DATABASE_CONNECTION_ERROR_MESSAGE)
}

pub async fn initialize_database() {
    create_database_if_not_exists().await;
    apply_migrations().await;
}

async fn create_database_if_not_exists() {
    let database_url = &Config::default().database_url;
    if !sqlx::Postgres::database_exists(database_url)
        .await
        .expect(DATABASE_CREATION_ERROR_MESSAGE)
    {
        sqlx::Postgres::create_database(database_url)
            .await
            .expect(DATABASE_CREATION_ERROR_MESSAGE);
    }
}

async fn apply_migrations() {
    let pool = get_connection_pool().await;
    sqlx::migrate!()
        .run(&pool)
        .await
        .expect(DATABASE_MIGRATION_ERROR_MESSAGE);
}
