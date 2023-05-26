use rorm::fields::ForeignModel;
use rorm::{DbEnum, Model, Patch};
use uuid::Uuid;

use super::shop::Shop;

#[derive(DbEnum)]
pub enum ProductCategory {
    All,
    BabyFoods,
    Beverages,
    Breads,
    Broths,
    Candies,
    CannedFruits,
    CannedVegetables,
    Cheeses,
    Chocolates,
    Condiments,
    Dairies,
    Desserts,
    DietarySupplements,
    Eggs,
    Flours,
    Flowers,
    Fruits,
    Jams,
    Meals,
    Meats,
    Milks,
    Pastas,
    PlantBasedFoodsAndBeverages,
    Sandwiches,
    Seafood,
    Snacks,
    Spreads,
    Sugars,
    Sweeteners,
    Vegetables,
}

/// The definition of a user
#[derive(Model)]
pub struct Product {
    /// Primary key of the product item, a uuid v4
    #[rorm(primary_key)]
    pub uuid: Uuid,

    /// Shop which owns this product
    #[rorm(on_update = "Cascade", on_delete = "Cascade")]
    pub shop: ForeignModel<Shop>,

    /// EAN-13 or EAN-7 code of this product
    #[rorm(max_length = 13, index)]
    pub ean_code: Option<String>,

    /// Product item display name
    #[rorm(max_length = 255, index)]
    pub name: String,

    /// Typical product item quantity
    #[rorm(max_length = 64)]
    pub quantity: Option<String>,

    /// Product item display name
    #[rorm(max_length = 4096)]
    pub description: Option<String>,

    /// Product item image URL
    #[rorm(max_length = 512)]
    pub image: Option<String>,

    #[rorm(default = "false")]
    pub image_requested: bool,

    /// Most specific product category
    #[rorm(max_length = 64)]
    pub main_category: String,

    /// Creation time of the user
    #[rorm(auto_create_time)]
    pub created_at: chrono::NaiveDateTime,
}

#[derive(Patch)]
#[rorm(model = "Product")]
pub struct ProductInsert {
    pub uuid: Uuid,
    pub shop: ForeignModel<Shop>,
    pub ean_code: Option<String>,
    pub name: String,
    pub quantity: Option<String>,
    pub description: Option<String>,
    pub image: Option<String>,
    pub main_category: String,
}
