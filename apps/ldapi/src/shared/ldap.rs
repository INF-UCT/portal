use crate::{auth::AuthError, shared::AppResult};
use ldap3::{Ldap, LdapConnAsync as LdapConn, LdapConnSettings};
use serde::Deserialize;
use std::time::Duration;
use sword::prelude::*;

#[config(key = "ldap")]
#[derive(Clone, Deserialize)]
pub struct LdapConfig {
	pub url: String,
	pub admin_user: String,
	pub admin_password: String,
	pub base_dn: String,
}

#[injectable(provider)]
pub struct LdapClient {
	pub config: LdapConfig,
}

/// Escapes special characters in an LDAP filter value (RFC 4515).
pub fn ldap_escape(input: &str) -> String {
	let mut out = String::with_capacity(input.len());

	for c in input.chars() {
		match c {
			'\\' => out.push_str("\\5c"),
			'*' => out.push_str("\\2a"),
			'(' => out.push_str("\\28"),
			')' => out.push_str("\\29"),
			'\0' => out.push_str("\\00"),
			'/' => out.push_str("\\2f"),
			_ => out.push(c),
		}
	}

	out
}

impl LdapClient {
	pub fn new(config: LdapConfig) -> Self {
		LdapClient { config }
	}

	pub async fn connect(&self) -> AppResult<Ldap> {
		let settings = LdapConnSettings::new()
			.set_conn_timeout(Duration::from_secs(5))
			.set_no_tls_verify(true);

		let (conn, ldap) = LdapConn::with_settings(settings, &self.config.url)
			.await
			.inspect_err(|e| {
				tracing::error!("[!] Error de conexión LDAP: {e}");
			})
			.map_err(AuthError::from)?;

		ldap3::drive!(conn);

		Ok(ldap)
	}

	pub async fn admin_connect(&self) -> AppResult<Ldap> {
		let mut ldap = self.connect().await?;
		let admin_dn = format!("{},{}", self.config.admin_user, self.config.base_dn);

		ldap.simple_bind(&admin_dn, &self.config.admin_password)
			.await
			.inspect_err(|e| {
				tracing::error!("[!] Error de conexión durante bind admin: {e}");
			})
			.map_err(AuthError::from)?
			.success()
			.inspect_err(|e| {
				tracing::error!("[!] Error de autenticación como admin LDAP: {e}");
			})
			.map_err(AuthError::from)?;

		Ok(ldap)
	}

	pub async fn unbind_connection(&self, conn: &mut Ldap) {
		if let Err(e) = conn.unbind().await {
			tracing::warn!("[!] Error al desautenticar LDAP: {e}");
		}
	}
}
