/// Reads a CSV file and produces a LibCutRequest.

use libcut_core::contracts::{LibCutPartRequest, LibCutRequest, LibCutSheetRequest};
use libcut_core::error::{LibCutValidationError, LibCutValidationIssue};
use std::fs;
use std::path::Path;

pub fn read(path: &Path) -> Result<LibCutRequest, LibCutValidationError> {
    let content = fs::read_to_string(path).map_err(|e| {
        LibCutValidationError::new(
            format!("Failed to read file: {}", e),
            vec![LibCutValidationIssue {
                path: "csv".to_string(),
                message: format!("Failed to read file: {}", e),
            }],
        )
    })?;

    parse_csv(&content)
}

pub fn parse_csv(content: &str) -> Result<LibCutRequest, LibCutValidationError> {
    let mut parts = Vec::new();
    let mut line_number = 0;

    for raw_line in content.lines() {
        line_number += 1;
        let line = raw_line.trim();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }

        let columns: Vec<&str> = line.split(|c| c == ';' || c == ',' || c == '\t').collect();
        if columns.len() < 3 {
            continue;
        }

        let length = columns[0].trim().parse::<i32>();
        let width = columns[1].trim().parse::<i32>();
        let qty = columns[2].trim().parse::<i32>();

        match (length, width, qty) {
            (Ok(l), Ok(w), Ok(q)) => {
                let rotate = columns.len() > 3 && columns[3].trim() == "1";
                let name = if columns.len() > 4 {
                    columns[4].trim().to_string()
                } else {
                    String::new()
                };

                parts.push(LibCutPartRequest {
                    name,
                    length: l,
                    width: w,
                    qty: q,
                    rotate,
                });
            }
            _ => {
                // Check if it looks like a header row
                let looks_like_header = columns[0].trim().eq_ignore_ascii_case("length")
                    && columns[1].trim().eq_ignore_ascii_case("width")
                    && columns[2].trim().eq_ignore_ascii_case("qty");

                if looks_like_header {
                    continue;
                }

                return Err(LibCutValidationError::new(
                    format!("Invalid CSV row at line {}: {}", line_number, line),
                    vec![LibCutValidationIssue {
                        path: format!("csv.line.{}", line_number),
                        message: format!("Invalid CSV row at line {}: {}", line_number, line),
                    }],
                ));
            }
        }
    }

    Ok(LibCutRequest {
        sheet: Some(LibCutSheetRequest::default()),
        parts: Some(parts),
        blade: None,
        padding: None,
        algorithm: None,
        options: None,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_with_header() {
        let csv = "\
# parts list
length;width;qty;rotate;name
800;400;5;1;Panel A
600;300;8;0;Panel B
";
        let request = parse_csv(csv).unwrap();
        let parts = request.parts.as_ref().unwrap();
        assert_eq!(parts.len(), 2);
        assert_eq!(parts[0].name, "Panel A");
        assert_eq!(parts[0].length, 800);
        assert_eq!(parts[0].width, 400);
        assert_eq!(parts[0].qty, 5);
        assert!(parts[0].rotate);
        assert_eq!(parts[1].name, "Panel B");
        assert!(!parts[1].rotate);
    }

    #[test]
    fn test_invalid_row_error() {
        let csv = "800;400;5;1;Panel A\nabc;def;ghi\n";
        let err = parse_csv(csv).unwrap_err();
        assert!(err.message.contains("Invalid CSV row at line 2"));
        assert_eq!(err.issues.len(), 1);
        assert!(err.issues[0].path.contains("csv.line.2"));
    }

    #[test]
    fn test_skip_comments_and_blank_lines() {
        let csv = "\n# comment\n\n800;400;3;0;Part\n";
        let request = parse_csv(csv).unwrap();
        let parts = request.parts.as_ref().unwrap();
        assert_eq!(parts.len(), 1);
    }

    #[test]
    fn test_comma_separator() {
        let csv = "800,400,5,1,Panel A\n";
        let request = parse_csv(csv).unwrap();
        let parts = request.parts.as_ref().unwrap();
        assert_eq!(parts.len(), 1);
        assert_eq!(parts[0].length, 800);
    }

    #[test]
    fn test_tab_separator() {
        let csv = "800\t400\t5\t1\tPanel A\n";
        let request = parse_csv(csv).unwrap();
        let parts = request.parts.as_ref().unwrap();
        assert_eq!(parts.len(), 1);
    }

    #[test]
    fn test_minimal_columns() {
        let csv = "800;400;5\n";
        let request = parse_csv(csv).unwrap();
        let parts = request.parts.as_ref().unwrap();
        assert_eq!(parts.len(), 1);
        assert_eq!(parts[0].name, "");
        assert!(!parts[0].rotate);
    }
}
