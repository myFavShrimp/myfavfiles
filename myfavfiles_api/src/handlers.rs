use std::sync::Arc;

use axum::{
    body::Body,
    extract::Extension,
    http::{Request, StatusCode},
    response::IntoResponse,
};
use tokio::sync::Mutex;

use crate::{database::loaders::Loaders, AppState};

pub mod graphql;

pub async fn graphql(
    Extension(ref state): Extension<AppState>,
    req: Request<Body>,
) -> impl IntoResponse {
    let context = std::sync::Arc::new(graphql::Context {
        app_state: state.clone(),
        loaders: Arc::new(Mutex::new(Loaders::default())),
    });

    juniper_hyper::graphql(state.graphql_root.clone(), context, req).await
}

pub async fn playground() -> impl IntoResponse {
    juniper_hyper::playground("/api/graphql", None).await
}