use rorm::{DbEnum, Model, Patch};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(DbEnum, Deserialize, Serialize)]
pub enum ShopCategory {
    Groceries,
    DrugStore,
    Flowers,
}

/// The definition of a user
#[derive(Model, Serialize)]
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

    /// Human understandable address
    #[rorm(max_length = 255)]
    pub address: String,

    /// Shop address' latitude
    pub lat: f64,

    /// Shop address' longitude
    pub lng: f64,

    /// Creation time of the user
    #[rorm(auto_create_time)]
    pub created_at: chrono::NaiveDateTime,
}

#[derive(Patch)]
#[rorm(model = "Shop")]
pub struct ShopInsert {
    pub uuid: Uuid,
    pub name: String,
    pub description: String,
    pub category: ShopCategory,
    pub address: String,
    pub lat: f64,
    pub lng: f64,
}
