use self::token::Token;

use std::convert::Infallible;

use async_trait::async_trait;
use axum::{
    extract::{FromRequest, RequestParts},
    http,
};

pub mod token;

#[derive(Debug)]
pub enum AuthStatus {
    Missing,
    Invalid,
    Ok(Token),
}

#[async_trait]
impl<B> FromRequest<B> for AuthStatus
where
    B: Send,
{
    type Rejection = Infallible;

    async fn from_request(req: &mut RequestParts<B>) -> Result<Self, Self::Rejection> {
        let token_maybe = req
            .headers()
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
