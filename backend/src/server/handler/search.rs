use actix_web::get;
use actix_web::web::{Data, Json, Query};
use rorm::Database;
use serde::{Deserialize, Serialize};
use utoipa::{IntoParams, ToSchema};
use uuid::Uuid;

use super::ApiResult;
use crate::server::handler::ApiError;

#[derive(Deserialize, IntoParams)]
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
    context_path = "/api/v1",
    responses(
        (status = 200, description = "Search results", body = SearchOutput),
        (status = 400, description = "Client error", body = ApiErrorResponse),
        (status = 500, description = "Server error", body = ApiErrorResponse)
    ),
    params(SearchInput),
    security(("api_key" = []))
)]
#[get("/search")]
pub async fn post_search(
    input: Query<SearchInput>,
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
