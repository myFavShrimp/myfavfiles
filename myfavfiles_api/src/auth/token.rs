use axum::response::IntoResponse;
use hyper::StatusCode;
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use myfavfiles_common::config::Config;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct Token {
    pub sub: Uuid,
    pub jti: Uuid,
    pub exp: usize,
}

impl Token {
    pub fn encode(&self, secret: &str) -> Result<String, jsonwebtoken::errors::Error> {
        encode(
            &Header::default(),
            self,
            &EncodingKey::from_secret(secret.as_bytes()),
        )
    }

    pub fn from_encoded(
        encoded_token: &str,
        secret: &str,
    ) -> Result<Self, jsonwebtoken::errors::Error> {
        decode::<Self>(
            encoded_token,
            &DecodingKey::from_secret(secret.as_bytes()),
            &Validation::new(Header::default().alg),
        )
        .map(|tok| tok.claims)
    }
}

impl TryFrom<&str> for Token {
    type Error = jsonwebtoken::errors::Error;

    fn try_from(encoded: &str) -> Result<Self, Self::Error> {
        Token::from_encoded(encoded, &Config::default().jwt_secret)
    }
}

impl IntoResponse for Token {
    fn into_response(self) -> axum::response::Response {
        (
            StatusCode::OK,
            self.encode(&Config::default().jwt_secret).unwrap(),
        )
            .into_response()
    }
}
