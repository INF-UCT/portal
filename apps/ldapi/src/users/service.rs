use crate::shared::AppResult;
use crate::users::*;

use std::sync::Arc;
use sword::prelude::*;

#[injectable]
pub struct UsersService {
	users: Arc<UsersRepository>,
}

impl UsersService {
	pub async fn find(&self, query: GetUsersQuery) -> AppResult<Vec<UserView>> {
		let filter = UserFilter {
			role: query.role,
			search: query.search,
		};

		self.users.list(filter).await
	}

	pub async fn find_by_id(&self, id: &UserId) -> AppResult<UserView> {
		let Some(user) = self.users.find_by_id(id).await? else {
			return Err(UserError::NotFound { id: *id })?;
		};

		Ok(user.into())
	}
}
