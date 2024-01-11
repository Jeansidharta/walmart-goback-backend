use std::path::PathBuf;

use aide::{
    axum::{routing::get, ApiRouter, IntoApiResponse},
    openapi::{Info, OpenApi},
    redoc::Redoc,
    OperationOutput,
};
use axum::{
    extract::Request,
    http::StatusCode,
    middleware::{self, Next},
    response::{IntoResponse, Response},
    Extension, Json,
};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use sqlx::{migrate::MigrateDatabase, Pool, Sqlite};

use crate::cart::route as cart_route;

#[derive(Deserialize, Serialize, Default, JsonSchema)]
pub struct ServerResponse<T> {
    pub message: String,
    pub data: T,
    #[serde(skip)]
    pub status_code: StatusCode,
}

impl<T> OperationOutput for ServerResponse<T> {
    type Inner = ServerResponse<T>;
}

impl<T> ServerResponse<T> {
    pub fn success(data: T) -> Self {
        Self {
            message: "success".to_string(),
            data,
            status_code: StatusCode::OK,
        }
    }

    pub fn success_code(data: T, status_code: StatusCode) -> Self {
        Self {
            message: "success".to_string(),
            data,
            status_code,
        }
    }

    pub fn json(self) -> Json<Self> {
        Json(self)
    }
}

impl ServerResponse<()> {
    pub fn error(message: impl ToString) -> Self {
        ServerResponse {
            data: (),
            message: message.to_string(),
            status_code: StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}

impl<T: Serialize> IntoResponse for ServerResponse<T> {
    fn into_response(self) -> Response {
        (self.status_code, serde_json::to_string(&self).unwrap()).into_response()
    }
}

pub type ServerResponseResult<T> = Result<Json<ServerResponse<T>>, InternalServerError>;

use crate::{app_error::InternalServerError, state::AppState};

async fn logging_middleware(request: Request, next: Next) -> Response {
    let now = std::time::Instant::now();

    let response = next.run(request).await;

    let elapsed_time = now.elapsed();
    println!("Request finished in {}µs", elapsed_time.as_micros());

    response
}

async fn serve_api(Extension(api): Extension<OpenApi>) -> impl IntoApiResponse {
    Json(api)
}

async fn connect_to_database(database_path: Option<PathBuf>) -> Pool<Sqlite> {
    let database_path = database_path
        .or_else(|| {
            std::env::var("DATABASE_URL")
                .ok()
                .map(|url| url.trim_start_matches("sqlite:").into())
        })
        .or_else(|| dirs::state_dir().map(|d| d.join("walmart-gobacks/db.sqlite3")))
        .expect("Could not resolve database path");

    let database_url = format!("sqlite:{}", database_path.to_string_lossy());

    println!("Resolved URL to {}", database_url);

    if !sqlx::sqlite::Sqlite::database_exists(&database_url)
        .await
        .unwrap()
    {
        std::fs::create_dir_all(database_path.parent().unwrap())
            .expect("Failed to create directory for database");
        std::fs::write(database_path, "").expect("Failed to create database file");
        sqlx::sqlite::Sqlite::create_database(&database_url)
            .await
            .unwrap();
    }

    println!("Connecting to database at {}", database_url);

    let connection = sqlx::sqlite::SqlitePool::connect(&database_url)
        .await
        .unwrap();

    sqlx::migrate!("./migrations")
        .run(&connection)
        .await
        .unwrap();

    connection
}

pub async fn server(port: u16, database_path: Option<PathBuf>) {
    let mut api = OpenApi {
        info: Info {
            description: Some("API for the Food Tracker app".to_string()),
            ..Info::default()
        },
        ..OpenApi::default()
    };

    let connection = connect_to_database(database_path).await;

    let state = AppState { connection };

    let app = ApiRouter::new()
        .nest_api_service("/cart", cart_route(state.clone()))
        .route("/openapi.json", get(serve_api))
        .route("/docs", Redoc::new("/openapi.json").axum_route())
        .layer(middleware::from_fn(logging_middleware))
        .layer(tower_http::cors::CorsLayer::permissive())
        .with_state(state);

    println!("Server running on port {port}");

    let listener = tokio::net::TcpListener::bind(&std::net::SocketAddr::V4(
        std::net::SocketAddrV4::new(std::net::Ipv4Addr::new(0, 0, 0, 0), port),
    ))
    .await
    .unwrap();

    axum::serve(
        listener,
        app.finish_api(&mut api)
            .layer(Extension(api))
            .into_make_service(),
    )
    .await
    .unwrap();
}
