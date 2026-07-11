// main.rs
use axum::{routing::get, extract::Path, Json, Router};
use serde::{Deserialize, Serialize};
use tower_http::cors::{Any, CorsLayer};
use std::net::SocketAddr;
use std::sync::OnceLock;
use std::fs::File;
use std::io::BufReader;
use geo::{Polygon, Point, Contains};
use geojson::{GeoJson, Value};

// Global static storage to load and cache the processed database once at startup
static DATABASE: OnceLock<Vec<SubzoneData>> = OnceLock::new();

#[derive(Serialize, Deserialize, Clone, Debug)]
struct CoordinatePair {
    lat: f64,
    lng: f64,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
struct SubzoneData {
    id: String,
    name: String,
    men_population: u32,
    women_population: u32,
    coordinates: Vec<CoordinatePair>,
}

// CSV row deserialization structures
#[derive(Deserialize)]
struct MenRow {
    longitude: f64,
    latitude: f64,
    sgp_men_2020: f64, // parsed as f64 in case values contain decimals, then casted
}

#[derive(Deserialize)]
struct WomenRow {
    longitude: f64,
    latitude: f64,
    sgp_women_2020: f64,
}

#[tokio::main]
async fn main() {
    // Initialize database from files before booting the server
    println!("⏳ Loading and processing geospatial datasets...");
    let db = load_datasets().expect("Failed to load and aggregate datasets");
    DATABASE.set(db).unwrap();
    println!("✅ Database successfully loaded into memory!");

    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_headers(Any)
        .allow_methods(Any);

    let app = Router::new()
        .route("/api/heatmap", get(get_heatmap_data))
        .route("/api/subzone/:id", get(get_subzone_by_id))
        .layer(cors);

    let addr = SocketAddr::from(([127, 0, 1, 1], 8085));
    println!("🚀 Phase 3 Rust Engine listening on http://{}", addr);
    
    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

fn load_datasets() -> Result<Vec<SubzoneData>, Box<dyn std::error::Error>> {
    // 1. Read and parse GeoJSON file
    let geojson_file = File::open("./data/sgp.geojson")?;
    let reader = BufReader::new(geojson_file);
    let geojson = serde_json::from_reader::<_, GeoJson>(reader)?;

    let mut subzones = Vec::new();

    if let GeoJson::FeatureCollection(collection) = geojson {
        // Cap GeoJSON parsing at the first 1000 features
        for feature in collection.features.into_iter().take(500) {
            let properties = feature.properties.clone().unwrap_or_default();
            
            let id = properties.get("SUBZONE_C")
                .and_then(|v| v.as_str())
                .unwrap_or("UNKNOWN")
                .to_string();
            let name = properties.get("SUBZONE_N")
                .and_then(|v| v.as_str())
                .unwrap_or("UNKNOWN")
                .to_string();

            if let Some(geometry) = feature.geometry {
                if let Value::Polygon(ref poly_coords) = geometry.value {
                    let coordinates: Vec<CoordinatePair> = poly_coords[0]
                        .iter()
                        .map(|coord| CoordinatePair {
                            lng: coord[0],
                            lat: coord[1],
                        })
                        .collect();

                    subzones.push(SubzoneData {
                        id,
                        name,
                        men_population: 0,
                        women_population: 0,
                        coordinates,
                    });
                }
            }
        }
    }

    // Convert boundaries into geo-crate Polygons
    let geo_polygons: Vec<Polygon<f64>> = subzones
        .iter()
        .map(|sz| {
            let line_string: Vec<(f64, f64)> = sz.coordinates.iter().map(|c| (c.lng, c.lat)).collect();
            Polygon::new(geo::LineString::from(line_string), vec![])
        })
        .collect();

    // 2. Aggregate Men Population CSV data (Limited to first 1000 data rows)
    if let Ok(men_file) = File::open("./data/sgp_men_2020.csv") {
        let mut rdr = csv::Reader::from_reader(men_file);
        for result in rdr.deserialize::<MenRow>().take(500) {
            if let Ok(row) = result {
                let point = Point::new(row.longitude, row.latitude);
                for (i, poly) in geo_polygons.iter().enumerate() {
                    if poly.contains(&point) {
                        subzones[i].men_population += row.sgp_men_2020.round() as u32;
                        break;
                    }
                }
            }
        }
    }

    // 3. Aggregate Women Population CSV data (Limited to first 1000 data rows)
    if let Ok(women_file) = File::open("./data/sgp_women_2020.csv") {
        let mut rdr = csv::Reader::from_reader(women_file);
        for result in rdr.deserialize::<WomenRow>().take(500) {
            if let Ok(row) = result {
                let point = Point::new(row.longitude, row.latitude);
                for (i, poly) in geo_polygons.iter().enumerate() {
                    if poly.contains(&point) {
                        subzones[i].women_population += row.sgp_women_2020.round() as u32;
                        break;
                    }
                }
            }
        }
    }

    Ok(subzones)
}

async fn get_heatmap_data() -> Json<Vec<SubzoneData>> {
    let db = DATABASE.get().cloned().unwrap_or_default();
    Json(db)
}

async fn get_subzone_by_id(Path(id): Path<String>) -> Json<Option<SubzoneData>> {
    let db = DATABASE.get().cloned().unwrap_or_default();
    let subzone = db.into_iter().find(|sz| sz.id == id);
    Json(subzone)
}