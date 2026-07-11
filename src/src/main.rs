use axum::{
    routing::get, // Cleaned up: removed unused 'post'
    Json, Router,
};
use serde::{Deserialize, Serialize};
use tower_http::cors::{Any, CorsLayer};
use std::net::SocketAddr;

#[derive(Serialize, Deserialize)]
struct HeatmapResponse {
    status: String,
    message: String,
}

#[tokio::main]
async fn main() {
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_headers(Any)
        .allow_methods(Any);

    let app = Router::new()
        .route("/api/health", get(health_check))
        .route("/api/heatmap", get(get_heatmap_data))
        .layer(cors);

    // Changed port from 8080 to 8085 to avoid Windows permission/occupation conflicts
    let addr = SocketAddr::from(([127, 0, 0, 1], 8085));
    println!("🚀 Rust Compute Engine running smoothly on http://{}", addr);
    
    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn health_check() -> &'static str {
    "Rust Engine is operational."
}

async fn get_heatmap_data() -> Json<HeatmapResponse> {
    Json(HeatmapResponse {
        status: "success".to_string(),
        message: "Ready to stream URA geospatial data.".to_string(),
    })
}