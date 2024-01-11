use axum::extract::State;
use schemars::JsonSchema;
use serde::Serialize;

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
pub struct ListCartsResponse {
    carts: Vec<CartResult>,
}

pub async fn list_carts(
    State(AppState { connection }): State<AppState>,
) -> ServerResponseResult<ListCartsResponse> {
    let carts = sqlx::query_as!(CartResult, "SELECT id, creation_date, name FROM Cart")
        .fetch_all(&connection)
        .await?;

    Ok(ServerResponse::success(ListCartsResponse { carts }).json())
}
