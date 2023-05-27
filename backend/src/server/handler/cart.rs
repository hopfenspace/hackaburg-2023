use actix_toolbox::tb_middleware::Session;
use actix_web::web::{Data, Json};
use actix_web::{get, put, HttpResponse};
use rorm::fields::ForeignModelByField;
use rorm::{delete, insert, query, Database, Model};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;

use crate::models::product::Product;
use crate::models::user::{CartEntry, CartEntryInsert};
use crate::server::handler::product::ProductSchema;
use crate::server::handler::{ApiError, ApiResult};

#[derive(Serialize, ToSchema)]
pub struct CartSchema {
    products: Vec<CartEntrySchema>,
}

#[derive(Serialize, ToSchema)]
pub struct CartEntrySchema {
    product: ProductSchema,
    amount: i64,
}

#[utoipa::path(
    tag = "Cart",
    context_path = "/api/v1",
    responses(
        (status = 200, description = "The user's cart", body = CartSchema),
        (status = 400, description = "Client error", body = ApiErrorResponse),
        (status = 500, description = "Server error", body = ApiErrorResponse)
    ),
    security(("api_key" = []))
)]
#[get("/cart")]
pub async fn get_cart(db: Data<Database>, session: Session) -> ApiResult<Json<CartSchema>> {
    let uuid: Uuid = session.get("uuid")?.ok_or(ApiError::Unauthenticated)?;
    let products = query!(
        db.as_ref(),
        (
            CartEntry::F.amount,
            CartEntry::F.product.uuid,
            CartEntry::F.product.shop,
            CartEntry::F.product.ean_code,
            CartEntry::F.product.name,
            CartEntry::F.product.quantity,
            CartEntry::F.product.description,
            CartEntry::F.product.image,
            CartEntry::F.product.image_requested,
            CartEntry::F.product.main_category,
            CartEntry::F.product.created_at,
        )
    )
    .condition(CartEntry::F.user.equals(uuid.as_ref()))
    .order_asc(CartEntry::F.index)
    .all()
    .await?
    .into_iter()
    .map(
        |(
            amount,
            uuid,
            shop,
            ean_code,
            name,
            quantity,
            description,
            image,
            image_requested,
            main_category,
            created_at,
        )| {
            CartEntrySchema {
                amount,
                product: Product {
                    uuid,
                    shop,
                    ean_code,
                    name,
                    quantity,
                    description,
                    image,
                    image_requested,
                    main_category,
                    created_at,
                }
                .into(),
            }
        },
    )
    .collect();
    Ok(Json(CartSchema { products }))
}

#[derive(Deserialize, ToSchema)]
pub struct PutCartSchema {
    products: Vec<PutCartEntrySchema>,
}

#[derive(Deserialize, ToSchema)]
pub struct PutCartEntrySchema {
    product: Uuid,
    amount: i64,
}

#[utoipa::path(
    tag = "Cart",
    context_path = "/api/v1",
    request_body = PutCartSchema,
    responses(
        (status = 200, description = "The user's cart has been updated"),
        (status = 400, description = "Client error", body = ApiErrorResponse),
        (status = 500, description = "Server error", body = ApiErrorResponse)
    ),
    security(("api_key" = []))
)]
#[put("/cart")]
pub async fn put_cart(
    db: Data<Database>,
    req: Json<PutCartSchema>,
    session: Session,
) -> ApiResult<HttpResponse> {
    let uuid: Uuid = session.get("uuid")?.ok_or(ApiError::Unauthenticated)?;

    let mut tx = db.start_transaction().await?;
    delete!(&mut tx, CartEntry)
        .condition(CartEntry::F.user.equals(uuid.as_ref()))
        .await?;
    let products: Vec<_> = req
        .into_inner()
        .products
        .into_iter()
        .enumerate()
        .map(
            |(index, PutCartEntrySchema { amount, product })| CartEntryInsert {
                index: index as i64,
                amount,
                product: ForeignModelByField::Key(product),
                user: ForeignModelByField::Key(uuid),
            },
        )
        .collect();
    insert!(&mut tx, CartEntryInsert).bulk(&products).await?;
    Ok(HttpResponse::Ok().finish())
}
