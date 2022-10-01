use std::error::Error;

use argon2::{password_hash::SaltString, Argon2, PasswordHash, PasswordHasher, PasswordVerifier};

pub fn hash_password(password: &str) -> Result<String, Box<dyn Error>> {
    Ok(Argon2::default()
        .hash_password(
            password.as_bytes(),
            &SaltString::generate(rand::thread_rng()),
        )?
        .to_string())
}

pub fn verify_password(password: &str, password_hash: &str) -> bool {
    match PasswordHash::new(&password_hash) {
        Ok(it) => match Argon2::default().verify_password(password.as_bytes(), &it) {
            Ok(_) => true,
            Err(_) => false,
        },
        Err(_) => false,
    }
}
