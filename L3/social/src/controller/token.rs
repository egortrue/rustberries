use std::env;

use crate::errors::{ErrorKind, Result};
use chrono::{Duration, TimeDelta, Utc};
use jsonwebtoken::{DecodingKey, EncodingKey, Header, TokenData, Validation};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Token {
    pub exp: usize,
    pub username: String,
}

impl Token {
    pub fn encode(username: &String) -> Result<String> {
        let secret: String =
            env::var("JWT_ENCODING_STRING").expect("ENV variable not found: JWT_ENCODING_STRING");
        let expire_time =
            env::var("JWT_EXPIRE_TIME").expect("ENV variable not found: JWT_EXPIRE_TIME");

        let now = Utc::now();
        let expire: TimeDelta = Duration::seconds(expire_time.parse().unwrap());
        let claim = Token {
            exp: (now + expire).timestamp() as usize,
            username: username.clone(),
        };
        let encoder = EncodingKey::from_secret(secret.as_ref());

        match jsonwebtoken::encode(&Header::default(), &claim, &encoder) {
            Ok(token) => Ok(token),
            Err(error) => Err(ErrorKind::JwtError(error)),
        }
    }

    pub fn decode(token: String) -> Result<TokenData<Token>> {
        let secret: String =
            env::var("JWT_ENCODING_STRING").expect("ENV variable not found: JWT_ENCODING_STRING");
        let decoder = DecodingKey::from_secret(secret.as_ref());

        match jsonwebtoken::decode(&token, &decoder, &Validation::default()) {
            Ok(token) => Ok(token),
            Err(error) => match error.kind() {
                jsonwebtoken::errors::ErrorKind::InvalidSignature => {
                    Err(ErrorKind::JwtError(error))
                }
                _ => Err(ErrorKind::BadRequest(error.to_string())),
            },
        }
    }
}
