use jsonwebtoken::{decode, TokenData, Validation, DecodingKey};
use std::env;
use serde::{Serialize, Deserialize};
use crate::prelude::*;

#[derive(Debug, Serialize, Deserialize)]
pub struct Payload {
    pub user_id: u32,
    pub expires_at: i64,
}

pub trait CanGenerateJwt {
    fn generate_jwt(&self) -> Result<String>;
}

pub trait CanDecodeJwt {
    fn decode_jwt(&self) -> Result<TokenData<Payload>>;
}

impl CanDecodeJwt for String {
    fn decode_jwt(&self) -> Result<TokenData<Payload>> {
        let secret = get_secret();
        let key = DecodingKey::from_secret(secret.as_ref());

        match decode::<Payload>(&self, &key, &Validation::default()) {
            Ok(res) => Ok(res),
            Err(e) => Err(e.into()),
        }
    }
}

pub fn get_secret() -> String {
    env::var("JWT_SECRET").unwrap_or_else(|_| "secret".into())
}
