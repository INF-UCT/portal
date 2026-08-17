use super::proto::*;
use crate::users::{GetUsersQuery, UserId, UsersService};

use std::str::FromStr;
use std::sync::Arc;
use sword::grpc::*;
use sword::prelude::*;

#[controller(kind = Controller::Grpc, service = UserRpcServiceServer)]
pub struct UsersController {
	users: Arc<UsersService>,
}

#[sword::grpc::async_trait]
impl UserRpcService for UsersController {
	async fn get_user(&self, req: Request<GetUserRequest>) -> GrpcResult<ProtoUser> {
		let body = req.into_inner();
		let user_id = UserId::from_str(&body.id)
			.map_err(|_| GrpcStatus::InvalidArgument().message("Invalid UUID"))?;

		let user = self.users.find_by_id(&user_id).await?;

		Ok(GrpcResponse::message(ProtoUser::from(user)))
	}

	async fn get_users(&self, req: Request<GetUsersRequest>) -> GrpcResult<GetUsersResponse> {
		let body = req.into_inner();
		let query = GetUsersQuery::try_from(body)
			.inspect_err(|e| tracing::error!("Invalid request body - {e}"))
			.map_err(|_| GrpcStatus::InvalidArgument().message("Invalid request schema"))?;

		let users = self
			.users
			.find(query)
			.await?
			.into_iter()
			.map(ProtoUser::from)
			.collect();

		Ok(GrpcResponse::message(GetUsersResponse { users }))
	}
}
