use aide::axum::{routing::get, ApiRouter};

use crate::state::AppState;

mod get;

pub fn route(state: AppState) -> ApiRouter {
    ApiRouter::new()
        .api_route("/", get(get::get_cart))
        .with_state(state)
}
