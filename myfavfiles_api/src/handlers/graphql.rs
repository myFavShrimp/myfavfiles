mod error;
mod private;
mod public;

use std::sync::Arc;

use async_graphql::http::{playground_source, GraphQLPlaygroundConfig};
use async_graphql_axum::{GraphQLRequest, GraphQLResponse};
use axum::{
    extract::State,
    headers::{self, authorization::Bearer},
    response::{Html, IntoResponse},
    TypedHeader,
};
use tokio::sync::Mutex;

use crate::{auth::AuthStatus, database::Caches, AppState};

pub use {private::create_root as create_private_root, private::Root as PrivateRoot};
pub use {public::create_root as create_public_root, public::Root as PublicRoot};

pub async fn playground() -> impl IntoResponse {
    Html(playground_source(GraphQLPlaygroundConfig::new(
        "/api/graphql",
    )))
}

pub async fn graphql(
    State(ref state): State<Arc<AppState>>,
    TypedHeader(_authorization): TypedHeader<headers::Authorization<Bearer>>,
    req: GraphQLRequest,
) -> GraphQLResponse {
    let database_connection = match state
        .database_connection()
        .await
        .map_err(error::graphql_error_response)
    {
        Ok(conn) => conn,
        Err(e) => return e,
    };

    let auth_status = AuthStatus::Missing;

    match auth_status {
        AuthStatus::Ok(auth_token) => {
            let context = private::Context {
                app_state: state.clone(),
                database_connection: Arc::new(Mutex::new(database_connection)),
                caches: Caches::default(),
                session_token: auth_token,
            };

            state
                .graphql_root_authenticated
                .execute(req.into_inner().data(context))
                .await
                .into()
            // juniper_hyper::graphql(state.graphql_root_authenticated.clone(), context, req).await
        }
        _unauthorised => {
            let context = public::Context {
                app_state: state.clone(),
                database_connection: Arc::new(Mutex::new(database_connection)),
            };

            state
                .graphql_root_unauthorised
                .execute(req.into_inner().data(context))
                .await
                .into()
            // juniper_hyper::graphql(state.graphql_root_unauthorised.clone(), context, req).await
        }
    }
}
