mod controller;
mod dtos;
mod errors;
mod service;

pub use dtos::*;
pub use errors::*;
pub use service::*;

use controller::AuthController;
use sword::prelude::*;

pub mod users {
	pub use crate::users::proto::*;
}

pub mod proto {
	tonic::include_proto!("auth");
	pub use auth_rpc_service_server::*;
}

pub struct AuthModule;

impl Module for AuthModule {
	fn register_controllers(controllers: &ControllerRegistry) {
		controllers.register::<AuthController>();
	}

	fn register_components(components: &ComponentRegistry) {
		components.register::<AuthService>();
	}
}
