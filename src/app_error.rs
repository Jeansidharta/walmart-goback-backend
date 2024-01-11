use aide::OperationOutput;
use axum::response::{IntoResponse, Response};
use schemars::JsonSchema;

use crate::server::ServerResponse;

pub struct InternalServerError(pub anyhow::Error);

impl JsonSchema for InternalServerError {
    fn schema_name() -> String {
        "InternalServerError".to_string()
    }

    fn json_schema(gen: &mut schemars::gen::SchemaGenerator) -> schemars::schema::Schema {
        gen.subschema_for::<ServerResponse<String>>()
    }
}

impl OperationOutput for InternalServerError {
    type Inner = anyhow::Error;
}

// Tell axum how to convert `AppError` into a response.
impl IntoResponse for InternalServerError {
    fn into_response(self) -> Response {
        {
            ServerResponse::error(self.0).into_response()
        }
    }
}

// This enables using `?` on functions that return `Result<_, anyhow::Error>` to turn them into
// `Result<_, AppError>`. That way you don't need to do that manually.
impl<E> From<E> for InternalServerError
where
    E: Into<anyhow::Error>,
{
    fn from(err: E) -> Self {
        Self(err.into())
    }
}
