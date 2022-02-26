use axum::{Json, http::StatusCode, response::IntoResponse, extract::Extension};
use crate::AppState;

pub async fn graphql(Extension(ref state): Extension<AppState>) -> Json<String> {
    Json::from("{\"Hi\": \"GraphQL\"}".to_owned())
}

pub async fn handler_404() -> impl IntoResponse {
    (StatusCode::NOT_FOUND, "NOT FOUND")
}

pub async fn handler_500() -> impl IntoResponse {
    (StatusCode::INTERNAL_SERVER_ERROR, "INTERNAL SERVER ERROR")
}
