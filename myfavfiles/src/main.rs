use myfavfiles_api as api;
use myfavfiles_common as common;
use myfavfiles_frontend as frontend;

#[tokio::main]
async fn main() {
    let config = common::config::Config::default();
    api::database::initialize_database(&config.database_url).await;

    let address = common::config::Config::default().address();
    let api_router = api::create_api_router(config).await;
    let app_router = axum::Router::new()
        .nest("/api", api_router)
        .fallback(axum::routing::get(frontend::fallback_frontend_handler));

    axum::Server::bind(&address)
        .serve(app_router.into_make_service())
        .await
        .unwrap();
}
