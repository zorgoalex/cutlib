use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct LibCutValidationIssue {
    pub path: String,
    pub message: String,
}

#[derive(Debug, Clone, thiserror::Error)]
#[error("{message}")]
pub struct LibCutValidationError {
    pub message: String,
    pub issues: Vec<LibCutValidationIssue>,
}

impl LibCutValidationError {
    pub fn new(message: impl Into<String>, issues: Vec<LibCutValidationIssue>) -> Self {
        Self {
            message: message.into(),
            issues,
        }
    }

    pub fn single(path: impl Into<String>, message: impl Into<String>) -> Self {
        Self {
            message: "Request validation failed.".into(),
            issues: vec![LibCutValidationIssue {
                path: path.into(),
                message: message.into(),
            }],
        }
    }

    pub fn to_error_dictionary(&self) -> HashMap<String, Vec<String>> {
        let mut map: HashMap<String, Vec<String>> = HashMap::new();
        for issue in &self.issues {
            map.entry(issue.path.clone())
                .or_default()
                .push(issue.message.clone());
        }
        map
    }
}
