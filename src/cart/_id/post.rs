use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use sqlx::{prelude::FromRow, QueryBuilder};

use crate::{
    server::{ServerResponse, ServerResponseResult},
    state::AppState,
};

#[derive(Deserialize, JsonSchema)]
struct CartItem {
    section: char,
    corridor: i32,
    shelf: i32,
    subshelf: Option<i32>,
    photo: String,
    is_cold: bool,
}

#[derive(Deserialize, JsonSchema)]
pub struct PostCartBody {
    items_created: Vec<CartItem>,
    items_deleted: Vec<i64>,
}

#[derive(Serialize, JsonSchema)]
pub struct CartResult {
    id: i64,
    creation_date: i64,
    name: String,
}

#[derive(Serialize, JsonSchema, FromRow)]
pub struct CartItemResult {
    id: i64,
    is_cold: i64,
    creation_date: i64,
    cart_id: i64,
    section: String,
    corridor: i64,
    shelf: i64,
    subshelf: Option<i64>,
    photo: String,
}

#[derive(Serialize, JsonSchema)]
pub struct PostCartResult {
    items_deleted: Vec<i64>,
    items_added: Vec<i64>,
    items: Vec<CartItemResult>,
}

#[derive(Deserialize, JsonSchema)]
pub struct CartId {
    cart_id: i64,
}
pub async fn post_cart_items(
    State(AppState { connection }): State<AppState>,
    Path(CartId { cart_id }): Path<CartId>,
    Json(PostCartBody {
        items_created,
        items_deleted,
    }): Json<PostCartBody>,
) -> ServerResponseResult<PostCartResult> {
    let items_deleted = if !items_deleted.is_empty() {
        let mut query = QueryBuilder::new("DELETE FROM Item WHERE id IN (");
        query.push(
            items_deleted
                .iter()
                .map(|id| id.to_string())
                .collect::<Vec<String>>()
                .join(", "),
        );
        query.push(") RETURNING id;");
        query
            .build_query_scalar::<i64>()
            .fetch_all(&connection)
            .await?
    } else {
        vec![]
    };
    let items_added = if !items_created.is_empty() {
        QueryBuilder::new(
            "INSERT INTO Item (cart_id, section, corridor, shelf, subshelf, photo, is_cold)",
        )
        .push_values(
            items_created,
            |mut b,
             CartItem {
                 section,
                 corridor,
                 shelf,
                 subshelf,
                 photo,
                 is_cold,
             }| {
                b.push_bind(cart_id)
                    .push_bind(section.to_string())
                    .push_bind(corridor)
                    .push_bind(shelf)
                    .push_bind(subshelf)
                    .push_bind(photo)
                    .push_bind(if is_cold { 1 } else { 0 });
            },
        )
        .push(" RETURNING id;")
        .build_query_scalar::<i64>()
        .fetch_all(&connection)
        .await?
    } else {
        vec![]
    };
    let items = sqlx::query_as!(
        CartItemResult,
        "SELECT id, section, creation_date, cart_id, corridor, shelf, subshelf, photo, is_cold FROM Item WHERE cart_id = ?;",
        cart_id
    )
    .fetch_all(&connection)
    .await?;

    Ok(ServerResponse::success_code(
        PostCartResult {
            items,
            items_deleted,
            items_added,
        },
        StatusCode::CREATED,
    )
    .json())
}
