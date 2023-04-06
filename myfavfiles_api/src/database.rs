use sqlx::{migrate::MigrateDatabase, PgPool, Pool, Postgres};

#[macro_use]
mod macros;
pub mod actions;
pub mod cache;
pub mod driver;
pub mod entities;
pub mod loaders;
pub mod password;
pub mod relation;
pub mod repository;

pub type DbPool = Pool<Postgres>;
pub type PoolConnection = sqlx::pool::PoolConnection<Postgres>;

pub const DATABASE_CONNECTION_ERROR_MESSAGE: &str = "Could not connect to the database.";
pub const DATABASE_MIGRATION_ERROR_MESSAGE: &str = "Could not apply database migrations.";
pub const DATABASE_CREATION_ERROR_MESSAGE: &str = "Could not create the database.";

pub fn connection_pool(database_url: &str) -> DbPool {
    PgPool::connect_lazy(database_url).expect(DATABASE_CONNECTION_ERROR_MESSAGE)
}

pub async fn initialize_database(database_url: &str) {
    create_database_if_not_exists(database_url).await;
    apply_migrations(database_url).await;
}

async fn create_database_if_not_exists(database_url: &str) {
    if !sqlx::Postgres::database_exists(database_url)
        .await
        .expect(DATABASE_CREATION_ERROR_MESSAGE)
    {
        sqlx::Postgres::create_database(database_url)
            .await
            .expect(DATABASE_CREATION_ERROR_MESSAGE);
    }
}

async fn apply_migrations(database_url: &str) {
    let pool = connection_pool(database_url);
    sqlx::migrate!("../migrations")
        .run(&pool)
        .await
        .expect(DATABASE_MIGRATION_ERROR_MESSAGE);
}
