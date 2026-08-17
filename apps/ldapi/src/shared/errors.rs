use crate::{auth::AuthError, users::UserError};
use sqlx::Error as SqlxError;
use sword::grpc::*;
use thiserror::Error;

pub type AppResult<T> = Result<T, AppError>;

#[derive(Debug, Error, GrpcError)]
pub enum AppError {
	#[grpc(transparent)]
	#[error(transparent)]
	Auth(#[from] AuthError),

	#[grpc(transparent)]
	#[error(transparent)]
	User(#[from] UserError),

	#[grpc(code = "internal")]
	#[tracing(error)]
	#[error("Database error: {0}")]
	Database(#[from] SqlxError),

	#[grpc(code = "internal", message = "Internal Server Error")]
	#[error("Internal Error")]
	InternalError,
}
