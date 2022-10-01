use std::error::Error;

use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde::Serialize;

use crate::auth::JWTPayload;

pub fn encode_token<T: Serialize>(secret: String, payload: &T) -> Result<String, Box<dyn Error>> {
    let result = encode(
        &Header::default(),
        &payload,
        &EncodingKey::from_secret(secret.as_ref()),
    );
    match result {
        Ok(result) => Ok(result),
        Err(e) => Err(e.into()),
    }
}

pub fn decode_token(secret: &str, token: &str) -> Result<JWTPayload, Box<dyn Error>> {
    match decode::<JWTPayload>(
        &token,
        &DecodingKey::from_secret(secret.as_ref()),
        &Validation::default(),
    ) {
        Ok(it) => Ok(it.claims),
        Err(err) => Err(err.into()),
    }
}
