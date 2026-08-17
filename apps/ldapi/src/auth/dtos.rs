use crate::users::UserRole;

pub struct AuthCredentialsDto {
	pub username: String,
	pub password: String,
}

#[derive(Debug)]
pub struct LdapUserInfo {
	pub email: String,
	pub name: String,
	pub role: UserRole,
}
