use axum::handler::Handler;
use myfavfiles_api as api;
use myfavfiles_common as common;

#[tokio::main]
async fn main() {
    api::database::initialize_database().await;

    let app = api::create_api_router()
        .await
        .fallback(common::handler::handler_404.into_service());
    let address = common::config::Config::default().address();

    axum::Server::bind(&address)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
