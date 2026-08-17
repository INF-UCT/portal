mod auth;
mod shared;
mod users;

use sword::Application;

#[tokio::main]
async fn main() {
	let application = Application::builder()
		.with_module::<auth::AuthModule>()
		.with_module::<users::UsersModule>()
		.with_module::<shared::SharedModule>()
		.build();

	application.run().await;
}
