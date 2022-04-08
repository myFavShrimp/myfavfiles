use axum::{http::{StatusCode, Request}, response::IntoResponse, extract::Extension, body::Body};

use crate::AppState;

pub mod graphql;


pub async fn graphql(Extension(ref state): Extension<AppState>, req: Request<Body>) -> impl IntoResponse {
    let context = std::sync::Arc::new(graphql::Context {
        app_state: state.clone(),
    });

    juniper_hyper::graphql(state.graphql_root.clone(), context, req).await
}

pub async fn playground() -> impl IntoResponse {
    juniper_hyper::playground("/api/graphql", None).await
}

pub async fn handler_404() -> impl IntoResponse {
    StatusCode::NOT_FOUND.into_response()
}

pub async fn handler_500() -> impl IntoResponse {
    StatusCode::INTERNAL_SERVER_ERROR.into_response()
}
