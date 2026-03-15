use axum::extract::State;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::Json;
use libcut_core::contracts::LibCutRequest;
use libcut_core::engine::LibCutEngine;
use serde::Serialize;

use crate::concurrency::ConcurrencyGate;
use crate::error_mapping;

#[derive(Clone)]
pub struct AppState {
    pub gate: ConcurrencyGate,
}

#[derive(Serialize)]
pub struct HealthResponse {
    pub status: String,
    pub service: String,
}

pub async fn health() -> Json<HealthResponse> {
    Json(HealthResponse {
        status: "ok".into(),
        service: "LibCut.Api".into(),
    })
}

pub async fn optimize(
    State(state): State<AppState>,
    body: axum::body::Bytes,
) -> impl IntoResponse {
    if body.is_empty() {
        return error_mapping::empty_body_problem().into_response();
    }

    let request: LibCutRequest = match serde_json::from_slice(&body) {
        Ok(r) => r,
        Err(e) => {
            return error_mapping::invalid_json_problem(&e.to_string()).into_response();
        }
    };

    let _permit = state.gate.acquire().await;

    let result = tokio::task::spawn_blocking(move || LibCutEngine::optimize(&request)).await;

    match result {
        Ok(Ok(cut_result)) => (StatusCode::OK, Json(cut_result)).into_response(),
        Ok(Err(validation_error)) => {
            error_mapping::validation_problem(&validation_error).into_response()
        }
        Err(e) => error_mapping::unexpected_failure(&e.to_string()).into_response(),
    }
}
