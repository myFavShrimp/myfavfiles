use myfavfiles_common as common;
use myfavfiles_frontend as frontend;

#[tokio::main]
async fn main() {
    let address = common::config::Config::default().address();

    let frontend_router = frontend::create_frontend_router();

    axum::Server::bind(&address)
        .serve(frontend_router.into_make_service())
        .await
        .unwrap();
}
