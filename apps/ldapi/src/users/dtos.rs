use crate::users::{User, UserId, UserRole, proto::*};
use chrono::{DateTime, Utc};
use o2o::o2o as FromImpl;
use serde::Serialize;
use sqlx::FromRow;
use std::str::FromStr;

#[derive(Debug, Default, FromImpl)]
#[try_from_owned(GetUsersRequest, String)]
pub struct GetUsersQuery {
	#[from(search)]
	pub search: Option<String>,

	#[from(~.map(|r| UserRole::from_str(r.as_str())).transpose()?)]
	pub role: Option<UserRole>,
}

#[derive(Debug, FromRow, Serialize, FromImpl)]
#[from_owned(User)]
pub struct UserView {
	#[from(id)]
	pub id: UserId,

	#[from(uid)]
	pub uid: String,

	#[from(name)]
	pub name: String,

	#[from(email)]
	pub email: String,

	#[from(role)]
	pub role: UserRole,

	#[from(last_login_at)]
	pub last_login_at: Option<DateTime<Utc>>,
}

impl From<UserView> for ProtoUser {
	fn from(value: UserView) -> Self {
		ProtoUser {
			id: value.id.to_string(),
			uid: value.uid,
			name: value.name,
			email: value.email,
			role: value.role.to_string(),
			last_login_at: value.last_login_at.map(|t| t.to_rfc3339()),
		}
	}
}
