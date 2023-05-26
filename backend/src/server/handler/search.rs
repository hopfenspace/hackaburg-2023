use actix_web::post;
use actix_web::web::{Data, Json};
use rorm::{insert, Database};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;

use super::ApiResult;
use crate::models::product::{Product, ProductInsert};
use crate::server::handler::ApiError;

#[derive(Deserialize, ToSchema)]
pub struct SearchInput {
    q: String,
}

#[derive(Serialize, ToSchema)]
pub struct SearchOutput {
    results: Vec<SearchResult>,
}

#[derive(Serialize, ToSchema)]
pub struct SearchResult {
    name: String,
    quantity: String,
    description: String,
    image: String,
    main_category: String,
}

#[derive(Deserialize, ToSchema)]
pub struct PostProductRequest {
    pub ean_code: Option<String>,
    pub name: String,
    pub quantity: Option<String>,
    pub description: String,
    pub image: String,
    pub main_category: String,
}

#[utoipa::path(
    tag = "Search",
    responses(
        (status = 200, description = "Search results", body = SearchOutput),
        (status = 400, description = "Client error", body = ApiErrorResponse),
        (status = 500, description = "Server error", body = ApiErrorResponse)
    ),
    request_body = SearchInput,
    security(("api_key" = []))
)]
#[post("/api/search")]
pub async fn post_search(
    input: Json<SearchInput>,
    db: Data<Database>,
) -> ApiResult<Json<SearchOutput>> {
    let binds = [rorm_sql::value::Value::String(&input.q)];

    let rows = db.raw_sql("SELECT name, quantity, description, image, main_category, ts_rank_cd(textsearchable_index_col, query, 32 /* rank/(rank+1) */) AS rank
        FROM product, websearch_to_tsquery('german', $1) query
        WHERE query @@ textsearchable_index_col
        ORDER BY COUNT(name) OVER(PARTITION BY main_category) DESC, rank DESC
        LIMIT 1000;", Some(binds.as_slice()), None)
        .await?;

    Ok(Json(SearchOutput {
        results: rows
            .iter()
            .map(|r| {
                Ok(SearchResult {
                    name: r.get("name")?,
                    quantity: r.get("quantity")?,
                    description: r.get("description")?,
                    image: r.get("image")?,
                    main_category: r.get("main_category")?,
                })
            })
            .collect::<ApiResult<_>>()?,
    }))
}

#[post("/api/product")]
pub async fn post_product(
    input_json: Json<PostProductRequest>,
    db: Data<Database>,
) -> ApiResult<Json<Product>> {
    let input = input_json.into_inner();
    let product = insert!(db.get_ref(), Product)
        .single(&ProductInsert {
            uuid: Uuid::new_v4(),
            ean_code: input.ean_code,
            quantity: input.quantity,
            description: input.description,
            image: input.image,
            main_category: input.main_category,
            name: input.name,
        })
        .await?;
    Ok(Json(product))
}
