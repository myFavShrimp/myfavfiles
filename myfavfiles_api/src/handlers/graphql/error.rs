use std::error::Error;

use async_graphql::BatchResponse;
use async_graphql_axum::GraphQLResponse;

pub fn graphql_error_response(error: impl Error) -> GraphQLResponse {
    let errors = vec![async_graphql::ServerError::new(error.to_string(), None)];
    let response = async_graphql::Response::from_errors(errors);
    GraphQLResponse(BatchResponse::Single(response))
}
