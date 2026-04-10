mod measure;
mod pose;
mod segmentation;
mod size_map;

use anyhow::Result;
use axum::{
    extract::{Multipart, State},
    http::StatusCode,
    response::Json,
    routing::{get, post},
    Router,
};
use once_cell::sync::OnceCell;
use serde_json::{json, Value};
use std::{net::SocketAddr, sync::{Arc, Mutex}};
use tower_http::{cors::CorsLayer, trace::TraceLayer};
use tracing::info;
use tracing_subscriber::EnvFilter;

use crate::pose::PoseEstimator;
use crate::segmentation::Segmentor;

// ── Shared application state ──────────────────────────────────────────────────

pub struct AppState {
    pub pose_estimator: Option<Mutex<PoseEstimator>>,
    pub segmentor:      Option<Mutex<Segmentor>>,
    pub demo_mode:      bool,
}

// ── Entry point ───────────────────────────────────────────────────────────────

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env().add_directive("klother_measure=debug".parse()?))
        .init();

    let model_dir = std::env::var("MODEL_DIR").unwrap_or_else(|_| "models".to_string());

    info!("Loading ONNX models from {model_dir}");

    // Attempt to load models; fall back to demo mode if they're missing.
    let blazepose_path = format!("{model_dir}/blazepose_full.onnx");
    let u2net_path     = format!("{model_dir}/u2net.onnx");

    let both_exist = std::path::Path::new(&blazepose_path).exists()
        && std::path::Path::new(&u2net_path).exists();

    let state = if both_exist {
        info!("Models found — running in inference mode");
        Arc::new(AppState {
            pose_estimator: Some(Mutex::new(PoseEstimator::load(&blazepose_path)?)),
            segmentor:      Some(Mutex::new(Segmentor::load(&u2net_path)?)),
            demo_mode:      false,
        })
    } else {
        info!("One or more model files missing — running in DEMO mode (fixed measurements returned)");
        Arc::new(AppState {
            pose_estimator: None,
            segmentor:      None,
            demo_mode:      true,
        })
    };

    let app = Router::new()
        .route("/health",  get(health))
        .route("/measure", post(handle_measure))
        .layer(CorsLayer::permissive())
        .layer(TraceLayer::new_for_http())
        .with_state(state);

    let port: u16 = std::env::var("PORT")
        .ok()
        .and_then(|p| p.parse().ok())
        .unwrap_or(9090);

    let addr = SocketAddr::from(([0, 0, 0, 0], port));
    info!("Klother measurement service listening on {addr}");

    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}

// ── Health check ──────────────────────────────────────────────────────────────

async fn health() -> Json<Value> {
    Json(json!({ "status": "ok" }))
}

// ── POST /measure ─────────────────────────────────────────────────────────────

async fn handle_measure(
    State(state): State<Arc<AppState>>,
    mut multipart: Multipart,
) -> Result<Json<Value>, (StatusCode, Json<Value>)> {

    // Collect multipart fields
    let mut front_bytes: Option<Vec<u8>> = None;
    let mut side_bytes:  Option<Vec<u8>> = None;
    let mut height_cm:   Option<f32>     = None;
    let mut gender:      String           = "women".to_string();

    while let Some(field) = multipart.next_field().await.map_err(|e| {
        (StatusCode::BAD_REQUEST, Json(json!({ "error": e.to_string() })))
    })? {
        let name = field.name().unwrap_or("").to_string();
        match name.as_str() {
            "front_photo" => {
                front_bytes = Some(field.bytes().await.map_err(|e| {
                    (StatusCode::BAD_REQUEST, Json(json!({ "error": e.to_string() })))
                })?.to_vec());
            }
            "side_photo" => {
                side_bytes = Some(field.bytes().await.map_err(|e| {
                    (StatusCode::BAD_REQUEST, Json(json!({ "error": e.to_string() })))
                })?.to_vec());
            }
            "height_cm" => {
                let text = field.text().await.map_err(|e| {
                    (StatusCode::BAD_REQUEST, Json(json!({ "error": e.to_string() })))
                })?;
                height_cm = text.parse::<f32>().ok();
            }
            "gender" => {
                gender = field.text().await.unwrap_or_else(|_| "women".to_string());
            }
            _ => { let _ = field.bytes().await; }
        }
    }

    // Validate
    let front = front_bytes.ok_or_else(|| {
        (StatusCode::BAD_REQUEST, Json(json!({ "error": "front_photo is required" })))
    })?;
    let side  = side_bytes.ok_or_else(|| {
        (StatusCode::BAD_REQUEST, Json(json!({ "error": "side_photo is required" })))
    })?;
    let height = height_cm.filter(|&h| (130.0..=220.0).contains(&h)).ok_or_else(|| {
        (StatusCode::BAD_REQUEST, Json(json!({ "error": "height_cm must be 130–220" })))
    })?;

    // ── Core pipeline (runs on Tokio thread pool — no blocking I/O) ──────────
    let measurements = if state.demo_mode {
        // Demo mode: derive plausible measurements from height + a seeded hash of
        // the front photo bytes so different photos give slightly different results.
        let seed = front.iter().fold(0u64, |acc, &b| acc.wrapping_add(b as u64));
        let variance = ((seed % 12) as f32) - 6.0; // ±6 cm variation
        measure::BodyMeasurements {
            height_cm:         height,
            chest_cm:          (88.0 + variance).max(70.0),
            waist_cm:          (70.0 + variance * 0.8).max(55.0),
            hip_cm:            (94.0 + variance).max(75.0),
            shoulder_width_cm: (38.0 + variance * 0.3).max(30.0),
            inseam_cm:         (74.0 + (height - 165.0) * 0.4),
        }
    } else {
        tokio::task::spawn_blocking(move || {
            let mut pose = state.pose_estimator.as_ref()
                .expect("pose estimator not loaded").lock().expect("pose mutex poisoned");
            let mut seg  = state.segmentor.as_ref()
                .expect("segmentor not loaded").lock().expect("segmentor mutex poisoned");
            measure::extract(&mut pose, &mut seg, &front, &side, height)
        })
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({ "error": e.to_string() }))))?
        .map_err(|e| (StatusCode::UNPROCESSABLE_ENTITY,  Json(json!({ "error": e.to_string() }))))?
    };

    Ok(Json(json!({
        "height_cm":         measurements.height_cm,
        "chest_cm":          measurements.chest_cm,
        "waist_cm":          measurements.waist_cm,
        "hip_cm":            measurements.hip_cm,
        "shoulder_width_cm": measurements.shoulder_width_cm,
        "inseam_cm":         measurements.inseam_cm,
    })))
}
