use self::token::Token;

use std::convert::Infallible;

use async_trait::async_trait;
use axum::{extract::{RequestParts, FromRequest}, http};


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
        let token_maybe = req.headers()
            .get(http::header::AUTHORIZATION)
            .and_then(|val| val.to_str().ok());

        dbg!(&token_maybe);
        if let Some(token_str) = token_maybe {
            Token::try_from(token_str)
                .and_then(|token| Ok(Self::Ok(token)))
                .or(Ok(Self::Invalid))
        } else {
            Ok(Self::Missing)
        }
    }
}
