use actix_web::post;
use actix_web::web::{Data, Json};
use rorm::{insert, Database};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;

use super::ApiResult;
use crate::models::shop::{Shop, ShopCategory, ShopInsert};

#[derive(Deserialize, ToSchema)]
pub struct CreateShopRequest {
    pub name: String,
    pub description: String,
    pub category: ShopCategory,
    pub address: String,
    pub lat: f64,
    pub lng: f64,
}

#[derive(Serialize, ToSchema)]
pub struct ShopSchema {
    pub uuid: Uuid,
    pub name: String,
    pub description: String,
    pub category: ShopCategory,
    pub address: String,
    pub lat: f64,
    pub lng: f64,
}

/// Create a new shop
#[utoipa::path(
    tag = "Shop",
    context_path = "/api/v1",
    request_body = CreateShopRequest,
    responses(
        (status = 200, description = "Created shop", body = ShopSchema),
        (status = 400, description = "Client error", body = ApiErrorResponse),
        (status = 500, description = "Server error", body = ApiErrorResponse)
    ),
    security(("api_key" = []))
)]
#[post("/shop")]
pub async fn create_shop(
    req: Json<CreateShopRequest>,
    db: Data<Database>,
) -> ApiResult<Json<ShopSchema>> {
    let CreateShopRequest {
        name,
        description,
        category,
        address,
        lat,
        lng,
    } = req.into_inner();
    let shop = insert!(db.get_ref(), Shop)
        .single(&ShopInsert {
            uuid: Uuid::new_v4(),
            name,
            description,
            category,
            address,
            lat,
            lng,
        })
        .await?;
    Ok(Json(shop.into()))
}

impl From<Shop> for ShopSchema {
    fn from(value: Shop) -> Self {
        let Shop {
            uuid,
            name,
            description,
            category,
            address,
            lat,
            lng,
            created_at: _,
        } = value;
        ShopSchema {
            uuid,
            name,
            description,
            category,
            address,
            lat,
            lng,
        }
    }
}
