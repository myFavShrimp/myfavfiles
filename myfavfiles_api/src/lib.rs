use axum::{
    routing::{get, post},
    Extension, Router,
};
use std::sync::Arc;

use myfavfiles_common as common;

#[macro_use]
pub mod database;
pub mod handlers;

pub struct State {
    database_connection: database::DbPool,
    config: common::config::Config,
    graphql_root: handlers::graphql::Root,
}

impl State {
    async fn new() -> Self {
        Self {
            database_connection: database::connection_pool().await,
            config: common::config::Config::default(),
            graphql_root: handlers::graphql::create_root(),
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

// Router::new()
// .route(
//     "/",
//     get_service(ServeDir::new("frontend"))
//         .handle_error(|_: std::io::Error| handlers::handler_500()),
// )
// .nest("/api", api_router)
