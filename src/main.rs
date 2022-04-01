use std::sync::Arc;
use tower_http::services::ServeDir;
use axum::{
    Router, handler::Handler, Extension,
    routing::{get, get_service}
};

mod config;
mod handlers;
mod database;

pub struct State {
    database_connection: database::DbPool,
    config: config::Config,
}

impl State {
    async fn new() -> Self {
        Self {
            database_connection: database::get_connection_pool().await,
            config: config::Config::default(),
        }
    }
}

type AppState = Arc<State>;

async fn create_app() -> Router {
    let app_state: AppState = Arc::new(State::new().await);

    Router::new()
        .route("/graphql", get(handlers::graphql))
        .route("/", get_service(ServeDir::new("frontend")).handle_error(|_: std::io::Error| handlers::handler_500()))
        .fallback(handlers::handler_404.into_service())
        .layer(Extension(app_state))
}

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();

    let app = create_app().await;
    let address = config::Config::default().address();

    axum::Server::bind(&address)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
