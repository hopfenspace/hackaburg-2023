use rorm::{Model, Patch};
use uuid::Uuid;

/// The definition of a user
#[derive(Model)]
pub struct User {
    /// Primary key of the user, a uuid v4
    #[rorm(primary_key)]
    pub uuid: Uuid,

    /// The username is used for login
    #[rorm(max_length = 255, unique, index)]
    pub username: String,

    /// This name is displayed to other users
    #[rorm(max_length = 255)]
    pub display_name: String,

    /// Password hash of the user
    #[rorm(max_length = 1024)]
    pub password_hash: String,

    /// Flag whether the user is has administrative privileges
    pub admin: bool,

    /// Last time the user has logged in
    pub last_login: Option<chrono::NaiveDateTime>,

    /// Creation time of the user
    #[rorm(auto_create_time)]
    pub created_at: chrono::NaiveDateTime,
}

#[derive(Patch)]
#[rorm(model = "User")]
pub(crate) struct UserInsert {
    pub(crate) uuid: Uuid,
    pub(crate) username: String,
    pub(crate) display_name: String,
    pub(crate) password_hash: String,
    pub(crate) admin: bool,
    pub(crate) last_login: Option<chrono::NaiveDateTime>,
}
