use aide::axum::{routing::post, ApiRouter};

use crate::state::AppState;

mod _id;
mod get;
mod post;

pub fn route(state: AppState) -> ApiRouter {
    ApiRouter::new()
        .api_route("/", post(post::post_cart).get(get::list_carts))
        .nest_api_service("/:cart_id", _id::route(state.clone()))
        .with_state(state)
}
