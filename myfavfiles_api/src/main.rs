use myfavfiles_api as api;
use myfavfiles_common as common;

use tower_http::trace::TraceLayer;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();
    let config = common::config::Config::default();
    api::database::initialize_database(&config.database_url).await;

    let app = api::create_api_router(config)
        .await
        .fallback(common::handler::handler_404)
        .layer(TraceLayer::new_for_http());
    let address = common::config::Config::default().address();

    axum::Server::bind(&address)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
