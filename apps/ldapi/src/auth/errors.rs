use ldap3::LdapError;
use sword::grpc::*;
use thiserror::Error;

#[derive(Debug, Error, GrpcError)]
pub enum AuthError {
	#[grpc(
		code = "unauthenticated",
		message = "Las credenciales de acceso son inválidas"
	)]
	#[error("Invalid credentials provided")]
	InvalidCredentials,

	#[grpc(code = "not_found", message = "Usuario no encontrado")]
	#[error("User not found")]
	UserNotFound,

	#[grpc(code = "internal", message = "Error interno del servidor")]
	#[error("LDAP authentication failed: {0}")]
	Ldap(#[from] LdapError),
}
