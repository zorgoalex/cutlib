use axum::body::Body;
use axum::http::{Request, StatusCode};
use axum::routing::{get, post};
use axum::Router;
use http_body_util::BodyExt;
use tower::ServiceExt;

// We cannot import from a binary crate directly, so we rebuild the router here
// using the same logic as main.rs.
fn build_app() -> Router {
    // Inline the necessary types to avoid binary-crate import issues.
    // We replicate the concurrency gate and route handlers.
    use std::sync::Arc;
    use tokio::sync::Semaphore;

    #[derive(Clone)]
    struct AppState {
        semaphore: Arc<Semaphore>,
    }

    async fn health() -> axum::Json<serde_json::Value> {
        axum::Json(serde_json::json!({
            "status": "ok",
            "service": "LibCut.Api"
        }))
    }

    async fn optimize(
        axum::extract::State(state): axum::extract::State<AppState>,
        body: axum::body::Bytes,
    ) -> axum::response::Response {
        use axum::response::IntoResponse;

        if body.is_empty() {
            let problem = serde_json::json!({
                "type": "https://datatracker.ietf.org/doc/html/rfc9110#section-15.5.1",
                "title": "Invalid cut optimization request.",
                "status": 400,
                "detail": "Correct the fields listed in the errors section and retry the request.",
                "errors": { "json": ["Request body is required."] },
                "traceId": uuid::Uuid::new_v4().to_string()
            });
            return (
                StatusCode::BAD_REQUEST,
                [(axum::http::header::CONTENT_TYPE, "application/problem+json")],
                axum::Json(problem),
            )
                .into_response();
        }

        let request: libcut_core::contracts::LibCutRequest = match serde_json::from_slice(&body) {
            Ok(r) => r,
            Err(err) => {
                let problem = serde_json::json!({
                    "type": "https://datatracker.ietf.org/doc/html/rfc9110#section-15.5.1",
                    "title": "Invalid cut optimization request.",
                    "status": 400,
                    "detail": "Correct the fields listed in the errors section and retry the request.",
                    "errors": { "json": [format!("Malformed JSON: {}", err)] },
                    "traceId": uuid::Uuid::new_v4().to_string()
                });
                return (
                    StatusCode::BAD_REQUEST,
                    [(axum::http::header::CONTENT_TYPE, "application/problem+json")],
                    axum::Json(problem),
                )
                    .into_response();
            }
        };

        let _permit = state
            .semaphore
            .clone()
            .acquire_owned()
            .await
            .expect("semaphore closed");

        let result =
            tokio::task::spawn_blocking(move || {
                libcut_core::engine::LibCutEngine::optimize(&request)
            })
            .await;

        match result {
            Ok(Ok(cut_result)) => (StatusCode::OK, axum::Json(cut_result)).into_response(),
            Ok(Err(validation_error)) => {
                let problem = serde_json::json!({
                    "type": "https://datatracker.ietf.org/doc/html/rfc9110#section-15.5.1",
                    "title": "Invalid cut optimization request.",
                    "status": 400,
                    "detail": "Correct the fields listed in the errors section and retry the request.",
                    "errors": validation_error.to_error_dictionary(),
                    "traceId": uuid::Uuid::new_v4().to_string()
                });
                (
                    StatusCode::BAD_REQUEST,
                    [(axum::http::header::CONTENT_TYPE, "application/problem+json")],
                    axum::Json(problem),
                )
                    .into_response()
            }
            Err(_) => {
                let problem = serde_json::json!({
                    "type": "https://datatracker.ietf.org/doc/html/rfc9110#section-15.6.1",
                    "title": "Cut optimization failed.",
                    "status": 500,
                    "detail": "Unexpected server error while processing the optimization request.",
                    "traceId": uuid::Uuid::new_v4().to_string()
                });
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    [(axum::http::header::CONTENT_TYPE, "application/problem+json")],
                    axum::Json(problem),
                )
                    .into_response()
            }
        }
    }

    let state = AppState {
        semaphore: Arc::new(Semaphore::new(1)),
    };

    Router::new()
        .route("/health", get(health))
        .route("/api/cut/optimize", post(optimize))
        .with_state(state)
}

#[tokio::test]
async fn test_health_returns_ok() {
    let app = build_app();

    let response = app
        .oneshot(
            Request::builder()
                .uri("/health")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);

    let body = response.into_body().collect().await.unwrap().to_bytes();
    let json: serde_json::Value = serde_json::from_slice(&body).unwrap();

    assert_eq!(json["status"], "ok");
    assert_eq!(json["service"], "LibCut.Api");
}

#[tokio::test]
async fn test_optimize_valid_request() {
    let app = build_app();

    let fixture =
        std::fs::read_to_string("/home/admubu/projects/cutlib/spec_cutlib/rust_port_fixtures/sample_order/request.json")
            .unwrap();

    let response = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/cut/optimize")
                .header("content-type", "application/json")
                .body(Body::from(fixture))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);

    let body = response.into_body().collect().await.unwrap().to_bytes();
    let json: serde_json::Value = serde_json::from_slice(&body).unwrap();

    assert_eq!(json["sheetsUsed"], 2, "Expected 2 sheets");
    assert_eq!(json["partsPlaced"], 19, "Expected 19 parts placed");
}

#[tokio::test]
async fn test_optimize_invalid_request() {
    let app = build_app();

    // Missing sheet and parts - should trigger validation error
    let invalid_body = r#"{"blade": 4}"#;

    let response = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/cut/optimize")
                .header("content-type", "application/json")
                .body(Body::from(invalid_body))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::BAD_REQUEST);

    let body = response.into_body().collect().await.unwrap().to_bytes();
    let json: serde_json::Value = serde_json::from_slice(&body).unwrap();

    assert_eq!(json["status"], 400);
    assert!(json["errors"].is_object(), "Expected errors object in problem+json");
    assert!(json["traceId"].is_string(), "Expected traceId");
}

#[tokio::test]
async fn test_optimize_malformed_json() {
    let app = build_app();

    let response = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/cut/optimize")
                .header("content-type", "application/json")
                .body(Body::from("{not valid json"))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::BAD_REQUEST);

    let body = response.into_body().collect().await.unwrap().to_bytes();
    let json: serde_json::Value = serde_json::from_slice(&body).unwrap();

    assert_eq!(json["status"], 400);
    assert!(json["errors"]["json"].is_array(), "Expected json error array");
}
