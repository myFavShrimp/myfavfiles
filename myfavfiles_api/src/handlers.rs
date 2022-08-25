use std::sync::Arc;

use axum::{
    body::Body,
    extract::Extension,
    http::{Request, StatusCode},
    response::IntoResponse,
};
use tokio::sync::Mutex;

use crate::{auth::AuthStatus, database::loaders::Loaders, AppState};

use self::graphql::error;

pub mod graphql;

pub async fn graphql(
    Extension(ref state): Extension<AppState>,
    auth_status: AuthStatus,
    req: Request<Body>,
) -> impl IntoResponse {
    let database_connection = match state.database_connection().await {
        Ok(db_connection) => db_connection,
        Err(_) => return error::graphql_error_response(StatusCode::INTERNAL_SERVER_ERROR, None),
    };

    match auth_status {
        AuthStatus::Ok(auth_token) => {
            let context = Arc::new(graphql::authenticated::Context {
                app_state: state.clone(),
                database_connection: Arc::new(Mutex::new(database_connection)),
                loaders: Arc::new(Mutex::new(Loaders::default())),
                session_token: auth_token,
            });

            juniper_hyper::graphql(state.graphql_root_authenticated.clone(), context, req).await
        }
        _unauthorised => {
            let context = Arc::new(graphql::unauthorised::Context {
                app_state: state.clone(),
                database_connection: Arc::new(Mutex::new(database_connection)),
            });

            juniper_hyper::graphql(state.graphql_root_unauthorised.clone(), context, req).await
        }
    }
}

pub async fn playground() -> impl IntoResponse {
    juniper_hyper::playground("/api/graphql", None).await
}
