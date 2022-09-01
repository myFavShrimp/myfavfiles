use axum::{
    routing::{get, post},
    Extension, Router,
};
use myfavfiles_common::config::Config;
use std::sync::Arc;
use tower::ServiceBuilder;

#[macro_use]
pub mod database;
pub mod auth;
pub mod handlers;

pub struct State {
    config: Config,
    database_connection_pool: database::DbPool,
    graphql_root_authenticated: handlers::graphql::authenticated::Root,
    graphql_root_unauthorised: handlers::graphql::unauthorised::Root,
}

impl State {
    pub async fn database_connection(
        &self,
    ) -> Result<sqlx::pool::PoolConnection<sqlx::Postgres>, sqlx::Error> {
        self.database_connection_pool.acquire().await
    }
}

type AppState = Arc<State>;

pub async fn create_api_router(config: Config) -> Router {
    let database_url = config.database_url.clone();
    let state = State {
        config,
        database_connection_pool: database::connection_pool(&database_url),
        graphql_root_authenticated: handlers::graphql::authenticated::create_root(),
        graphql_root_unauthorised: handlers::graphql::unauthorised::create_root(),
    };

    let app_state: AppState = Arc::new(state);

    Router::new()
        .route("/graphql", post(handlers::graphql))
        .route("/playground", get(handlers::playground))
        .layer(ServiceBuilder::new().layer(Extension(app_state)))
}
