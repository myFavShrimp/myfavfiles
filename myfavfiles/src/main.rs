use myfavfiles_api as api;
use myfavfiles_common as common;
use myfavfiles_frontend as frontend;

use tower_http::trace::TraceLayer;
use tracing_subscriber::filter::LevelFilter;

#[tokio::main]
async fn main() {
    tracing::subscriber::set_global_default(
        tracing_subscriber::FmtSubscriber::builder()
            .with_max_level(LevelFilter::TRACE)
            .finish(),
    )
    .expect("setting default subscriber failed");

    let config = common::config::Config::default();
    api::database::initialize_database(&config.database_url).await;

    tracing::info!("test");

    let address = common::config::Config::default().address();
    let api_router = api::create_api_router(config).await;
    let app_router = axum::Router::new()
        .nest("/api", api_router)
        .fallback(axum::routing::get(frontend::fallback_frontend_handler))
        .layer(TraceLayer::new_for_http());

    axum::Server::bind(&address)
        .serve(app_router.into_make_service())
        .await
        .unwrap();
}
