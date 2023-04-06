use std::error::Error;

use axum::{body::Body, http::StatusCode, response::Response};
use juniper::{http::GraphQLResponse, DefaultScalarValue, FieldError, Value};

pub fn graphql_error_response(status: StatusCode, error: impl Error) -> Response<Body> {
    let graphql_resp: GraphQLResponse = GraphQLResponse::error(FieldError::new(
        status.to_string(),
        Value::Scalar(DefaultScalarValue::String(format!("{error}"))),
    ));
    let resp_body = Body::from(serde_json::to_string_pretty(&graphql_resp).unwrap());

    Response::builder()
        .status(StatusCode::OK)
        .body(resp_body)
        .unwrap()
}
