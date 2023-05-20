use super::dto;

// --------- //
// Structure //
// --------- //

pub struct AuthController;

// -------------- //
// Implémentation //
// -------------- //

impl AuthController {
    pub async fn show_login_page(
		ctx: HttpContextContract,
	) -> Result<impl IntoResponse, AuthResponseError> {
		unimplemented!()
	}

	pub async fn login(
		ctx: HttpContextContract<dto::UserLoginCredentialsDTO>,
	) -> Result<impl IntoResponse, AuthResponseError> {
		unimplemented!()
	}

    
    pub async fn show_register_page(
		ctx: HttpContextContract,
	) -> Result<impl IntoResponse, AuthResponseError> {
		unimplemented!()
	}

	pub async fn register(
		Repository(user_repository): Repository<UserRepository>,
		Extension(encryption_service): Extension<EncryptionService>,
		Extension(auth_mailer): Extension<AuthMailerService>,
		ctx: HttpContextContract<dto::UserRegistrationCredentialsDTO>,
	) -> Result<impl IntoResponse, AuthResponseError> {
		unimplemented!()
    }
	
	// etc...
}
