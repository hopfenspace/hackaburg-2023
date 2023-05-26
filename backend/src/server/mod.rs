use actix_toolbox::tb_middleware::{
    setup_logging_mw, DBSessionStore, LoggingMiddlewareConfig, PersistentSession, SessionMiddleware,
};
use actix_web::cookie::time::Duration;
use actix_web::cookie::Key;
use actix_web::http::StatusCode;
use actix_web::middleware::{Compress, ErrorHandlers};
use actix_web::web::{Data, JsonConfig, PayloadConfig};
use actix_web::{App, HttpServer};
use actix_files::Files;
use base64::prelude::BASE64_STANDARD;
use base64::Engine;
use rorm::Database;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

use crate::config::Config;
use crate::server::middleware::{handle_not_found, json_extractor_error};
use crate::server::swagger::ApiDoc;

use self::handler::search::{post_search};
use self::handler::product::{post_product, get_product_images};

mod handler;
pub mod middleware;
mod swagger;

pub(crate) async fn start_server(db: Database, config: &Config) -> Result<(), String> {
    let key = Key::try_from(
        BASE64_STANDARD
            .decode(&config.server.secret_key)
            .map_err(|e| e.to_string())?
            .as_slice(),
    )
    .map_err(|e| e.to_string())?;

    HttpServer::new(move || {
        App::new()
            .app_data(Data::new(db.clone()))
            .app_data(JsonConfig::default().error_handler(json_extractor_error))
            .app_data(PayloadConfig::default())
            .wrap(setup_logging_mw(LoggingMiddlewareConfig::default()))
            .wrap(
                SessionMiddleware::builder(DBSessionStore::new(db.clone()), key.clone())
                    .session_lifecycle(PersistentSession::session_ttl(
                        PersistentSession::default(),
                        Duration::hours(1),
                    ))
                    .build(),
            )
            .wrap(Compress::default())
            .wrap(ErrorHandlers::new().handler(StatusCode::NOT_FOUND, handle_not_found))
            .service(SwaggerUi::new("/docs/{_:.*}").url("/api-doc/openapi.json", ApiDoc::openapi()))
            .service(post_search)
            .service(post_product)
            .service(get_product_images)
            .service(Files::new("/image_cache", "image_cache"))
    })
    .bind((
        config.server.listen_address.as_str(),
        config.server.listen_port,
    ))
    .map_err(|e| format!("Could not bind to address: {e}"))?
    .run()
    .await
    .map_err(|e| format!("Error starting server: {e}"))
}
