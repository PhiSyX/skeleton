// --------- //
// Structure //
// --------- //

#[project::Repository]
pub struct UserRepository {
    // ...
}

// -------------- //
// Implémentation //
// -------------- //

impl UserRepository {
    pub fn create(&self, input: impl Into<UserEntity>) -> Result<UserEntity> {
        unimplemented!();
    }

    pub fn find_by_email(
        &self,
        email_address: &web::types::EmailAddress,
    ) -> Result<Option<UserEntity>> {
        unimplemented!();
    }

    // etc...
}
