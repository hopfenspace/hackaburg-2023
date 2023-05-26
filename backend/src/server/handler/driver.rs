use actix_web::get;
use actix_web::web::{Data, Json, Path};
use rorm::Database;
use serde::Serialize;
use utoipa::ToSchema;

use crate::server::handler::{ApiResult, PathUuid};

#[derive(Serialize, ToSchema)]
pub struct DriverWaypoints {
    waypoints: Vec<DriverWaypoint>,
}

#[derive(Serialize, ToSchema)]
pub struct DriverWaypoint {
    pub lat: f64,
    pub lng: f64,
}

#[get("/api/driver/{id}/waypoints")]
pub async fn get_waypoints(
    path: Path<PathUuid>,
    db: Data<Database>,
) -> ApiResult<Json<DriverWaypoints>> {
    todo!()
}
