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
    let data_dir = ServeDir::new("data");

    let app = Router::new()
        .route("/", get(index))
        .route("/products", get(products))
        .route("/demo", get(demo))
        .route("/extensions", get(extensions))
        .route("/mobile-apps", get(mobile_apps))
        .route("/data-analytics", get(data_analytics))
        .route("/sample-contract", get(sample_contract))
        .route("/sample-contract-pdf", get(sample_contract_pdf))
        .route("/health", get(health_check))
        .route("/api/data/:path", get(get_data))
        .nest_service("/assets", serve_dir)
        .nest_service("/data", data_dir)
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
    serve_html("frontend/dist/index.html", "index.html")
}

async fn products() -> impl IntoResponse {
    serve_html("frontend/dist/products.html", "products.html")
}

async fn demo() -> impl IntoResponse {
    serve_html("frontend/dist/demo.html", "demo.html")
}

async fn extensions() -> impl IntoResponse {
    serve_html("frontend/dist/extensions.html", "extensions.html")
}

async fn mobile_apps() -> impl IntoResponse {
    serve_html("frontend/dist/mobile-apps.html", "mobile-apps.html")
}

async fn data_analytics() -> impl IntoResponse {
    serve_html("frontend/dist/data-analytics.html", "data-analytics.html")
}

async fn sample_contract() -> impl IntoResponse {
    serve_html("frontend/dist/sample-contract.html", "sample-contract.html")
}

async fn sample_contract_pdf() -> impl IntoResponse {
    serve_html("frontend/dist/sample-contract-pdf.html", "sample-contract-pdf.html")
}

fn serve_html(path: &str, name: &str) -> Response {
    match std::fs::read_to_string(path) {
        Ok(content) => (
            StatusCode::OK,
            [(axum::http::header::CONTENT_TYPE, "text/html; charset=utf-8")],
            content,
        )
            .into_response(),
        Err(_) => (
            StatusCode::NOT_FOUND,
            format!("{name} not found"),
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
