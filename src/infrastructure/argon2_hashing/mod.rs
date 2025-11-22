use anyhow::{Ok, Result};
use argon2::{
    password_hash::{rand_core::OsRng, SaltString},
    Argon2, PasswordHash, PasswordHasher, PasswordVerifier,
};

pub fn hash(password: String) -> Result<String> {
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();
    let result = argon2
        .hash_password(password.as_bytes(), &salt)
        .map_err(|e| anyhow::anyhow!(e.to_string()))?;
    Ok(result.to_string())
}

pub fn verify(password: String, hashed_password: String) -> Result<bool> {
    let parsed_hash =
        PasswordHash::new(&hashed_password).map_err(|e| anyhow::anyhow!(e.to_string()))?;
    let argon2 = Argon2::default();
    let is_correct_password = argon2
        .verify_password(password.as_bytes(), &parsed_hash)
        .is_ok();
    Ok(is_correct_password)
}
