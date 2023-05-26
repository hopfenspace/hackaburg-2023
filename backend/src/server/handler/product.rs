use std::fs::File;
use std::io::Write;

use actix_web::web::{Data, Json, Query};
use actix_web::{get, post};
use rorm::{insert, query, update, Database, Model};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;

use super::ApiResult;
use crate::models::product::{Product, ProductInsert};
use crate::server::handler::ApiError;

#[derive(Deserialize, ToSchema)]
pub struct PostProductRequest {
    pub ean_code: Option<String>,
    pub name: String,
    pub quantity: Option<String>,
    pub description: Option<String>,
    pub image: Option<String>,
    pub main_category: String,
}

#[derive(Serialize, ToSchema)]
pub struct ProductImages {
    // only a single image for now
    pub image: Option<String>,
}

#[derive(Deserialize)]
pub struct ProductImagesQuery {
    pub uuid: String,
}

#[post("/api/product")]
pub async fn post_product(
    input_json: Json<PostProductRequest>,
    db: Data<Database>,
) -> ApiResult<Json<Product>> {
    let input = input_json.into_inner();
    let product = insert!(db.get_ref(), Product)
        .single(&ProductInsert {
            uuid: Uuid::new_v4(),
            ean_code: input.ean_code,
            quantity: input.quantity,
            description: input.description,
            image: input.image,
            main_category: input.main_category,
            name: input.name,
        })
        .await?;
    Ok(Json(product))
}

#[utoipa::path(
    tag = "Search",
    responses(
        (status = 200, description = "Product image", body = ProductImages),
        (status = 400, description = "Client error", body = ApiErrorResponse),
        (status = 500, description = "Server error", body = ApiErrorResponse)
    ),
    request_body = ProductImagesQuery,
    security(("api_key" = []))
)]
#[get("/api/images")]
pub async fn get_product_images(
    req: Query<ProductImagesQuery>,
    db: Data<Database>,
) -> ApiResult<Json<ProductImages>> {
    let uuid: Uuid = Uuid::parse_str(req.uuid.as_str()).map_err(|_| ApiError::MalformedInput)?;
    let product: Result<(Option<String>, Option<String>), rorm::Error> =
        query!(db.as_ref(), (Product::F.ean_code, Product::F.image))
            .condition(Product::F.uuid.equals(uuid.as_ref()))
            .one()
            .await;

    match product {
        Ok((ean_code, image)) => {
            if image.is_some() {
                return Ok(Json(ProductImages { image }));
            } else if ean_code.is_some() {
                let img = download_ean_image(ean_code.unwrap()).await;
                if img.is_some() {
                    update!(db.as_ref(), Product)
                        .set(Product::F.image, img.as_ref())
                        .condition(Product::F.uuid.equals(uuid.as_ref()))
                        .await?;
                    return Ok(Json(ProductImages { image: img }));
                } else {
                    return Ok(Json(ProductImages { image: None }));
                }
            } else {
                return Ok(Json(ProductImages { image: None }));
            }
        }
        Err(_) => Ok(Json(ProductImages { image: None })),
    }
}

async fn download_ean_image(ean: String) -> Option<String> {
    #[derive(Deserialize)]
    struct Product {
        image_front_small_url: String,
    }
    #[derive(Deserialize)]
    struct ProductRoot {
        product: Product,
    }

    let client = reqwest::Client::builder()
        .user_agent("https://github.com/hopfenspace/hackaburg-2023/")
        .build()
        .ok()?;
    let remote_image_url = client.get(
        format!("https://world.openfoodfacts.org/api/v0/product/{ean}.json?fields=image_front_small_url"))
        .send()
        .await
        .ok()?
        .json::<ProductRoot>()
        .await
        .ok()?
        .product.image_front_small_url;
    let image_response_string = client
        .get(remote_image_url)
        .send()
        .await
        .ok()?
        .text()
        .await
        .ok()?;
    let mut out = File::create(format!("image_cache/{ean}.jpg")).expect("failed to create file");
    out.write_all(image_response_string.as_bytes()).ok()?;

    Some(format!("image_cache/{ean}.jpg"))
}
