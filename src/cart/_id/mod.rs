use aide::axum::{routing::get, ApiRouter};

use crate::state::AppState;

mod delete;
mod get;
mod post;

pub fn route(state: AppState) -> ApiRouter {
    ApiRouter::new()
        .api_route(
            "/",
            get(get::get_cart)
                .delete(delete::delete_cart)
                .post(post::post_cart_items),
        )
        .with_state(state)
}
