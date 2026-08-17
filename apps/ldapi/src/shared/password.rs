use crate::shared::{AppError, AppResult};
use argon2::Argon2;
use argon2::password_hash::rand_core::OsRng;
use argon2::password_hash::{PasswordHash, PasswordHasher, PasswordVerifier, SaltString};
use sword::prelude::*;

#[injectable]
pub struct PassswordHashService;

impl PassswordHashService {
	pub async fn hash(&self, password: &str) -> AppResult<String> {
		let password = password.to_string();
		let salt = SaltString::generate(&mut OsRng);

		tokio::task::spawn_blocking(move || {
			Argon2::default()
				.hash_password(password.as_bytes(), &salt)
				.map(|hash| hash.to_string())
				.map_err(|err| {
					tracing::error!("Error while hashing password: {}", err);
					AppError::InternalError
				})
		})
		.await
		.map_err(|err| {
			tracing::error!("Password hashing task panicked: {}", err);
			AppError::InternalError
		})?
	}

	pub async fn verify(&self, password: &str, hash: &str) -> AppResult<bool> {
		let password = password.to_string();
		let hash = hash.to_string();

		tokio::task::spawn_blocking(move || {
			let Ok(parsed) = PasswordHash::new(&hash) else {
				return Ok(false);
			};

			Ok(Argon2::default()
				.verify_password(password.as_bytes(), &parsed)
				.is_ok())
		})
		.await
		.map_err(|err| {
			tracing::error!("Password verification task panicked: {}", err);
			AppError::InternalError
		})?
	}
}
