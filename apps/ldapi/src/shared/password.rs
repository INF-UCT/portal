use crate::shared::{AppError, AppResult};
use argon2::Argon2;
use argon2::password_hash::rand_core::OsRng;
use argon2::password_hash::{PasswordHash, PasswordHasher, PasswordVerifier, SaltString};
use sword::prelude::*;

#[injectable]
pub struct PassswordHashService;

impl PassswordHashService {
	pub fn hash(&self, password: &str) -> AppResult<String> {
		let salt = SaltString::generate(&mut OsRng);

		Argon2::default()
			.hash_password(password.as_bytes(), &salt)
			.map(|hash| hash.to_string())
			.inspect_err(|e| tracing::error!("Error while hashing password: {}", e))
			.map_err(|_| AppError::InternalError)
	}

	pub fn verify(&self, password: &str, hash: &str) -> bool {
		let Ok(parsed) = PasswordHash::new(hash) else {
			return false;
		};

		Argon2::default()
			.verify_password(password.as_bytes(), &parsed)
			.is_ok()
	}
}
