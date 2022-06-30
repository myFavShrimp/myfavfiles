use axum::{
    routing::{get, post},
    Extension, Router,
};
use std::sync::Arc;

#[macro_use]
pub mod database;
pub mod handlers;

pub struct State {
    database_connection: database::DbPool,
    graphql_root_authenticated: handlers::graphql::authenticated::Root,
}

impl State {
    async fn new() -> Self {
        Self {
            database_connection: database::connection_pool().await,
            graphql_root_authenticated: handlers::graphql::authenticated::create_root(),
        }
    }
}

type AppState = Arc<State>;

pub async fn create_api_router() -> Router {
    let app_state: AppState = Arc::new(State::new().await);

    Router::new()
        .route("/graphql", post(handlers::graphql))
        .route("/playground", get(handlers::playground))
        .layer(Extension(app_state))
}
