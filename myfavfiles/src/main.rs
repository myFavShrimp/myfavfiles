use axum::handler::Handler;
use myfavfiles_api as api;
use myfavfiles_common as common;
use myfavfiles_frontend as frontend;

#[tokio::main]
async fn main() {
    api::database::initialize_database().await;

    let api_router = api::create_api_router().await;
    let frontend_router = frontend::create_frontend_router();
    let app_router = axum::Router::new()
        .merge(frontend_router)
        .nest("/api", api_router)
        .fallback(common::handler::handler_404.into_service());
    let address = common::config::Config::default().address();

    axum::Server::bind(&address)
        .serve(app_router.into_make_service())
        .await
        .unwrap();
}
