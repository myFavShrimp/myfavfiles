use axum::{Router, routing::get_service};
use myfavfiles_common as common;
use myfavfiles_frontend as frontend;

#[tokio::main]
async fn main() {
    let spa = frontend::create_frontend_router();
    let frontend_router = Router::new().merge(spa);

    let address = common::config::Config::default().address();

    axum::Server::bind(&address)
        .serve(frontend_router.into_make_service())
        .await
        .unwrap();
}
