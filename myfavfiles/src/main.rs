use myfavfiles_api as api;
use myfavfiles_common as common;
use myfavfiles_frontend as frontend;

#[tokio::main]
async fn main() {
    api::database::initialize_database().await;

    let address = common::config::Config::default().address();
    let api_router = api::create_api_router().await;
    let app_router = axum::Router::new()
        .nest("/api", api_router)
        .fallback(axum::routing::get(frontend::fallback_frontend_handler));

    axum::Server::bind(&address)
        .serve(app_router.into_make_service())
        .await
        .unwrap();
}
