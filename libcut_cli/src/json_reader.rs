/// Reads a JSON file and produces a LibCutRequest.

use libcut_core::contracts::LibCutRequest;
use libcut_core::error::{LibCutValidationError, LibCutValidationIssue};
use std::fs;
use std::path::Path;

pub fn read(path: &Path) -> Result<LibCutRequest, LibCutValidationError> {
    let content = fs::read_to_string(path).map_err(|e| {
        LibCutValidationError::new(
            format!("Failed to read file: {}", e),
            vec![LibCutValidationIssue {
                path: "json".to_string(),
                message: format!("Failed to read file: {}", e),
            }],
        )
    })?;

    parse_json(&content)
}

pub fn parse_json(content: &str) -> Result<LibCutRequest, LibCutValidationError> {
    let mut request: LibCutRequest = serde_json::from_str(content).map_err(|e| {
        LibCutValidationError::new(
            "Input JSON is invalid.".to_string(),
            vec![LibCutValidationIssue {
                path: "json".to_string(),
                message: e.to_string(),
            }],
        )
    })?;

    // Ensure collections are initialized (mirror .NET behavior)
    if request.sheet.is_none() {
        request.sheet = Some(Default::default());
    }
    if request.parts.is_none() {
        request.parts = Some(Vec::new());
    }
    if request.options.is_none() {
        request.options = Some(Default::default());
    }

    Ok(request)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_missing_collections_initialized() {
        let json = r#"{ "blade": 5 }"#;
        let request = parse_json(json).unwrap();
        assert!(request.sheet.is_some());
        assert!(request.parts.is_some());
        assert!(request.options.is_some());
        assert!(request.parts.as_ref().unwrap().is_empty());
    }

    #[test]
    fn test_invalid_json_error() {
        let json = "{ not valid json }";
        let err = parse_json(json).unwrap_err();
        assert_eq!(err.message, "Input JSON is invalid.");
        assert_eq!(err.issues.len(), 1);
        assert_eq!(err.issues[0].path, "json");
    }

    #[test]
    fn test_full_json_parse() {
        let json = r#"{
            "sheet": { "length": 2440, "width": 1220 },
            "blade": 4,
            "padding": 10,
            "algorithm": "optimal",
            "parts": [
                { "length": 800, "width": 400, "qty": 5, "rotate": true, "name": "Panel A" }
            ]
        }"#;
        let request = parse_json(json).unwrap();
        assert_eq!(request.sheet.as_ref().unwrap().length, 2440);
        assert_eq!(request.sheet.as_ref().unwrap().width, 1220);
        assert_eq!(request.blade, Some(4));
        let parts = request.parts.as_ref().unwrap();
        assert_eq!(parts.len(), 1);
        assert_eq!(parts[0].name, "Panel A");
    }
}
