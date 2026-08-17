use argon2::Argon2;
use argon2::password_hash::rand_core::OsRng;
use argon2::password_hash::{PasswordHasher, SaltString};
use sqlx::{Pool, Postgres};
use std::{env, error::Error};
use uuid::Uuid;

fn expect_env(key: &str) -> String {
	env::var(key).unwrap_or_else(|_| panic!("Missing expected env var: {key}"))
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
	let db_url = {
		let user = expect_env("POSTGRES_USER");
		let password = expect_env("POSTGRES_PASSWORD");
		let port = expect_env("POSTGRES_PORT");

		format!("postgres://{user}:{password}@localhost:{port}/ldapi-db")
	};

	let admin_uid = expect_env("SEED_ADMIN_UID");
	let admin_name = expect_env("SEED_ADMIN_NAME");
	let admin_email = expect_env("SEED_ADMIN_EMAIL");
	let admin_password = expect_env("SEED_ADMIN_PASSWORD");

	println!("Connecting to: {db_url}");

	let db = Pool::connect(&db_url).await?;
	println!("Database connection success!");

	let salt = SaltString::generate(&mut OsRng);
	let pwd_hash = Argon2::default()
		.hash_password(admin_password.as_bytes(), &salt)
		.map_err(|err| format!("Failed to hash password: {err}"))?
		.to_string();

	sqlx::query::<Postgres>(
		"INSERT INTO users (id, uid, name, email, role, password_hash)
         VALUES ($1, $2, $3, $4, $5::user_role, $6)
         ON CONFLICT (uid) DO UPDATE
         SET name = EXCLUDED.name,
             email = EXCLUDED.email,
             password_hash = EXCLUDED.password_hash,
             role = EXCLUDED.role",
	)
	.bind(Uuid::new_v4())
	.bind(admin_uid)
	.bind(admin_name)
	.bind(admin_email)
	.bind("admin")
	.bind(pwd_hash)
	.execute(&db)
	.await?;

	println!("Admin user seeded successfully.");

	Ok(())
}
