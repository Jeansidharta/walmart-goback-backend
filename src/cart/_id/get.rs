use axum::extract::{Path, State};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::{
    server::{ServerResponse, ServerResponseResult},
    state::AppState,
};

#[derive(Serialize, JsonSchema)]
pub struct CartResult {
    id: i64,
    creation_date: i64,
    name: String,
}

#[derive(Serialize, JsonSchema)]
struct CartItem {
    id: i64,
    section: String,
    corridor: i64,
    shelf: i64,
    subshelf: Option<i64>,
    photo: String,
}

#[derive(Serialize, JsonSchema)]
pub struct GetCartResponse {
    cart: CartResult,
    items: Vec<CartItem>,
}

#[derive(Deserialize, JsonSchema)]
pub struct CartId {
    cart_id: i64,
}

pub async fn get_cart(
    State(AppState { connection }): State<AppState>,
    Path(CartId { cart_id }): Path<CartId>,
) -> ServerResponseResult<GetCartResponse> {
    let cart = sqlx::query_as!(
        CartResult,
        "SELECT id, creation_date, name FROM Cart WHERE id = ?;",
        cart_id
    )
    .fetch_one(&connection)
    .await?;

    let items = sqlx::query_as!(
        CartItem,
        "SELECT id, section, corridor, shelf, subshelf, photo FROM Item WHERE cart_id = ?;",
        cart_id
    )
    .fetch_all(&connection)
    .await?;

    Ok(ServerResponse::success(GetCartResponse { cart, items }).json())
}
