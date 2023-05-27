use std::fs::File;
use std::io::Cursor;

use actix_web::web::{Data, Json, Path};
use actix_web::{get, post};
use rorm::fields::ForeignModelByField;
use rorm::{insert, query, update, Database, Model};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;

use super::ApiResult;
use crate::models::product::{Product, ProductInsert};
use crate::server::handler::{ApiError, PathUuid};

#[derive(Deserialize, ToSchema)]
pub struct CreateProductRequest {
    pub ean_code: Option<String>,
    pub name: String,
    pub quantity: Option<String>,
    pub description: Option<String>,
    pub image: Option<String>,
    pub main_category: String,
    pub shop: Uuid,
}

#[derive(Serialize, ToSchema)]
pub struct ProductSchema {
    pub uuid: Uuid,
    pub shop: Uuid,
    pub ean_code: Option<String>,
    pub name: String,
    pub quantity: Option<String>,
    pub description: Option<String>,
    pub image_state: ImageState,
    pub image: Option<String>,
    pub main_category: String,
}

#[derive(Serialize, ToSchema)]
pub enum ImageState {
    Untried,
    Found,
    NotFound,
}

#[derive(Serialize, ToSchema)]
pub struct ImageResult {
    pub image_state: ImageState,
    pub image: Option<String>,
}

#[utoipa::path(
    tag = "Product",
    context_path = "/api/v1",
    responses(
        (status = 200, description = "Product", body = ProductSchema),
        (status = 400, description = "Client error", body = ApiErrorResponse),
        (status = 500, description = "Server error", body = ApiErrorResponse)
    ),
    params(PathUuid),
    security(("api_key" = []))
)]
#[get("/product/{uuid}")]
pub async fn get_product(
    db: Data<Database>,
    path: Path<PathUuid>,
) -> ApiResult<Json<ProductSchema>> {
    query!(db.as_ref(), Product)
        .condition(Product::F.uuid.equals(path.uuid.as_ref()))
        .optional()
        .await?
        .map(Into::into)
        .map(Json)
        .ok_or(ApiError::NotFound)
}

#[derive(Serialize, ToSchema)]
pub struct ProductImages {
    // only a single image for now
    pub image: Option<String>,
}

#[utoipa::path(
    tag = "Product",
    context_path = "/api/v1",
    request_body = CreateProductRequest,
    responses(
        (status = 200, description = "Created product", body = ProductSchema),
        (status = 400, description = "Client error", body = ApiErrorResponse),
        (status = 500, description = "Server error", body = ApiErrorResponse)
    ),
    security(("api_key" = []))
)]
#[post("/product")]
pub async fn create_product(
    input_json: Json<CreateProductRequest>,
    db: Data<Database>,
) -> ApiResult<Json<ProductSchema>> {
    let input = input_json.into_inner();
    let product = insert!(db.get_ref(), Product)
        .single(&ProductInsert {
            uuid: Uuid::new_v4(),
            shop: ForeignModelByField::Key(input.shop),
            ean_code: input.ean_code,
            quantity: input.quantity,
            description: input.description,
            image: input.image,
            main_category: input.main_category,
            name: input.name,
        })
        .await?;
    Ok(Json(product.into()))
}

#[utoipa::path(
    tag = "Product",
    context_path = "/api/v1",
    responses(
        (status = 200, description = "Product image", body = ImageResult),
        (status = 400, description = "Client error", body = ApiErrorResponse),
        (status = 500, description = "Server error", body = ApiErrorResponse)
    ),
    params(PathUuid),
    security(("api_key" = []))
)]
#[get("/product/{uuid}/image")]
pub async fn get_product_images(
    path: Path<PathUuid>,
    db: Data<Database>,
) -> ApiResult<Json<ImageResult>> {
    let (ean_code, image, requested) = query!(
        db.as_ref(),
        (
            Product::F.ean_code,
            Product::F.image,
            Product::F.image_requested
        )
    )
    .condition(Product::F.uuid.equals(path.uuid.as_ref()))
    .one()
    .await?;

    let state = if let Some(image) = image {
        ImageResult {
            image_state: ImageState::Found,
            image: Some(image),
        }
    } else if requested || ean_code.is_none() {
        ImageResult {
            image_state: ImageState::NotFound,
            image: None,
        }
    } else {
        if let Some(image) = download_ean_image(ean_code.expect("Checked in other if branch")).await
        {
            update!(db.as_ref(), Product)
                .set(Product::F.image, &image)
                .set(Product::F.image_requested, true)
                .condition(Product::F.uuid.equals(path.uuid.as_ref()))
                .await?;
            ImageResult {
                image_state: ImageState::Found,
                image: Some(image),
            }
        } else {
            ImageResult {
                image_state: ImageState::NotFound,
                image: None
            }
        }
    };
    Ok(Json(state))
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

    let file_path = format!("image_cache/{ean}.jpg");

    if std::path::Path::new(&file_path).exists() {
        return Some(file_path);
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
        .bytes()
        .await
        .ok()?;
    let mut out = File::create(&file_path).expect("failed to create file");
    let mut content = Cursor::new(image_response_string);
    std::io::copy(&mut content, &mut out).ok()?;

    Some(file_path)
}

impl From<Product> for ProductSchema {
    fn from(value: Product) -> Self {
        let Product {
            uuid,
            shop,
            ean_code,
            name,
            quantity,
            description,
            image,
            image_requested,
            main_category,
            created_at: _,
        } = value;
        ProductSchema {
            uuid,
            ean_code,
            name,
            quantity,
            description,
            image_state: if image.is_some() {
                ImageState::Found
            } else if image_requested {
                ImageState::NotFound
            } else {
                ImageState::Untried
            },
            image,
            main_category,
            shop: *shop.key(),
        }
    }
}
