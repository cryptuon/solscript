use axum::{http::StatusCode, Json};
use serde::{Deserialize, Serialize};

use crate::sandbox;

#[derive(Deserialize)]
pub struct BuildRequest {
    pub source: String,
}

#[derive(Serialize)]
pub struct BuildResponse {
    pub success: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bytecode: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub idl: Option<String>,
    pub errors: Vec<BuildError>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub build_time_secs: Option<f64>,
}

#[derive(Serialize)]
pub struct BuildError {
    pub phase: String,
    pub message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end: Option<usize>,
}

#[derive(Serialize)]
pub struct HealthResponse {
    pub status: String,
    pub version: String,
}

pub async fn health() -> Json<HealthResponse> {
    Json(HealthResponse {
        status: "ok".to_string(),
        version: env!("CARGO_PKG_VERSION").to_string(),
    })
}

pub async fn build(Json(payload): Json<BuildRequest>) -> (StatusCode, Json<BuildResponse>) {
    // Validate source size (100KB max)
    if payload.source.len() > 100_000 {
        return (
            StatusCode::BAD_REQUEST,
            Json(BuildResponse {
                success: false,
                bytecode: None,
                idl: None,
                errors: vec![BuildError {
                    phase: "validation".to_string(),
                    message: "Source code exceeds maximum size of 100KB".to_string(),
                    start: None,
                    end: None,
                }],
                build_time_secs: None,
            }),
        );
    }

    match sandbox::compile_source(&payload.source).await {
        Ok(result) => (StatusCode::OK, Json(result)),
        Err(response) => (StatusCode::INTERNAL_SERVER_ERROR, Json(response)),
    }
}
