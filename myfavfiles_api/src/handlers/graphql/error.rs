use axum::{body::Body, http::StatusCode, response::Response};
use juniper::{http::GraphQLResponse, DefaultScalarValue, FieldError, Value};

pub fn graphql_error_response(
    status: StatusCode,
    additional_message: Option<&'static str>,
) -> Response<Body> {
    let graphql_resp: GraphQLResponse = GraphQLResponse::error(FieldError::new(
        status.to_string(),
        match additional_message {
            Some(msg) => Value::Scalar(DefaultScalarValue::String(msg.to_string())),
            None => Value::Null,
        },
    ));
    let resp_body = Body::from(serde_json::to_string_pretty(&graphql_resp).unwrap());

    Response::builder().status(status).body(resp_body).unwrap()
}
