use std::convert::Infallible;

use async_trait::async_trait;
use axum::{extract::{FromRequest, RequestParts}, http, response::IntoResponse};
use hyper::StatusCode;
use jsonwebtoken::{encode, Header, EncodingKey, decode, DecodingKey, Validation};
use myfavfiles_common::config::Config;
use serde::{Serialize, Deserialize};
use uuid::Uuid;

#[derive(Debug)]
pub enum AuthTokenStatus {
    Missing,
    Invalid,
    Correct(Token),
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct Token {
    pub sub: Uuid,
    pub jti: Uuid,
    pub exp: usize,
}

impl Token {
    pub fn encode(&self, secret: &str) -> Result<String, jsonwebtoken::errors::Error> {
        encode(&Header::default(), self, &EncodingKey::from_secret(secret.as_bytes()))
    }

    pub fn from_encoded(encoded_token: &str, secret: &str) -> Result<Self, jsonwebtoken::errors::Error> {
        decode::<Self>(encoded_token, &DecodingKey::from_secret(secret.as_bytes()), &Validation::new(Header::default().alg))
            .and_then(|tok| Ok(tok.claims))
    }
}

impl TryFrom<&str> for Token {
    type Error = jsonwebtoken::errors::Error;

    fn try_from(encoded: &str) -> Result<Self, Self::Error> {
        Token::from_encoded(&encoded, &Config::default().jwt_secret)
    }
}

impl IntoResponse for Token {
    fn into_response(self) -> axum::response::Response {
        (StatusCode::OK, self.encode(&Config::default().jwt_secret).unwrap().to_string()).into_response()
    }
}

#[async_trait]
impl<B> FromRequest<B> for AuthTokenStatus
where
    B: Send,
{
    type Rejection = Infallible;

    async fn from_request(req: &mut RequestParts<B>) -> Result<Self, Self::Rejection> {
        let token_maybe = req.headers()
            .get(http::header::AUTHORIZATION)
            .and_then(|val| val.to_str().ok());

        dbg!(&token_maybe);
        if let Some(token_str) = token_maybe {
            Token::try_from(token_str)
                .and_then(|token| Ok(self::AuthTokenStatus::Correct(token)))
                .or(Ok(self::AuthTokenStatus::Invalid))
        } else {
            Ok(self::AuthTokenStatus::Missing)
        }
    }
}
