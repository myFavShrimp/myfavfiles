use axum::{
    routing::{get, post},
    Extension, Router,
};
use myfavfiles_common::config::Config;
use tower::ServiceBuilder;
use std::sync::Arc;

#[macro_use]
pub mod database;
pub mod handlers;
pub mod auth;

pub struct State {
    config: Config,
    graphql_root_authenticated: handlers::graphql::authenticated::Root,
    graphql_root_unauthorised: handlers::graphql::unauthorised::Root,
}

type AppState = Arc<State>;

pub async fn create_api_router(config: Config) -> Router {
    let state = State {
        config: config,
        graphql_root_authenticated: handlers::graphql::authenticated::create_root(),
        graphql_root_unauthorised: handlers::graphql::unauthorised::create_root(),
    };

    let app_state: AppState = Arc::new(state);

    Router::new()
        .route("/graphql", post(handlers::graphql))
        .route("/playground", get(handlers::playground))
        .layer(
            ServiceBuilder::new()
                .layer(Extension(app_state))
        )
}
