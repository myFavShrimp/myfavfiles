use axum::{
    handler::Handler,
    routing::{get, get_service, post},
    Extension, Router,
};
use std::sync::Arc;
use tower_http::services::ServeDir;

#[macro_use]
mod entities;
mod config;
mod database;
mod handlers;

pub struct State {
    database_connection: database::DbPool,
    config: config::Config,
    graphql_root: handlers::graphql::Root,
}

impl State {
    async fn new() -> Self {
        Self {
            database_connection: database::get_connection_pool().await,
            config: config::Config::default(),
            graphql_root: handlers::graphql::create_root(),
        }
    }
}

type AppState = Arc<State>;

async fn create_app() -> Router {
    let app_state: AppState = Arc::new(State::new().await);

    let api_router = Router::new()
        .route("/graphql", post(handlers::graphql))
        .route("/playground", get(handlers::playground))
        .layer(Extension(app_state));

    Router::new()
        .route(
            "/",
            get_service(ServeDir::new("frontend"))
                .handle_error(|_: std::io::Error| handlers::handler_500()),
        )
        .nest("/api", api_router)
        .fallback(handlers::handler_404.into_service())
}

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();
    database::initialize_database().await;

    let app = create_app().await;
    let address = config::Config::default().address();

    axum::Server::bind(&address)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
