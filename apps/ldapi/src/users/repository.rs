use crate::shared::{AppResult, Database};
use crate::users::{User, UserFilter, UserId, UserView};

use sqlx::QueryBuilder;
use std::sync::Arc;
use sword::prelude::*;

#[injectable]
pub struct UsersRepository {
	database: Arc<Database>,
}

impl UsersRepository {
	pub async fn list(&self, filter: UserFilter) -> AppResult<Vec<UserView>> {
		let mut query = QueryBuilder::new(
			"SELECT id, uid, name, email, role, last_login_at FROM users WHERE 1=1",
		);

		if let Some(q) = filter.search {
			let pattern = format!("%{}%", q.trim());

			query
				.push(" AND (uid ILIKE ")
				.push_bind(pattern.clone())
				.push(" OR email ILIKE ")
				.push_bind(pattern.clone())
				.push(" OR name ILIKE ")
				.push_bind(pattern)
				.push(")");
		}

		if let Some(role) = filter.role {
			query.push(" AND role IN (");
			let mut separated = query.separated(", ");
			separated.push_bind(role);
			separated.push_unseparated(")");
		}

		query.push(" ORDER BY name ASC LIMIT 200");

		let users = query
			.build_query_as::<UserView>()
			.fetch_all(self.database.pool())
			.await?;

		Ok(users)
	}

	pub async fn find_by_uid(&self, uid: &str) -> AppResult<Option<User>> {
		let user = sqlx::query_as::<_, User>("SELECT * FROM users WHERE uid = $1")
			.bind(uid)
			.fetch_optional(self.database.pool())
			.await?;

		Ok(user)
	}

	pub async fn find_by_id(&self, id: &UserId) -> AppResult<Option<User>> {
		let user = sqlx::query_as::<_, User>("SELECT * FROM users WHERE id = $1")
			.bind(id)
			.fetch_optional(self.database.pool())
			.await?;

		Ok(user)
	}

	pub async fn create(&self, user: &User) -> AppResult<()> {
		sqlx::query(
			"INSERT INTO users (id, uid, name, email, role, password_hash)
             VALUES ($1, $2, $3, $4, $5, $6)
             ON CONFLICT (uid) DO UPDATE SET
                name = EXCLUDED.name,
                email = EXCLUDED.email,
                role = EXCLUDED.role,
                password_hash = EXCLUDED.password_hash",
		)
		.bind(user.id)
		.bind(&user.uid)
		.bind(&user.name)
		.bind(&user.email)
		.bind(user.role)
		.bind(&user.password_hash)
		.execute(self.database.pool())
		.await?;

		Ok(())
	}

	pub async fn update_password_hash(&self, id: &UserId, password_hash: &str) -> AppResult<()> {
		sqlx::query("UPDATE users SET password_hash = $1 WHERE id = $2")
			.bind(password_hash)
			.bind(id)
			.execute(self.database.pool())
			.await?;

		Ok(())
	}

	pub async fn touch_last_login(&self, id: &UserId) -> AppResult<()> {
		sqlx::query("UPDATE users SET last_login_at = now() WHERE id = $1")
			.bind(id)
			.execute(self.database.pool())
			.await?;

		Ok(())
	}
}
