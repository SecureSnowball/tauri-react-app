use std::error::Error;

use serde::{Deserialize, Serialize};
use sqlx::{Pool, Sqlite};

use crate::{
    config::Config,
    util::{
        common::unix_timestamp_into_future,
        hashing::{hash_password, verify_password},
        jwt::{decode_token, encode_token},
    },
};

#[derive(Debug, Deserialize)]
pub struct Register {
    pub name: String,
    pub email: String,
    pub password: String,
}

#[derive(Debug, Deserialize)]
pub struct Login {
    pub email: String,
    pub password: String,
}

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct User {
    pub id: i64,
    pub name: String,
    pub email: String,
    pub password: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct JWTPayload {
    pub id: i64,
    pub sub: String,
    pub exp: u64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AuthResponse {
    pub token: String,
    pub payload: JWTPayload,
}

pub async fn login(
    pool: &Pool<Sqlite>,
    config: &Config,
    payload: Login,
) -> Result<AuthResponse, Box<dyn Error>> {
    let user = sqlx::query_as!(User, "SELECT * FROM users WHERE email=?", payload.email)
        .fetch_one(pool)
        .await?;
    if verify_password(&payload.password, &user.password) {
        let payload = JWTPayload {
            id: user.id,
            sub: user.email.clone(),
            exp: unix_timestamp_into_future(7),
        };
        let token = encode_token(config.app_secret.clone(), &payload)?;
        return Ok(AuthResponse { token, payload });
    }
    Err("Login failed".into())
}

pub async fn register(
    pool: &Pool<Sqlite>,
    config: &Config,
    payload: Register,
) -> Result<AuthResponse, Box<dyn Error>> {
    let password = hash_password(&payload.password)?;
    let user_id = sqlx::query!(
        "INSERT INTO users (name, email, password) VALUES (?, ?, ?)",
        payload.name,
        payload.email,
        password,
    )
    .execute(pool)
    .await?
    .last_insert_rowid();

    let user = sqlx::query_as!(User, "SELECT * FROM users WHERE id=?", user_id)
        .fetch_one(pool)
        .await?;
    let payload = JWTPayload {
        id: user.id,
        sub: user.email.clone(),
        exp: unix_timestamp_into_future(7),
    };
    let token = encode_token(config.app_secret.clone(), &payload)?;
    Ok(AuthResponse { token, payload })
}

pub async fn me(config: &Config, token: String) -> Result<AuthResponse, Box<dyn Error>> {
    Ok(AuthResponse {
        payload: decode_token(&config.app_secret, &token)?,
        token,
    })
}
