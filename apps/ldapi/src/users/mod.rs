mod controller;
mod dtos;
mod entity;
mod errors;
mod repository;
mod service;

use controller::UsersController;
use sword::prelude::*;

pub use dtos::*;
pub use entity::*;
pub use errors::*;
pub use repository::*;
pub use service::*;

pub mod proto {
	tonic::include_proto!("users");
	pub use user_rpc_service_server::*;
}

pub struct UsersModule;

impl Module for UsersModule {
	fn register_controllers(controllers: &ControllerRegistry) {
		controllers.register::<UsersController>();
	}

	fn register_components(components: &ComponentRegistry) {
		components.register::<UsersRepository>();
		components.register::<UsersService>();
	}
}
