use actix_web::post;
use actix_web::web::{Data, Json};
use rorm::{Database};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;

use crate::server::handler::ApiError;

use super::ApiResult;

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
    uuid: Uuid,
    name: String,
    quantity: Option<String>,
    description: Option<String>,
    image: Option<String>,
    main_category: String,
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

    let rows = db.raw_sql("SELECT uuid, name, quantity, description, image, main_category, ts_rank_cd(textsearchable_index_col, query, 32 /* rank/(rank+1) */) AS rank
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
                    uuid: Uuid::from_slice(r.get("uuid")?)
                        .map_err(|_| ApiError::InternalServerError)?,
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
