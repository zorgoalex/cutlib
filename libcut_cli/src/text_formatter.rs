/// Formats a LibCutResult as human-readable text.

use libcut_core::contracts::{LibCutRequest, LibCutResult};

pub fn format(request: &LibCutRequest, result: &LibCutResult) -> String {
    let mut sb = String::new();

    sb.push_str("=== CUTTING RESULTS ===\n");
    sb.push_str(&format!(
        "Sheet: {} x {} mm\n",
        result.sheet_size.length, result.sheet_size.width
    ));
    sb.push_str(&format!("Sheets used: {}\n", result.sheets_used));
    sb.push_str(&format!(
        "Parts placed: {} / {}\n",
        result.parts_placed, result.parts_total
    ));
    sb.push_str(&format!(
        "Material efficiency: {:.1}%\n",
        result.efficiency_percent
    ));
    sb.push('\n');
    sb.push_str("--- Parts placement ---\n");

    let parts_list = request.parts_list();

    for requested_part in parts_list {
        let name = if requested_part.name.is_empty() {
            "Part"
        } else {
            &requested_part.name
        };

        // Collect placements matching this requested part
        let mut placements: Vec<(i32, &libcut_core::contracts::LibCutPartPlacement)> = Vec::new();
        for sheet_result in &result.sheets {
            for placement in &sheet_result.parts {
                if placement.name == name
                    && placement.length == requested_part.length
                    && placement.width == requested_part.width
                {
                    placements.push((sheet_result.sheet, placement));
                }
            }
        }

        sb.push_str(&format!(
            "{}: {}x{} mm, placed {}/{}\n",
            name,
            requested_part.length,
            requested_part.width,
            placements.len(),
            requested_part.qty
        ));

        for (sheet_num, placement) in &placements {
            let rotated = if placement.rotated { " [rotated]" } else { "" };
            sb.push_str(&format!(
                "    Sheet {}: ({}, {}){}\n",
                sheet_num, placement.x, placement.y, rotated
            ));
        }
    }

    // Offcuts
    let has_offcuts = result.sheets.iter().any(|s| !s.offcuts.is_empty());
    if has_offcuts {
        sb.push('\n');
        sb.push_str("--- Waste/offcuts ---\n");
        for sheet_result in &result.sheets {
            for offcut in &sheet_result.offcuts {
                sb.push_str(&format!(
                    "  Sheet {}: {}x{} mm at ({}, {})\n",
                    sheet_result.sheet, offcut.length, offcut.width, offcut.x, offcut.y
                ));
            }
        }
    }

    sb
}

#[cfg(test)]
mod tests {
    use super::*;
    use libcut_core::contracts::*;

    fn sample_request_and_result() -> (LibCutRequest, LibCutResult) {
        let request = LibCutRequest {
            sheet: Some(LibCutSheetRequest { length: 2440, width: 1220 }),
            parts: Some(vec![LibCutPartRequest {
                name: "Panel A".to_string(),
                length: 800,
                width: 400,
                qty: 2,
                rotate: false,
            }]),
            blade: Some(4),
            padding: None,
            algorithm: None,
            options: None,
        };

        let result = LibCutResult {
            sheet_size: LibCutSheetSize { length: 2440, width: 1220 },
            sheets_used: 1,
            parts_placed: 2,
            parts_total: 2,
            efficiency_percent: 50.0,
            sheets: vec![LibCutSheetResult {
                sheet: 1,
                parts: vec![
                    LibCutPartPlacement {
                        name: "Panel A".to_string(),
                        length: 800,
                        width: 400,
                        x: 10,
                        y: 10,
                        rotated: false,
                    },
                    LibCutPartPlacement {
                        name: "Panel A".to_string(),
                        length: 800,
                        width: 400,
                        x: 814,
                        y: 10,
                        rotated: false,
                    },
                ],
                offcuts: vec![LibCutOffcut {
                    length: 100,
                    width: 200,
                    x: 1618,
                    y: 10,
                }],
            }],
        };

        (request, result)
    }

    #[test]
    fn test_text_output_contains_header() {
        let (req, res) = sample_request_and_result();
        let output = format(&req, &res);
        assert!(output.contains("=== CUTTING RESULTS ==="));
        assert!(output.contains("Sheet: 2440 x 1220 mm"));
        assert!(output.contains("Sheets used: 1"));
        assert!(output.contains("Parts placed: 2 / 2"));
        assert!(output.contains("Material efficiency: 50.0%"));
    }

    #[test]
    fn test_text_output_contains_parts() {
        let (req, res) = sample_request_and_result();
        let output = format(&req, &res);
        assert!(output.contains("Panel A: 800x400 mm, placed 2/2"));
        assert!(output.contains("Sheet 1: (10, 10)"));
        assert!(output.contains("Sheet 1: (814, 10)"));
    }

    #[test]
    fn test_text_output_contains_offcuts() {
        let (req, res) = sample_request_and_result();
        let output = format(&req, &res);
        assert!(output.contains("--- Waste/offcuts ---"));
        assert!(output.contains("Sheet 1: 100x200 mm at (1618, 10)"));
    }
}
