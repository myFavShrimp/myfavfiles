use std::sync::Arc;

use axum::{body::Body, extract::Extension, http::Request, response::IntoResponse};
use tokio::sync::Mutex;

use crate::{database::{loaders::Loaders, self}, AppState, auth::AuthStatus};

pub mod graphql;

pub async fn graphql(
    Extension(ref state): Extension<AppState>,
    auth_status: AuthStatus,
    req: Request<Body>,
) -> impl IntoResponse {
    match auth_status {
        AuthStatus::Ok(_auth_token) => {
            let context = Arc::new(graphql::authenticated::Context {
                app_state: state.clone(),
                database_connection_pool: database::connection_pool(&state.config.database_url).await,
                loaders: Arc::new(Mutex::new(Loaders::default())),
            });

            juniper_hyper::graphql(state.graphql_root_authenticated.clone(), context, req).await
        },
        _unauthorised => {
            let context = Arc::new(graphql::unauthorised::Context {
                app_state: state.clone(),
                database_connection_pool: database::connection_pool(&state.config.database_url).await,
            });

            juniper_hyper::graphql(state.graphql_root_unauthorised.clone(), context, req).await
        },
    }
}

pub async fn playground() -> impl IntoResponse {
    juniper_hyper::playground("/api/graphql", None).await
}
