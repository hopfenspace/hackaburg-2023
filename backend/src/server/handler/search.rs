use actix_web::{
    post,
    web::{Data, Json},
};
use rorm::{insert, Database};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{
    models::product::{Product, ProductInsert},
    server::handler::ApiError,
};

use super::ApiResult;

#[derive(Deserialize)]
pub struct SearchInput {
    q: String,
}

#[derive(Serialize)]
pub struct SearchOutput {
    results: Vec<SearchResult>,
}

#[derive(Serialize)]
pub struct SearchResult {
    name: String,
    quantity: String,
    description: String,
    image: String,
    main_category: String,
}

#[derive(Deserialize)]
pub struct PostProductRequest {
    pub ean_code: Option<String>,
    pub name: String,
    pub quantity: Option<String>,
    pub description: String,
    pub image: String,
    pub main_category: String,
}

#[post("/api/search")]
pub async fn post_search(
    input: Json<SearchInput>,
    db: Data<Database>,
) -> ApiResult<Json<SearchOutput>> {
    let binds = [rorm_sql::value::Value::String(&input.q)];

    db.raw_sql("SELECT name, quantity, description, image, main_category, ts_rank_cd(textsearchable_index_col, query, 32 /* rank/(rank+1) */) AS rank
        FROM product, websearch_to_tsquery('german', $1) query
        WHERE query @@ textsearchable_index_col
        ORDER BY COUNT(name) OVER(PARTITION BY main_category) DESC, rank DESC
        LIMIT 1000;", Some(binds.as_slice()), None)
        .await
        .map(|r| Json(SearchOutput {
            results: r
                .iter()
                .map(|r| SearchResult {
                    name: r.get("name").unwrap_or_default(),
                    quantity: r.get("quantity").unwrap_or_default(),
                    description: r.get("description").unwrap_or_default(),
                    image: r.get("image").unwrap_or_default(),
                    main_category: r.get("main_category").unwrap_or_default(),
                })
                .collect(),
        }))
        .map_err(|_| ApiError::InternalServerError)
}

#[post("/api/product")]
pub async fn post_product(
    input_json: Json<PostProductRequest>,
    db: Data<Database>,
) -> ApiResult<Json<Product>> {
    let input = input_json.into_inner();
    insert!(db.get_ref(), Product)
        .single(&ProductInsert {
            uuid: Uuid::new_v4(),
            ean_code: input.ean_code,
            quantity: input.quantity,
            description: input.description,
            image: input.image,
            main_category: input.main_category,
            name: input.name,
        })
        .await
        .map(|p| Json(p))
        .map_err(|_| ApiError::InternalServerError)
}
