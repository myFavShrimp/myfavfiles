use axum::{http::StatusCode, response::IntoResponse};

pub async fn handler_404() -> impl IntoResponse {
    StatusCode::NOT_FOUND.into_response()
}

pub async fn handler_500() -> impl IntoResponse {
    StatusCode::INTERNAL_SERVER_ERROR.into_response()
}
