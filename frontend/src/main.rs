use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    routing::get,
    Json, Router,
};
use serde_json::json;
use serde_json::Value;
use std::sync::Arc;
use tower_http::compression::CompressionLayer;
use tower_http::services::ServeDir;
use tracing::info;

use kreatives_frontend::data_loader;

#[derive(Clone)]
pub struct AppState {
    // Add any shared state here
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();
    info!("Starting Kreatives server");

    let state = AppState {};

    let serve_dir = ServeDir::new("frontend/dist");

    let app = Router::new()
        .route("/", get(index))
        .route("/health", get(health_check))
        .route("/api/data/:path", get(get_data))
        .nest_service("/assets", serve_dir)
        .layer(CompressionLayer::new())
        .with_state(Arc::new(state));

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .expect("Failed to bind to port 3000");

    info!("Server running on http://127.0.0.1:3000");

    axum::serve(listener, app)
        .await
        .expect("Server error");
}

async fn index() -> impl IntoResponse {
    // Serve the HTML file
    match std::fs::read_to_string("frontend/dist/index.html") {
        Ok(content) => (
            StatusCode::OK,
            [(axum::http::header::CONTENT_TYPE, "text/html; charset=utf-8")],
            content,
        )
            .into_response(),
        Err(_) => (
            StatusCode::NOT_FOUND,
            "index.html not found".to_string(),
        )
            .into_response(),
    }
}

async fn get_data(
    axum::extract::Path(path): axum::extract::Path<String>,
) -> Result<Json<Value>, ApiError> {
    let data_path = format!("./data/{}", path);
    
    match data_loader::load_data(&data_path) {
        Ok(data) => Ok(Json(data)),
        Err(e) => {
            eprintln!("Error loading data: {}", e);
            Err(ApiError::NotFound)
        }
    }
}

async fn health_check() -> impl IntoResponse {
    Json(json!({
        "status": "ok",
        "timestamp": chrono::Utc::now().to_rfc3339()
    }))
}

pub enum ApiError {
    NotFound,
    InternalError,
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        match self {
            ApiError::NotFound => (StatusCode::NOT_FOUND, "Data not found").into_response(),
            ApiError::InternalError => {
                (StatusCode::INTERNAL_SERVER_ERROR, "Internal server error").into_response()
            }
        }
    }
}
