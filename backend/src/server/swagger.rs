//! This module holds the definition of the swagger declaration

use utoipa::openapi::security::{ApiKey, ApiKeyValue, SecurityScheme};
use utoipa::{Modify, OpenApi};

use crate::server::handler;

struct CookieSecurity;

impl Modify for CookieSecurity {
    fn modify(&self, openapi: &mut utoipa::openapi::OpenApi) {
        if let Some(components) = openapi.components.as_mut() {
            components.add_security_scheme(
                "session_cookie",
                SecurityScheme::ApiKey(ApiKey::Cookie(ApiKeyValue::new("id"))),
            )
        }
    }
}

/// Helper struct for the openapi definitions.
#[derive(OpenApi)]
#[openapi(
    paths(
        handler::search::post_search,
        handler::product::create_product,
        handler::product::get_product,
        handler::product::get_product_images,
        handler::shop::create_shop,
        handler::cart::get_cart,
        handler::cart::put_cart,
        handler::driver::get_waypoints,
        handler::auth::login,
        handler::auth::logout,
    ),
    components(schemas(
        handler::ApiStatusCode,
        handler::ApiErrorResponse,
        handler::search::SearchOutput,
        handler::search::SearchResult,
        handler::product::ProductSchema,
        handler::product::ProductImages,
        handler::product::ImageState,
        handler::product::CreateProductRequest,
        handler::shop::CreateShopRequest,
        handler::shop::ShopSchema,
        handler::cart::CartSchema,
        handler::cart::CartEntrySchema,
        handler::cart::PutCartSchema,
        handler::cart::PutCartEntrySchema,
        handler::driver::DriverWaypoints,
        handler::driver::DriverWaypoint,
        handler::auth::LoginRequest,
    )),
    modifiers(&CookieSecurity)
)]
pub struct ApiDoc;
