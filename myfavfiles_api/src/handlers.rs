use std::sync::Arc;

use axum::{body::Body, extract::Extension, http::Request, response::IntoResponse};
use tokio::sync::Mutex;

use crate::{database::loaders::Loaders, AppState};

pub mod graphql;

pub async fn graphql(
    Extension(ref state): Extension<AppState>,
    req: Request<Body>,
) -> impl IntoResponse {
    let context = std::sync::Arc::new(graphql::authenticated::Context {
        app_state: state.clone(),
        loaders: Arc::new(Mutex::new(Loaders::default())),
    });

    juniper_hyper::graphql(state.graphql_root_authenticated.clone(), context, req).await
}

pub async fn playground() -> impl IntoResponse {
    juniper_hyper::playground("/api/graphql", None).await
}
