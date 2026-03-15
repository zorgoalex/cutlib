use serde::{Deserialize, Serialize};

use crate::error::LibCutValidationError;

// --- Algorithm ---

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LibCutAlgorithm {
    Length = 1,
    Width = 2,
    Optimal = 3,
}

impl LibCutAlgorithm {
    pub fn parse(s: &str) -> Result<Self, LibCutValidationError> {
        match s.trim().to_lowercase().as_str() {
            "length" | "l" | "1" => Ok(LibCutAlgorithm::Length),
            "width" | "w" | "2" => Ok(LibCutAlgorithm::Width),
            "optimal" | "opt" | "3" => Ok(LibCutAlgorithm::Optimal),
            _ => Err(LibCutValidationError::single(
                "algorithm",
                format!(
                    "Unknown algorithm '{}'. Allowed values: length, width, optimal.",
                    s
                ),
            )),
        }
    }
}

// --- Request DTOs ---

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct LibCutSheetRequest {
    #[serde(alias = "lengthMm", alias = "Length", alias = "LENGTH")]
    pub length: i32,
    #[serde(alias = "widthMm", alias = "Width", alias = "WIDTH")]
    pub width: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LibCutPartRequest {
    #[serde(default)]
    pub name: String,
    #[serde(alias = "lengthMm", alias = "Length", alias = "LENGTH")]
    pub length: i32,
    #[serde(alias = "widthMm", alias = "Width", alias = "WIDTH")]
    pub width: i32,
    #[serde(alias = "quantity", alias = "Qty", alias = "QTY", default = "default_qty")]
    pub qty: i32,
    #[serde(
        alias = "canRotate",
        alias = "can_rotate",
        alias = "Rotate",
        alias = "ROTATE",
        default = "default_rotate"
    )]
    pub rotate: bool,
}

fn default_qty() -> i32 {
    1
}

