use rorm::{DbEnum, Model, Patch};
use uuid::Uuid;

#[derive(DbEnum)]
pub enum ShopCategory {
    Groceries,
    DrugStore,
    Flowers,
}

/// The definition of a user
#[derive(Model)]
pub struct Shop {
    /// Primary key of the shop, a uuid v4
    #[rorm(primary_key)]
    pub uuid: Uuid,

    /// Shop display name
    #[rorm(max_length = 255, unique, index)]
    pub name: String,

    /// Shop description
    #[rorm(max_length = 512)]
    pub description: String,

    /// Shop category
    pub category: ShopCategory,

    /// Creation time of the user
    #[rorm(auto_create_time)]
    pub created_at: chrono::NaiveDateTime,
}

#[derive(Patch)]
#[rorm(model = "Shop")]
pub(crate) struct ShopInsert {
    pub(crate) uuid: Uuid,
    pub(crate) name: String,
    pub(crate) description: String,
    pub(crate) category: ShopCategory,
}
