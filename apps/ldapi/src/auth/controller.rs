use super::proto::*;
use crate::auth::{AuthCredentialsDto, AuthService};

use std::sync::Arc;
use sword::grpc::*;
use sword::prelude::*;

#[controller(kind = Controller::Grpc, service = AuthRpcServiceServer)]
pub struct AuthController {
	auth_service: Arc<AuthService>,
}

#[sword::grpc::async_trait]
impl AuthRpcService for AuthController {
	async fn check_credentials(
		&self,
		req: Request<AuthCheckRequest>,
	) -> GrpcResult<AuthCheckResponse> {
		let body = req.into_inner();

		let dto = AuthCredentialsDto {
			username: body.uid,
			password: body.password,
		};

		let user = self.auth_service.authenticate(&dto).await?;

		Ok(GrpcResponse::message(AuthCheckResponse {
			user: Some(user.into()),
		}))
	}
}