fn default_rotate() -> bool {
    true
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct LibCutOptions {
    #[serde(alias = "bladeMm", alias = "Blade")]
    pub blade: Option<i32>,
    #[serde(alias = "paddingMm", alias = "Padding")]
    pub padding: Option<i32>,
    #[serde(alias = "Algorithm")]
    pub algorithm: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct LibCutRequest {
    #[serde(default)]
    pub sheet: Option<LibCutSheetRequest>,
    #[serde(default)]
    pub parts: Option<Vec<LibCutPartRequest>>,
    #[serde(default)]
    pub blade: Option<i32>,
    #[serde(default)]
    pub padding: Option<i32>,
    #[serde(default)]
    pub algorithm: Option<String>,
    #[serde(default)]
    pub options: Option<LibCutOptions>,
}

// --- Resolved Options ---

#[derive(Debug, Clone)]
pub struct LibCutResolvedOptions {
    pub blade_mm: i32,
    pub padding_mm: i32,
    pub algorithm: LibCutAlgorithm,
}

impl LibCutRequest {
    pub fn resolve_options(&self) -> Result<LibCutResolvedOptions, LibCutValidationError> {
        let opts = self.options.as_ref();

        let blade = self
            .blade
            .or_else(|| opts.and_then(|o| o.blade))
            .unwrap_or(4);

        let padding = self
            .padding
            .or_else(|| opts.and_then(|o| o.padding))
            .unwrap_or(0);

        let alg_str = self
            .algorithm
            .as_deref()
            .or_else(|| opts.and_then(|o| o.algorithm.as_deref()));

        let algorithm = match alg_str {
            Some(s) => LibCutAlgorithm::parse(s)?,
            None => LibCutAlgorithm::Optimal,
        };

        Ok(LibCutResolvedOptions {
            blade_mm: blade,
            padding_mm: padding,
            algorithm,
        })
    }

    pub fn parts_list(&self) -> &[LibCutPartRequest] {
        match &self.parts {
            Some(p) => p,
            None => &[],
        }
    }

    pub fn sheet_ref(&self) -> Option<&LibCutSheetRequest> {
        self.sheet.as_ref()
    }
}

// --- Response DTOs ---

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LibCutResult {
    pub sheet_size: LibCutSheetSize,
    pub sheets_used: i32,
    pub parts_placed: i32,
    pub parts_total: i32,
    pub efficiency_percent: f64,
    pub sheets: Vec<LibCutSheetResult>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LibCutSheetSize {
    pub length: i32,
    pub width: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LibCutSheetResult {
    pub sheet: i32,
    pub parts: Vec<LibCutPartPlacement>,
    pub offcuts: Vec<LibCutOffcut>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LibCutPartPlacement {
    pub name: String,
    pub length: i32,
    pub width: i32,
    pub x: i32,
    pub y: i32,
    pub rotated: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LibCutOffcut {
    pub length: i32,
    pub width: i32,
    pub x: i32,
    pub y: i32,
}

// --- Tests ---

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_algorithm_parser_valid_values() {
        assert_eq!(LibCutAlgorithm::parse("length").unwrap(), LibCutAlgorithm::Length);
        assert_eq!(LibCutAlgorithm::parse("l").unwrap(), LibCutAlgorithm::Length);
        assert_eq!(LibCutAlgorithm::parse("1").unwrap(), LibCutAlgorithm::Length);
        assert_eq!(LibCutAlgorithm::parse("width").unwrap(), LibCutAlgorithm::Width);
        assert_eq!(LibCutAlgorithm::parse("w").unwrap(), LibCutAlgorithm::Width);
        assert_eq!(LibCutAlgorithm::parse("2").unwrap(), LibCutAlgorithm::Width);
        assert_eq!(LibCutAlgorithm::parse("optimal").unwrap(), LibCutAlgorithm::Optimal);
        assert_eq!(LibCutAlgorithm::parse("opt").unwrap(), LibCutAlgorithm::Optimal);
        assert_eq!(LibCutAlgorithm::parse("3").unwrap(), LibCutAlgorithm::Optimal);
    }

    #[test]
    fn test_algorithm_parser_case_insensitive() {
        assert_eq!(LibCutAlgorithm::parse("LENGTH").unwrap(), LibCutAlgorithm::Length);
        assert_eq!(LibCutAlgorithm::parse("Optimal").unwrap(), LibCutAlgorithm::Optimal);
    }

    #[test]
    fn test_algorithm_parser_invalid() {
        let err = LibCutAlgorithm::parse("broken").unwrap_err();
        assert!(err.message.contains("validation failed"));
        let dict = err.to_error_dictionary();
        assert!(dict.contains_key("algorithm"));
        assert!(dict["algorithm"][0].contains("Allowed values: length, width, optimal."));
    }

    #[test]
    fn test_request_json_roundtrip() {
        let json = r#"{
            "sheet": { "length": 2440, "width": 1220 },
            "blade": 4,
            "padding": 10,
            "algorithm": "optimal",
            "parts": [
                { "length": 800, "width": 400, "qty": 5, "rotate": true, "name": "Panel A" }
            ]
        }"#;

        let req: LibCutRequest = serde_json::from_str(json).unwrap();
        assert_eq!(req.sheet.as_ref().unwrap().length, 2440);
        assert_eq!(req.sheet.as_ref().unwrap().width, 1220);
        assert_eq!(req.blade, Some(4));
        assert_eq!(req.padding, Some(10));
        assert_eq!(req.parts_list().len(), 1);
        assert_eq!(req.parts_list()[0].name, "Panel A");
        assert_eq!(req.parts_list()[0].length, 800);
        assert_eq!(req.parts_list()[0].qty, 5);
        assert!(req.parts_list()[0].rotate);
    }

    #[test]
    fn test_request_defaults() {
        let json = r#"{ "parts": [{ "length": 100, "width": 50 }] }"#;
        let req: LibCutRequest = serde_json::from_str(json).unwrap();
        assert_eq!(req.parts_list()[0].qty, 1);
        assert!(req.parts_list()[0].rotate);
        assert_eq!(req.parts_list()[0].name, "");
    }

    #[test]
    fn test_resolve_options_defaults() {
        let req = LibCutRequest::default();
        let opts = req.resolve_options().unwrap();
        assert_eq!(opts.blade_mm, 4);
        assert_eq!(opts.padding_mm, 0);
        assert_eq!(opts.algorithm, LibCutAlgorithm::Optimal);
    }

    #[test]
    fn test_resolve_options_root_overrides_options() {
        let json = r#"{
            "blade": 7,
            "algorithm": "length",
            "options": { "blade": 3, "algorithm": "width" }
        }"#;
        let req: LibCutRequest = serde_json::from_str(json).unwrap();
        let opts = req.resolve_options().unwrap();
        assert_eq!(opts.blade_mm, 7);
        assert_eq!(opts.algorithm, LibCutAlgorithm::Length);
    }

    #[test]
    fn test_resolve_options_falls_back_to_options() {
        let json = r#"{ "options": { "blade": 5, "padding": 3, "algorithm": "width" } }"#;
        let req: LibCutRequest = serde_json::from_str(json).unwrap();
        let opts = req.resolve_options().unwrap();
        assert_eq!(opts.blade_mm, 5);
        assert_eq!(opts.padding_mm, 3);
        assert_eq!(opts.algorithm, LibCutAlgorithm::Width);
    }

    #[test]
    fn test_result_serialization_camelcase() {
        let result = LibCutResult {
            sheet_size: LibCutSheetSize { length: 2440, width: 1220 },
            sheets_used: 1,
            parts_placed: 5,
            parts_total: 5,
            efficiency_percent: 75.3,
            sheets: vec![],
        };
        let json = serde_json::to_string(&result).unwrap();
        assert!(json.contains("sheetSize"));
        assert!(json.contains("sheetsUsed"));
        assert!(json.contains("partsPlaced"));
        assert!(json.contains("partsTotal"));
        assert!(json.contains("efficiencyPercent"));
    }
}
