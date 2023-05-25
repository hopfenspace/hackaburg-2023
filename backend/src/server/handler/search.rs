use actix_web::{web::{Json, Data}, post};
use rorm::Database;
use serde::{Serialize, Deserialize};

use super::ApiResult;

#[derive(Deserialize)]
pub struct SearchInput {
	q: String
}

#[derive(Serialize)]
pub struct SearchOutput {

}

#[post("/api/search")]
pub async fn post_search(input: Json<SearchInput>, db: Data<Database>) -> ApiResult<Json<SearchOutput>> {
	todo!()
}
