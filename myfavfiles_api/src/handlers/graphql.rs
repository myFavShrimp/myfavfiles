mod error;
mod private;
mod public;

use std::sync::Arc;

use axum::{
    body::Body,
    extract::Extension,
    http::{Request, StatusCode},
    response::IntoResponse,
};
use tokio::sync::Mutex;

use crate::{auth::AuthStatus, database::Caches, AppState};

pub use {private::create_root as create_private_root, private::Root as PrivateRoot};
pub use {public::create_root as create_public_root, public::Root as PublicRoot};

pub async fn playground() -> impl IntoResponse {
    juniper_hyper::playground("/api/graphql", None).await
}

pub async fn graphql(
    Extension(ref state): Extension<AppState>,
    auth_status: AuthStatus,
    req: Request<Body>,
) -> impl IntoResponse {
    let database_connection = match state.database_connection().await {
        Ok(conn) => conn,
        Err(e) => return error::graphql_error_response(StatusCode::INTERNAL_SERVER_ERROR, e),
    };

    match auth_status {
        AuthStatus::Ok(auth_token) => {
            let context = Arc::new(private::Context {
                app_state: state.clone(),
                database_connection: Arc::new(Mutex::new(database_connection)),
                caches: Caches::default(),
                session_token: auth_token,
            });

            juniper_hyper::graphql(state.graphql_root_authenticated.clone(), context, req).await
        }
        _unauthorised => {
            let context = Arc::new(public::Context {
                app_state: state.clone(),
                database_connection: Arc::new(Mutex::new(database_connection)),
            });

            juniper_hyper::graphql(state.graphql_root_unauthorised.clone(), context, req).await
        }
    }
}
