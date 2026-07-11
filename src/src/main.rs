use axum::{routing::get, extract::Path, Json, Router};
use serde::{Deserialize, Serialize};
use tower_http::cors::{Any, CorsLayer};
use std::net::SocketAddr;

#[derive(Serialize, Deserialize, Clone)]
struct CoordinatePair {
    lat: f64,
    lng: f64,
}

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
        .route("/api/heatmap", get(get_heatmap_data))
        .route("/api/subzone/:id", get(get_subzone_by_id)) // New deep-dive endpoint
        .layer(cors);

    let addr = SocketAddr::from(([127, 0, 1, 1], 8085));
    println!("🚀 Phase 3 Rust Engine listening on http://{}", addr);
    
    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

fn get_mock_database() -> Vec<SubzoneData> {
    vec![
        SubzoneData {
            id: "SZ01".to_string(),
            name: "Downtown Core / Marina Central".to_string(),
            men_population: 12000,
            women_population: 8500,
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
            women_population: 14000,
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
            women_population: 24800,
            coordinates: vec![
                CoordinatePair { lat: 1.3380, lng: 103.9200 },
                CoordinatePair { lat: 1.3320, lng: 103.9380 },
                CoordinatePair { lat: 1.3210, lng: 103.9310 },
                CoordinatePair { lat: 1.3250, lng: 103.9150 },
            ],
        },
    ]
}

async fn get_heatmap_data() -> Json<Vec<SubzoneData>> {
    Json(get_mock_database())
}

async fn get_subzone_by_id(Path(id): Path<String>) -> Json<Option<SubzoneData>> {
    let db = get_mock_database();
    let subzone = db.into_iter().find(|sz| sz.id == id);
    Json(subzone)
}