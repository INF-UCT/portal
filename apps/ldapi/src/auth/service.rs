use crate::{auth::*, shared::*, users::*};
use ldap3::{Ldap, Scope, SearchEntry};
use std::sync::Arc;
use sword::prelude::*;

#[injectable]
pub struct AuthService {
	ldap: Arc<LdapClient>,
	user_repository: Arc<UsersRepository>,
	password_service: Arc<PassswordHashService>,
}

impl AuthService {
	pub async fn authenticate(&self, data: &AuthCredentialsDto) -> AppResult<UserView> {
		let record = self
			.authenticate_credentials(&data.username, &data.password)
			.await?;

		self.user_repository.touch_last_login(&record.id).await?;

		Ok(record.into())
	}

	async fn authenticate_credentials(&self, username: &str, password: &str) -> AppResult<User> {
		if let Some(record) = self.user_repository.find_by_uid(username).await? {
			if self
				.password_service
				.verify(password, &record.password_hash)
				.await?
			{
				tracing::info!("[✓] Usuario {} autenticado con hash local", username);

				return Ok(record);
			}

			tracing::debug!("[*] Hash local falló para {}, probando LDAP", username);

			// El password pudo cambiar en LDAP: se reintenta una vez y se actualiza el hash.
			match self.ldap_authenticate(username, password).await {
				Ok(_user_info) => {
					let new_hash = self.password_service.hash(password).await?;

					self.user_repository
						.update_password_hash(&record.id, &new_hash)
						.await?;

					tracing::info!("[✓] Hash de {} actualizado tras validar en LDAP", username);

					Ok(User {
						password_hash: new_hash,
						..record
					})
				}
				Err(_) => Err(AuthError::InvalidCredentials)?,
			}
		} else {
			tracing::info!("[*] Primer login de {}, registrando desde LDAP", username);

			let user_info = self.ldap_authenticate(username, password).await?;
			let password_hash = self.password_service.hash(password).await?;

			let record = User {
				id: UserId::new(),
				uid: username.to_string(),
				name: user_info.name,
				email: user_info.email,
				role: user_info.role,
				password_hash,
				last_login_at: None,
			};

			self.user_repository.create(&record).await?;

			Ok(record)
		}
	}

	async fn ldap_authenticate(&self, username: &str, password: &str) -> AppResult<LdapUserInfo> {
		let mut admin_ldap_conn = self.ldap.admin_connect().await?;

		let user_dn = self
			.find_user_dn(&mut admin_ldap_conn, username)
			.await
			.inspect_err(|e| {
				tracing::error!("[!] No se pudo encontrar usuario {}: {}", username, e);
			})
			.map_err(|_| AuthError::InvalidCredentials)?;

		self.ldap.unbind_connection(&mut admin_ldap_conn).await;

		let mut ldap = self.ldap.connect().await?;

		ldap.simple_bind(&user_dn, password)
			.await
			.inspect_err(|e| {
				tracing::error!("[!] Error de conexión durante bind de usuario: {e}");
			})
			.map_err(AuthError::from)?
			.success()
			.inspect_err(|e| {
				tracing::warn!("[!] Autenticación fallida para usuario {}: {e}", username);
			})
			.map_err(|_| AuthError::InvalidCredentials)?;

		tracing::info!("[✓] Usuario {} autenticado exitosamente", username);

		let user_info = self.find_user_info(&mut ldap, &user_dn, username).await?;

		self.ldap.unbind_connection(&mut ldap).await;

		Ok(user_info)
	}

	async fn find_user_dn(&self, conn: &mut Ldap, username: &str) -> AppResult<String> {
		let filter = format!("(uid={})", ldap_escape(username));

		let (results, _) = conn
			.search(
				&self.ldap.config.base_dn,
				Scope::Subtree,
				&filter,
				vec!["dn"],
			)
			.await
			.map_err(AuthError::from)?
			.success()
			.map_err(AuthError::from)?;

		if results.is_empty() {
			tracing::error!("[!] Usuario no encontrado en LDAP: {}", username);
			Err(AuthError::InvalidCredentials)?
		}

		let dn = results
			.iter()
			.map(|entry| SearchEntry::construct(entry.clone()).dn)
			.collect::<Vec<String>>()
			.first()
			.ok_or_else(|| {
				tracing::error!("[!] No se pudo extraer DN para el usuario: {}", username);
				AuthError::InvalidCredentials
			})?
			.to_owned();

		tracing::debug!("[✓] DN encontrado: {}", dn);

		Ok(dn)
	}

	async fn find_user_info(
		&self,
		conn: &mut Ldap,
		user_dn: &str,
		username: &str,
	) -> AppResult<LdapUserInfo> {
		let filter = "(|(objectClass=inetOrgPerson)(objectClass=posixAccount))";

		tracing::debug!("[*] Buscando atributos del usuario: {}", user_dn);

		let (results, _) = conn
			.search(
				user_dn,
				Scope::Base,
				filter,
				vec!["mail", "cn", "gidNumber"],
			)
			.await
			.map_err(AuthError::from)?
			.success()
			.map_err(AuthError::from)?;

		let entry = results.into_iter().next().ok_or_else(|| {
			tracing::error!("[!] no se encontró correo electrónico para el usuario: {username}");
			AuthError::InvalidCredentials
		})?;

		let entry = SearchEntry::construct(entry);

		let email = entry
			.attrs
			.get("mail")
			.and_then(|mail| mail.first().cloned())
			.ok_or_else(|| {
				tracing::error!(
					"[!] no se encontró correo electrónico para el usuario: {username}"
				);

				AuthError::InvalidCredentials
			})?;

		let name = entry
			.attrs
			.get("cn")
			.and_then(|full_name| full_name.first().cloned())
			.unwrap_or_else(|| username.to_string());

		let role = match entry.attrs.get("gidNumber").and_then(|g| g.first()) {
			Some(gid) if gid == "600" => {
				tracing::debug!("[*] Usuario {} es Func (gid=600)", username);
				UserRole::Func
			}
			Some(gid) if gid == "500" => {
				tracing::debug!("[*] Usuario {} es Student (gid=500)", username);
				UserRole::Student
			}
			Some(gid) => {
				tracing::warn!(
					"[LDAP] gidNumber desconocido para {}: {}, se asigna Student",
					username,
					gid
				);
				UserRole::Student
			}
			None => {
				tracing::warn!(
					"[LDAP] usuario sin gidNumber: {}, se asigna Student",
					username
				);
				UserRole::Student
			}
		};

		let user_info = LdapUserInfo { email, name, role };

		tracing::info!(
			"[✓] Información obtenida para {}: email={}, role={:?}",
			username,
			user_info.email,
			user_info.role
		);

		Ok(user_info)
	}
}
