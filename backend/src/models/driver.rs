use chrono::NaiveDateTime;
use rorm::fields::{BackRef, ForeignModel};
use rorm::{field, Model, Patch};
use uuid::Uuid;

use crate::models::shop::Shop;

#[derive(Model)]
pub struct Driver {
    #[rorm(primary_key)]
    pub id: Uuid,
    pub route: BackRef<field!(RouteNode::F.driver)>,
}

#[derive(Patch)]
#[rorm(model = "Driver")]
pub struct DriverInsert {
    pub id: Uuid,
}

#[derive(Model)]
pub struct RouteNode {
    #[rorm(id)]
    pub id: i64,
    pub index: i64,
    pub driver: ForeignModel<Driver>,
    pub shop: ForeignModel<Shop>,
    pub arrival: NaiveDateTime,
}

#[derive(Patch)]
#[rorm(model = "RouteNode")]
pub struct RouteNodeInsert {
    pub index: i64,
    pub driver: ForeignModel<Driver>,
    pub shop: ForeignModel<Shop>,
    pub arrival: NaiveDateTime,
}
