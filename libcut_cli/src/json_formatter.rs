/// Formats a LibCutResult as indented camelCase JSON.

use libcut_core::contracts::LibCutResult;

pub fn format(result: &LibCutResult) -> String {
    // LibCutResult already has #[serde(rename_all = "camelCase")] on it,
    // so serde_json will produce camelCase keys automatically.
    serde_json::to_string_pretty(result).unwrap_or_else(|e| format!("{{\"error\": \"{}\"}}", e))
}

#[cfg(test)]
mod tests {
    use super::*;
    use libcut_core::contracts::{LibCutSheetSize, LibCutResult};

    #[test]
    fn test_json_output_camelcase_and_indented() {
        let result = LibCutResult {
            sheet_size: LibCutSheetSize { length: 2440, width: 1220 },
            sheets_used: 1,
            parts_placed: 5,
            parts_total: 5,
            efficiency_percent: 75.3,
            sheets: vec![],
        };
        let json = format(&result);
        assert!(json.contains("\"sheetSize\""));
        assert!(json.contains("\"sheetsUsed\""));
        assert!(json.contains("\"partsPlaced\""));
        assert!(json.contains("\"efficiencyPercent\""));
        // Verify it is indented (contains newline + spaces)
        assert!(json.contains('\n'));
    }
}
