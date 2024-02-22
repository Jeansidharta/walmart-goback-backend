use axum::{extract::State, http::StatusCode, Json};
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
    is_cold: Option<bool>,
}

#[derive(Deserialize, JsonSchema)]
pub struct PostCartBody {
    name: String,
    items: Vec<CartItem>,
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
    creation_date: i64,
    cart_id: i64,
    section: String,
    corridor: i64,
    shelf: i64,
    subshelf: Option<i64>,
    photo: String,
    is_cold: i64,
}

#[derive(Serialize, JsonSchema)]
pub struct PostCartResult {
    cart: CartResult,
    items: Vec<CartItemResult>,
}

pub async fn post_cart(
    State(AppState { connection }): State<AppState>,
    Json(PostCartBody { name, items }): Json<PostCartBody>,
) -> ServerResponseResult<PostCartResult> {
    let cart = sqlx::query_as!(
        CartResult,
        "INSERT INTO Cart (name) VALUES (?) RETURNING id, creation_date, name;",
        name
    )
    .fetch_one(&connection)
    .await?;

    let items = if !items.is_empty() {
        QueryBuilder::new(
            "INSERT INTO Item (cart_id, section, corridor, shelf, subshelf, photo, is_cold)",
        )
        .push_values(
            items,
            |mut b,
             CartItem {
                 section,
                 corridor,
                 shelf,
                 subshelf,
                 photo,
                 is_cold,
             }| {
                b.push_bind(cart.id)
                    .push_bind(section.to_string())
                    .push_bind(corridor)
                    .push_bind(shelf)
                    .push_bind(subshelf)
                    .push_bind(photo)
                    .push_bind(if is_cold.unwrap_or_default(){ 1 } else { 0 });
            },
        )
        .push("RETURNING id, creation_date, cart_id, section, corridor, shelf, subshelf, photo, is_cold")
        .build_query_as::<CartItemResult>()
        .fetch_all(&connection)
        .await?
    } else {
        vec![]
    };

    Ok(ServerResponse::success_code(PostCartResult { cart, items }, StatusCode::CREATED).json())
}
