use std::path::PathBuf;

use axum::{
    body::Body,
    http::Request,
    response::IntoResponse,
    routing::get_service,
    Router,
};
use myfavfiles_common as common;
use tower::ServiceExt;
use tower_http::services::{ServeDir, ServeFile};

pub fn create_frontend_router() -> Router {
    let frontend_path = PathBuf::from(common::config::Config::default().frontend_path);
    let mut index_file_path = frontend_path.clone();
    index_file_path.push("index.html");

    Router::new().nest(
        "/",
        get_service(ServeDir::new(frontend_path).fallback(ServeFile::new(index_file_path)))
            .handle_error(common::handler::error_handler_500),
    )
}

pub async fn fallback_frontend_handler(req: Request<Body>) -> impl IntoResponse {
    create_frontend_router().oneshot(req).await
}
