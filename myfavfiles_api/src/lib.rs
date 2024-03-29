use axum::{routing::get, Router};
use myfavfiles_common::config::Config;
use std::sync::Arc;

#[macro_use]
pub mod database;
pub mod auth;
pub mod handlers;

pub struct AppState {
    config: Config,
    database_connection_pool: database::DbPool,
    graphql_root_authenticated: handlers::graphql::PrivateRoot,
    graphql_root_unauthorised: handlers::graphql::PublicRoot,
}

impl AppState {
    pub async fn database_connection(
        &self,
    ) -> Result<sqlx::pool::PoolConnection<sqlx::Postgres>, sqlx::Error> {
        self.database_connection_pool.acquire().await
    }
}

pub async fn create_api_router(config: Config) -> Router {
    let database_url = config.database_url.clone();
    let state = AppState {
        config,
        database_connection_pool: database::connection_pool(&database_url),
        graphql_root_authenticated: handlers::graphql::create_private_root(),
        graphql_root_unauthorised: handlers::graphql::create_public_root(),
    };

    let app_state = Arc::new(state);

    Router::new()
        .route(
            "/graphql",
            get(handlers::graphql::playground).post(handlers::graphql::graphql),
        )
        .with_state(app_state)
}
