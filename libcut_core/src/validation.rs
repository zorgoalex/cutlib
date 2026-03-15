use crate::contracts::{LibCutAlgorithm, LibCutRequest};
use crate::error::{LibCutValidationError, LibCutValidationIssue};

pub struct LibCutRequestValidator;

impl LibCutRequestValidator {
    pub fn validate(request: &LibCutRequest) -> Result<(), LibCutValidationError> {
        let mut issues = Vec::new();

        // Sheet validation
        match request.sheet_ref() {
            None => {
                issues.push(LibCutValidationIssue {
                    path: "sheet".into(),
                    message: "Sheet is required.".into(),
                });
            }
            Some(sheet) => {
                if sheet.length <= 0 {
                    issues.push(LibCutValidationIssue {
                        path: "sheet.length".into(),
                        message: "Sheet length must be greater than zero.".into(),
                    });
                }
                if sheet.width <= 0 {
                    issues.push(LibCutValidationIssue {
                        path: "sheet.width".into(),
                        message: "Sheet width must be greater than zero.".into(),
                    });
                }
            }
        }

        // Parts validation
        let parts = request.parts_list();
        if parts.is_empty() {
            issues.push(LibCutValidationIssue {
                path: "parts".into(),
                message: "At least one part is required.".into(),
            });
        } else {
            for (i, part) in parts.iter().enumerate() {
                if part.length <= 0 {
                    issues.push(LibCutValidationIssue {
                        path: format!("parts[{}].length", i),
                        message: "Part length must be greater than zero.".into(),
                    });
                }
                if part.width <= 0 {
                    issues.push(LibCutValidationIssue {
                        path: format!("parts[{}].width", i),
                        message: "Part width must be greater than zero.".into(),
                    });
                }
                if part.qty <= 0 {
                    issues.push(LibCutValidationIssue {
                        path: format!("parts[{}].qty", i),
                        message: "Part quantity must be greater than zero.".into(),
                    });
                }
            }
        }

        // Blade / Padding validation
        if let Some(blade) = request.blade.or_else(|| {
            request.options.as_ref().and_then(|o| o.blade)
        }) {
            if blade < 0 {
                issues.push(LibCutValidationIssue {
                    path: "blade".into(),
                    message: "Blade width must not be negative.".into(),
                });
            }
        }
        if let Some(padding) = request.padding.or_else(|| {
            request.options.as_ref().and_then(|o| o.padding)
        }) {
            if padding < 0 {
                issues.push(LibCutValidationIssue {
                    path: "padding".into(),
                    message: "Padding must not be negative.".into(),
                });
            }
        }

        // Algorithm validation
        let alg_str = request
            .algorithm
            .as_deref()
            .or_else(|| request.options.as_ref().and_then(|o| o.algorithm.as_deref()));
        if let Some(s) = alg_str {
            if LibCutAlgorithm::parse(s).is_err() {
                issues.push(LibCutValidationIssue {
                    path: "algorithm".into(),
                    message: format!(
                        "Unknown algorithm '{}'. Allowed values: length, width, optimal.",
                        s
                    ),
                });
            }
        }

        if issues.is_empty() {
            Ok(())
        } else {
            Err(LibCutValidationError::new(
                "Request validation failed.",
                issues,
            ))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::contracts::*;

    fn create_valid_request() -> LibCutRequest {
        LibCutRequest {
            sheet: Some(LibCutSheetRequest {
                length: 2440,
                width: 1220,
            }),
            parts: Some(vec![
                LibCutPartRequest {
                    name: "Panel A".into(),
                    length: 800,
                    width: 400,
                    qty: 5,
                    rotate: true,
                },
                LibCutPartRequest {
                    name: "Panel B".into(),
                    length: 600,
                    width: 300,
                    qty: 8,
                    rotate: true,
                },
                LibCutPartRequest {
                    name: "Shelf".into(),
                    length: 500,
                    width: 250,
                    qty: 4,
                    rotate: false,
                },
                LibCutPartRequest {
                    name: "Door".into(),
                    length: 1200,
                    width: 600,
                    qty: 2,
                    rotate: true,
                },
            ]),
            blade: Some(4),
            padding: Some(10),
            algorithm: Some("optimal".into()),
            options: None,
        }
    }

    #[test]
    fn validate_allows_valid_request() {
        let req = create_valid_request();
        assert!(LibCutRequestValidator::validate(&req).is_ok());
    }

    #[test]
    fn validate_reports_field_level_errors() {
        let req = LibCutRequest {
            sheet: Some(LibCutSheetRequest {
                length: 0,
                width: -5,
            }),
            parts: Some(vec![LibCutPartRequest {
                name: "Bad".into(),
                length: 0,
                width: 100,
                qty: 0,
                rotate: true,
            }]),
            blade: None,
            padding: None,
            algorithm: Some("broken".into()),
            options: None,
        };

        let err = LibCutRequestValidator::validate(&req).unwrap_err();
        assert_eq!(err.message, "Request validation failed.");

        let dict = err.to_error_dictionary();
        assert!(dict.contains_key("sheet.length"));
        assert!(dict.contains_key("sheet.width"));
        assert!(dict.contains_key("parts[0].length"));
        assert!(dict.contains_key("parts[0].qty"));
        assert!(dict.contains_key("algorithm"));
        assert!(dict["algorithm"][0].contains("Allowed values: length, width, optimal."));
    }

    #[test]
    fn validate_requires_sheet() {
        let req = LibCutRequest {
            sheet: None,
            parts: Some(vec![LibCutPartRequest {
                name: "A".into(),
                length: 100,
                width: 50,
                qty: 1,
                rotate: true,
            }]),
            ..Default::default()
        };

        let err = LibCutRequestValidator::validate(&req).unwrap_err();
        let dict = err.to_error_dictionary();
        assert!(dict.contains_key("sheet"));
        assert!(dict["sheet"][0].contains("required"));
    }

    #[test]
    fn validate_requires_parts() {
        let req = LibCutRequest {
            sheet: Some(LibCutSheetRequest {
                length: 2440,
                width: 1220,
            }),
            parts: None,
            ..Default::default()
        };

        let err = LibCutRequestValidator::validate(&req).unwrap_err();
        let dict = err.to_error_dictionary();
        assert!(dict.contains_key("parts"));
    }

    #[test]
    fn validate_rejects_negative_blade() {
        let mut req = create_valid_request();
        req.blade = Some(-1);

        let err = LibCutRequestValidator::validate(&req).unwrap_err();
        let dict = err.to_error_dictionary();
        assert!(dict.contains_key("blade"));
    }
}
