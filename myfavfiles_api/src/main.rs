use myfavfiles_api as api;
use myfavfiles_common as common;

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
