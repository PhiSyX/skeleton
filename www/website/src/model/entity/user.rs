// --------- //
// Structure //
// --------- //

/// Concerne une entité d'un utilisateur dans une base de données.
#[derive(serde::Deserialize, serde::Serialize, project::Entity)]
pub struct UserEntity {
    /// ID sous forme d'UUID.
    pub id: uuid::Uuid,
    /// Pseudo public.
    pub username: String,
    /// Mot de passe chiffré.
    #[serde(skip_serializing)]
    pub password: String,
    /// Son adresse e-mail.
    pub email_address: web::types::EmailAddress,

    // etc...
}
