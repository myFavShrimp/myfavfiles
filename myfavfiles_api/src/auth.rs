use std::convert::Infallible;

use self::token::Token;

use axum::{
    extract::FromRequestParts,
    http::{self, request::Parts},
};
use chrono::Local;
use myfavfiles_common::config::Config;

pub mod token;

#[derive(Debug)]
pub enum AuthStatus {
    Missing,
    Invalid,
    Ok(Token),
}

#[async_trait::async_trait]
impl<S> FromRequestParts<S> for AuthStatus
where
    S: Send + Sync,
{
    type Rejection = Infallible;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        if let Some(id) = Config::default().force_session {
            return Ok(Self::Ok(Token {
                sub: id,
                jti: id,
                exp: (Local::now().timestamp() * 2) as usize,
            }));
        }

        let token_maybe = parts
            .headers
            .get(http::header::AUTHORIZATION)
            .and_then(|val| val.to_str().ok());

        if let Some(token_str) = token_maybe {
            Token::try_from(token_str)
                .map(Self::Ok)
                .or(Ok(Self::Invalid))
        } else {
            Ok(Self::Missing)
        }
    }
}
