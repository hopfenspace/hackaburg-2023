//! This module holds the definition of the swagger declaration

use utoipa::openapi::security::{ApiKey, ApiKeyValue, SecurityScheme};
use utoipa::{Modify, OpenApi};

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
#[openapi(paths(), components(schemas()), modifiers(&CookieSecurity))]
pub struct ApiDoc;