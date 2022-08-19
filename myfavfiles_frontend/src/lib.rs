use std::path::PathBuf;

use axum::{body::Body, http::Request, response::{IntoResponse}, Router};
use myfavfiles_common as common;
use tower::ServiceExt;

#[cfg(debug_assertions)]
use tower_http::services::{ServeDir, ServeFile};
#[cfg(debug_assertions)]
use axum::routing::get_service;

#[cfg(not(debug_assertions))]
use axum::{response::Html, routing::get};
#[cfg(not(debug_assertions))]
mod assets;

pub fn create_frontend_router() -> Router {
    let frontend_path = PathBuf::from(common::config::Config::default().frontend_path);
    let mut index_file_path = frontend_path.clone();
    index_file_path.push("index.html");

    Router::new().nest(
        "/",

        #[cfg(debug_assertions)]
        get_service(ServeDir::new(frontend_path).fallback(ServeFile::new(index_file_path)))
            .handle_error(common::handler::error_handler_500),

        #[cfg(not(debug_assertions))]
        get(serve)
    )
}

#[cfg(not(debug_assertions))]
async fn serve(req: Request<Body>) -> impl IntoResponse {
    let path = req.uri().path().trim_start_matches("/");

    assets::ASSETS.iter().for_each(|a| {dbg!(a.relative_path);});

    match assets::ASSETS.iter().position(|asset| asset.relative_path == path) {
        None => Html::from(assets::base::INDEX_HTML.contents_str.to_owned()),
        Some(index) => Html::from(assets::ASSETS[index].contents_str.to_owned()),
    }
}

pub async fn fallback_frontend_handler(req: Request<Body>) -> impl IntoResponse {
    create_frontend_router().oneshot(req).await
}
