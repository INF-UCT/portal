use crate::shared::{Entity, Id};

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, Type};
use std::{fmt::Display, str::FromStr};

#[derive(Debug, Clone, Serialize, Deserialize, Type, Copy, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
#[sqlx(type_name = "user_role", rename_all = "lowercase")]
pub enum UserRole {
	Student,
	Func,
	Admin,
}

pub type UserId = Id<User>;

#[derive(Debug, Clone, FromRow)]
pub struct User {
	pub id: UserId,
	pub uid: String,
	pub name: String,
	pub email: String,
	pub role: UserRole,
	pub password_hash: String,
	pub last_login_at: Option<DateTime<Utc>>,
}

#[derive(Debug)]
pub struct UserFilter {
	pub search: Option<String>,
	pub role: Option<UserRole>,
}

impl Entity for User {
	fn key_name() -> &'static str {
		"user"
	}
}

impl FromStr for UserRole {
	type Err = String;

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		match s {
			"admin" => Ok(Self::Admin),
			"func" => Ok(Self::Func),
			"student" => Ok(Self::Student),
			_ => Err("Invalid role value".into()),
		}
	}
}

impl Display for UserRole {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		let value = match self {
			Self::Admin => "admin",
			Self::Func => "func",
			Self::Student => "student",
		};

		write!(f, "{value}")
	}
}
