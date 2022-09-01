use axum::response::IntoResponse;
use hyper::StatusCode;
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use myfavfiles_common::config::Config;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
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

#[cfg(test)]
mod tests {
    use chrono::Local;
    use uuid::Uuid;

    use super::Token;

    #[test]
    fn encode_decode() {
        let secret = "1234567890";
        let timestamp = Local::now().timestamp() + 1000;

        let token = Token {
            sub: Uuid::parse_str("a0c86a8a-4210-4487-bdd5-e0ecb5c31882").unwrap(),
            jti: Uuid::parse_str("a0c86a8a-4210-4487-bdd5-e0ecb5c31882").unwrap(),
            exp: timestamp as usize,
        };

        let encoded = token.encode(secret).unwrap();

        let decoded = Token::from_encoded(encoded.as_str(), secret).unwrap();

        assert_eq!(decoded, token);
    }
}
