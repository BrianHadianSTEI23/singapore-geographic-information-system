use axum::{routing::get, Json, Router};
use serde::{Deserialize, Serialize};
use tower_http::cors::{Any, CorsLayer};
use std::net::SocketAddr;

// Structural blueprint representing localized geometry coordinates
#[derive(Serialize, Deserialize, Clone)]
struct CoordinatePair {
    lat: f64,
    lng: f64,
}

// Custom model representing optimized demographic/boundary entries mapped from URA data
#[derive(Serialize, Deserialize, Clone)]
struct SubzoneData {
    id: String,
    name: String,
    men_population: u32,
    women_population: u32,
    coordinates: Vec<CoordinatePair>,
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

    let addr = SocketAddr::from(([127, 0, 0, 1], 8085));
    println!("🚀 Rust Engine cooking data at http://{}", addr);
    
    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn health_check() -> &'static str {
    "Operational"
}

// Endpoint merging structural geographical polygon shapes with gender data vectors
async fn get_heatmap_data() -> Json<Vec<SubzoneData>> {
    let mock_subzones = vec![
        SubzoneData {
            id: "SZ01".to_string(),
            name: "Downtown Core / Marina Central".to_string(),
            men_population: 12000,
            women_population: 8500, // Higher male skew
            coordinates: vec![
                CoordinatePair { lat: 1.2940, lng: 103.8550 },
                CoordinatePair { lat: 1.2910, lng: 103.8610 },
                CoordinatePair { lat: 1.2840, lng: 103.8580 },
                CoordinatePair { lat: 1.2860, lng: 103.8510 },
            ],
        },
        SubzoneData {
            id: "SZ02".to_string(),
            name: "Orchard / Somerset".to_string(),
            men_population: 7500,
            women_population: 14000, // Higher female skew
            coordinates: vec![
                CoordinatePair { lat: 1.3060, lng: 103.8350 },
                CoordinatePair { lat: 1.3040, lng: 103.8450 },
                CoordinatePair { lat: 1.2980, lng: 103.8420 },
                CoordinatePair { lat: 1.2990, lng: 103.8340 },
            ],
        },
        SubzoneData {
            id: "SZ03".to_string(),
            name: "Bedok North".to_string(),
            men_population: 25000,
            women_population: 24800, // Fairly balanced density
            coordinates: vec![
                CoordinatePair { lat: 1.3380, lng: 103.9200 },
                CoordinatePair { lat: 1.3320, lng: 103.9380 },
                CoordinatePair { lat: 1.3210, lng: 103.9310 },
                CoordinatePair { lat: 1.3250, lng: 103.9150 },
            ],
        },
    ];

    Json(mock_subzones)
}