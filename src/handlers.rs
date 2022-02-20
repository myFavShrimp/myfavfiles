use actix_web::{Error, HttpResponse};

pub async fn graphql() -> Result<HttpResponse, Error> {
    Ok(HttpResponse::Ok()
        .content_type("application/json")
        .body("{\"Hi\": \"GraphQL\"}"))
}

pub async fn handler_404() -> HttpResponse {
    HttpResponse::NotFound().body("NOT FOUND")
}
