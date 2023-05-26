use actix_web::post;
use actix_web::web::{Data, Json};
use rorm::{insert, Database};
use serde::Deserialize;
use utoipa::ToSchema;
use uuid::Uuid;

use super::ApiResult;
use crate::models::shop::{Shop, ShopCategory, ShopInsert};

#[derive(Deserialize, ToSchema)]
pub struct PostShopRequest {
    pub name: String,
    pub description: String,
    pub category: ShopCategory,
}

#[post("/shop")]
pub async fn post_shop(
    input_json: Json<PostShopRequest>,
    db: Data<Database>,
) -> ApiResult<Json<Shop>> {
    let input = input_json.into_inner();
    let product = insert!(db.get_ref(), Shop)
        .single(&ShopInsert {
            uuid: Uuid::new_v4(),
            name: input.name,
            description: input.description,
            category: input.category,
        })
        .await?;
    Ok(Json(product))
}
