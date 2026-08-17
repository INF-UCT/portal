use crate::users::UserId;
use sword::grpc::*;
use thiserror::Error;

#[derive(Debug, Error, GrpcError)]
pub enum UserError {
	#[grpc(code = "not_found", message = "Usuario '{id}' no encontrado")]
	#[error("User not found")]
	NotFound { id: UserId },
}
