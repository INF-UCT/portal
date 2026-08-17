mod database;
mod errors;
mod id;
mod ldap;
mod password;

use database::DatabaseConfig;
use sword::prelude::*;

pub use database::Database;
pub use errors::*;
pub use id::{Entity, Id};
pub use ldap::{LdapClient, LdapConfig, ldap_escape};
pub use password::PassswordHashService;

pub struct SharedModule;

impl Module for SharedModule {
	fn register_components(components: &ComponentRegistry) {
		components.register::<PassswordHashService>()
	}

	async fn register_providers(config: &Config, providers: &ProviderRegistry) {
		let db_config = config.expect::<DatabaseConfig>();
		let database = Database::new(db_config).await;

		providers.register(database);

		let ldap_config = config.expect::<LdapConfig>();
		let ldap_client = LdapClient::new(ldap_config);

		providers.register(ldap_client);
	}
}
