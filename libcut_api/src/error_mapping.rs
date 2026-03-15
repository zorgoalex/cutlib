use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::Json;
use libcut_core::error::LibCutValidationError;
use serde::Serialize;
use std::collections::HashMap;

#[derive(Debug, Serialize)]
pub struct ProblemDetail {
    #[serde(rename = "type")]
    pub problem_type: String,
    pub title: String,
    pub status: u16,
    pub detail: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub errors: Option<HashMap<String, Vec<String>>>,
    #[serde(rename = "traceId")]
    pub trace_id: String,
}

impl IntoResponse for ProblemDetail {
    fn into_response(self) -> Response {
        let status =
            StatusCode::from_u16(self.status).unwrap_or(StatusCode::INTERNAL_SERVER_ERROR);
        let mut response = Json(&self).into_response();
        *response.status_mut() = status;
        response.headers_mut().insert(
            axum::http::header::CONTENT_TYPE,
            "application/problem+json"
                .parse()
                .expect("valid header value"),
        );
        response
    }
}

fn new_trace_id() -> String {
    uuid::Uuid::new_v4().to_string()
}

pub fn validation_problem(error: &LibCutValidationError) -> ProblemDetail {
    ProblemDetail {
        problem_type: "https://datatracker.ietf.org/doc/html/rfc9110#section-15.5.1".into(),
        title: "Invalid cut optimization request.".into(),
        status: 400,
        detail: "Correct the fields listed in the errors section and retry the request.".into(),
        errors: Some(error.to_error_dictionary()),
        trace_id: new_trace_id(),
    }
}

pub fn invalid_json_problem(message: &str) -> ProblemDetail {
    let mut errors = HashMap::new();
    errors.insert(
        "json".to_string(),
        vec![format!("Malformed JSON: {}", message)],
    );
    ProblemDetail {
        problem_type: "https://datatracker.ietf.org/doc/html/rfc9110#section-15.5.1".into(),
        title: "Invalid cut optimization request.".into(),
        status: 400,
        detail: "Correct the fields listed in the errors section and retry the request.".into(),
        errors: Some(errors),
        trace_id: new_trace_id(),
    }
}

pub fn empty_body_problem() -> ProblemDetail {
    let mut errors = HashMap::new();
    errors.insert(
        "json".to_string(),
        vec!["Request body is required.".to_string()],
    );
    ProblemDetail {
        problem_type: "https://datatracker.ietf.org/doc/html/rfc9110#section-15.5.1".into(),
        title: "Invalid cut optimization request.".into(),
        status: 400,
        detail: "Correct the fields listed in the errors section and retry the request.".into(),
        errors: Some(errors),
        trace_id: new_trace_id(),
    }
}

pub fn unexpected_failure(_message: &str) -> ProblemDetail {
    ProblemDetail {
        problem_type: "https://datatracker.ietf.org/doc/html/rfc9110#section-15.6.1".into(),
        title: "Cut optimization failed.".into(),
        status: 500,
        detail: "Unexpected server error while processing the optimization request.".into(),
        errors: None,
        trace_id: new_trace_id(),
    }
}
