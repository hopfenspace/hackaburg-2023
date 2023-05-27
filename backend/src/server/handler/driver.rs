use actix_web::get;
use actix_web::web::{Data, Json, Path};
use chrono::{DateTime, TimeZone, Utc};
use rorm::{query, Database, Model};
use serde::Serialize;
use utoipa::ToSchema;

use crate::models::driver::RouteNode;
use crate::server::handler::{ApiResult, PathUuid};

#[derive(Serialize, ToSchema)]
pub struct DriverWaypoints {
    waypoints: Vec<DriverWaypoint>,
}

#[derive(Serialize, ToSchema)]
pub struct DriverWaypoint {
    pub lat: f64,
    pub lng: f64,
    pub arrival: DateTime<Utc>,
    pub name: String,
    pub address: String,
}

#[utoipa::path(
    tag = "Driver",
    context_path = "/api/v1",
    params(PathUuid),
    responses(
        (status = 200, description = "List of waypoints", body = DriverWaypoints),
        (status = 400, description = "Client error", body = ApiErrorResponse),
        (status = 500, description = "Server error", body = ApiErrorResponse)
    ),
    security(("api_key" = []))
)]
#[get("/api/driver/{uuid}/waypoints")]
pub async fn get_waypoints(
    path: Path<PathUuid>,
    db: Data<Database>,
) -> ApiResult<Json<DriverWaypoints>> {
    Ok(Json(DriverWaypoints {
        waypoints: query!(
            db.as_ref(),
            (
                RouteNode::F.shop.name,
                RouteNode::F.shop.address,
                RouteNode::F.shop.lat,
                RouteNode::F.shop.lng,
                RouteNode::F.arrival,
            )
        )
        .condition(RouteNode::F.driver.equals(path.uuid.as_ref()))
        .order_asc(RouteNode::F.index)
        .all()
        .await?
        .into_iter()
        .map(|(name, address, lat, lng, arrival)| DriverWaypoint {
            lat,
            lng,
            name,
            address,
            arrival: Utc.from_utc_datetime(&arrival),
        })
        .collect(),
    }))
}
