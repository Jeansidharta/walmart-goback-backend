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
pub struct GetCartResponse {}

#[derive(Deserialize, JsonSchema)]
pub struct CartId {
    cart_id: i64,
}

pub async fn delete_cart(
    State(AppState { connection }): State<AppState>,
    Path(CartId { cart_id }): Path<CartId>,
) -> ServerResponseResult<bool> {
    sqlx::query!("DELETE FROM Item WHERE cart_id = ?;", cart_id)
        .execute(&connection)
        .await?;

    sqlx::query_scalar!("DELETE FROM Cart WHERE id = ? RETURNING id;", cart_id)
        .fetch_one(&connection)
        .await?;

    Ok(ServerResponse::success(true).json())
}
