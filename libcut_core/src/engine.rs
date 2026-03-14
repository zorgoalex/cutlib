#![allow(non_snake_case)]

use std::collections::HashMap;
use std::sync::mpsc;
use std::thread;
use std::time::Duration;

use crate::contracts::LibCutResult;
use crate::contracts::LibCutRequest;
use crate::error::LibCutValidationError;
use crate::internal::alg_utils;
use crate::internal::algorithm_types::*;
use crate::internal::mapping::order_factory::OrderFactory;
use crate::internal::mapping::result_mapper::ResultMapper;
use crate::internal::types::*;
use crate::validation::LibCutRequestValidator;

pub struct LibCutEngine;

impl LibCutEngine {
    pub fn optimize(request: &LibCutRequest) -> Result<LibCutResult, LibCutValidationError> {
        LibCutRequestValidator::validate(request)?;
        let mut order = OrderFactory::create(request)?;
        run_cutting(&mut order);
        Ok(ResultMapper::map(&order))
    }
}

fn run_cutting(order: &mut Order) {
    // Reset state
    order.SheetCount = 0;
    order.PartsPlaced = 0;
    order.NSnips.clear();

    for part in &mut order.Parts {
        part.nPlaced = 0;
        for coord in &mut part.Coords {
            *coord = Coord::default();
        }
    }

    let mut parts = alg_utils::convert_parts_to_cparts(&order.Parts);
    let algorithm = order.parameters.Algoritm;
    let mut all_sheets: Vec<CSheet> = Vec::new();

    loop {
        let list_length = order.parameters.ListLength_mm * 10;
        let list_width = order.parameters.ListWidth_mm * 10;
        let padding = order.parameters.Padding * 10;

        if !alg_utils::fast_find_first_cpart(&parts, list_length - padding, list_width - padding) {
            break;
        }

        let mut best_sheet: Option<CSheet> = None;

        // Algorithm 1 (Length) or 3 (Optimal)
        if algorithm == 1 || algorithm == 3 {
            let results = run_parallel_variants(&parts, order, 1);
            if let Some(best) = pick_best(results, 1) {
                if best_sheet.is_none()
                    || best.Parts_Sq > best_sheet.as_ref().unwrap().Parts_Sq
                {
                    best_sheet = Some(best);
                }
            }
        }

        // Algorithm 2 (Width) or 3 (Optimal)
        if algorithm == 2 || algorithm == 3 {
            let results = run_parallel_variants(&parts, order, 2);
            if let Some(best) = pick_best(results, 2) {
                if best_sheet.is_none()
                    || best.Parts_Sq > best_sheet.as_ref().unwrap().Parts_Sq
                {
                    best_sheet = Some(best);
                }
            }
        }

        // Algorithm 3 (Optimal): also try combined Opt_Alg
        if algorithm == 3 {
            let mut optimized_parts = alg_utils::copy_cparts(&parts);
            let mut opt_alg =
                crate::internal::opt_alg::OptAlg::new();
            let mut optimized_sheet = opt_alg.get_sheet_opt_alg_2(
                &mut optimized_parts,
                list_length,
                list_width,
                order.parameters.Blade * 10,
                order.parameters.Padding * 10,
                true,  // DoublePadding
                true,  // SAME_MAX
                false, // MAX_SQ
                true,  // OPTI_ON
                true,  // TURN_ON
                3,     // ALG
            );

            if optimized_sheet.Parts_Sq > 0.0 {
                if best_sheet.is_none()
                    || optimized_sheet.Parts_Sq > best_sheet.as_ref().unwrap().Parts_Sq
                {
                    optimized_sheet.Alg = 3;
                    best_sheet = Some(optimized_sheet);
                }
            }
        }

        // Check if we found any valid sheet
        match &best_sheet {
            None => break,
            Some(s) if s.Parts_Sq <= 0.0 => break,
            _ => {}
        }

        let best_sheet = best_sheet.unwrap();

        // Mark placed parts
        mark_placed(&mut parts, &best_sheet);

        // Count how many identical sheets can be cut
        let same_count = count_same_sheets(&best_sheet, &parts);
        all_sheets.push(best_sheet.clone());

        for _ in 0..same_count {
            mark_placed(&mut parts, &best_sheet);
            all_sheets.push(best_sheet.clone());
        }

        // Remove fully placed parts
        parts.retain(|p| p.Plased < p.Qty);
    }

    // Set algorithm type on sheets that don't have it
    for sheet in &mut all_sheets {
        if sheet.Alg == 0 {
            sheet.Alg = if algorithm == 3 { 3 } else { algorithm };
        }
    }

    alg_utils::write_sheets_to_order(order, &mut all_sheets);
}

fn run_parallel_variants(parts: &[CPart], order: &Order, algorithm_type: i32) -> Vec<CSheet> {
    let variants: [(bool, bool, bool, bool); 8] = [
        (true, true, true, true),
        (true, false, true, true),
        (true, false, false, true),
        (true, false, false, false),
        (false, true, true, true),
        (false, true, false, true),
        (false, false, true, true),
        (false, false, false, true),
    ];

    let list_length = order.parameters.ListLength_mm * 10;
    let list_width = order.parameters.ListWidth_mm * 10;
    let blade = order.parameters.Blade * 10;
    let padding = order.parameters.Padding * 10;
    let parts_sq = order.PartsSq;

    let (tx, rx) = mpsc::channel::<Option<CSheet>>();

    // Base variant (thread 0)
    {
        let mut base_parts = alg_utils::copy_cparts(parts);
        let tx = tx.clone();
        let alg_type = algorithm_type;
        thread::spawn(move || {
            let result = if alg_type == 1 {
                let mut alg = crate::internal::length_alg::LengthAlg::new();
                Some(alg.get_csheet_length_cut(
                    &mut base_parts, list_length, list_width, blade, padding, true, true, true,
                ))
            } else {
                let mut alg = crate::internal::width_alg::WidthAlg::new();
                Some(alg.get_csheet_width_cut(
                    &mut base_parts, list_length, list_width, blade, padding, true, true, true,
                ))
            };
            let _ = tx.send(result);
        });
    }

    // LW16 variants (threads 1-8)
    for (same_max, max_sq, opti_on, turn_on) in &variants {
        let mut variant_parts = alg_utils::copy_cparts(parts);
        let tx = tx.clone();
        let alg_type = algorithm_type;
        let params = LW16 {
            SAME_MAX: *same_max,
            MAX_SQ: *max_sq,
            OPTI_ON: *opti_on,
            TURN_ON: *turn_on,
        };
        let psq = parts_sq;

        thread::spawn(move || {
            let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                if alg_type == 1 {
                    let mut alg = crate::internal::length2::Length2::new();
                    let (sheet, _) = alg.get_csheet_length_cut(
                        &mut variant_parts,
                        list_length,
                        list_width,
                        blade,
                        padding,
                        true,
                        &params,
                        psq as f64,
                        0.0,
                    );
                    sheet
                } else {
                    let mut alg = crate::internal::width2::Width2::new();
                    let (sheet, _) = alg.GetCSheet_WIDTH_CUT(
                        &mut variant_parts,
                        list_length,
                        list_width,
                        blade,
                        padding,
                        true,
                        params,
                        psq as f64,
                        0.0,
                    );
                    sheet
                }
            }));
            let _ = tx.send(result.unwrap_or(None));
        });
    }

    drop(tx); // Close sender so rx stops waiting after all threads done

    // Collect results with 30s timeout
    let mut results = Vec::new();
    let deadline = std::time::Instant::now() + Duration::from_secs(30);
    loop {
        let remaining = deadline.saturating_duration_since(std::time::Instant::now());
        if remaining.is_zero() {
            break;
        }
        match rx.recv_timeout(remaining) {
            Ok(Some(sheet)) => results.push(sheet),
            Ok(None) => {} // variant returned no sheet
            Err(mpsc::RecvTimeoutError::Timeout) => break,
            Err(mpsc::RecvTimeoutError::Disconnected) => break,
        }
    }

    results
}

fn pick_best(sheets: Vec<CSheet>, algorithm_type: i32) -> Option<CSheet> {
    if sheets.is_empty() {
        return None;
    }

    let mut best = sheets[0].clone();
    for sheet in sheets.iter().skip(1) {
        if sheet.Parts_Sq > best.Parts_Sq {
            best = sheet.clone();
        } else if (sheet.Parts_Sq - best.Parts_Sq).abs() < f64::EPSILON {
            if algorithm_type == 1 && sheet.Remain.W > best.Remain.W {
                best = sheet.clone();
            } else if algorithm_type == 2 && sheet.Remain.L > best.Remain.L {
                best = sheet.clone();
            }
        }
    }

    best.Alg = algorithm_type;
    Some(best)
}

fn mark_placed(parts: &mut [CPart], sheet: &CSheet) {
    for line in &sheet.Lines {
        for &part_id in &line.PartIDs {
            let index = if part_id < -1 {
                (part_id * -1 - 2) as usize
            } else {
                part_id as usize
            };
            if index < parts.len() {
                parts[index].Plased += 1;
            }
        }
    }
}

fn count_same_sheets(sheet: &CSheet, parts: &[CPart]) -> i32 {
    let mut used: HashMap<usize, i32> = HashMap::new();

    for line in &sheet.Lines {
        for &part_id in &line.PartIDs {
            let index = if part_id < -1 {
                (part_id * -1 - 2) as usize
            } else {
                part_id as usize
            };
            *used.entry(index).or_insert(0) += 1;
        }
    }

    if used.is_empty() {
        return 0;
    }

    let mut min_possible = i32::MAX;
    for (&idx, &count) in &used {
        if idx < parts.len() {
            let remaining = parts[idx].Qty - parts[idx].Plased;
            let possible = remaining / count;
            if possible < min_possible {
                min_possible = possible;
            }
        }
    }

    min_possible.max(0)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_request() -> LibCutRequest {
        serde_json::from_str(
            r#"{
                "sheet": { "length": 2440, "width": 1220 },
                "blade": 4,
                "padding": 10,
                "algorithm": "optimal",
                "parts": [
                    { "length": 800, "width": 400, "qty": 5, "rotate": true, "name": "Panel A" },
                    { "length": 600, "width": 300, "qty": 8, "rotate": true, "name": "Panel B" },
                    { "length": 500, "width": 250, "qty": 4, "rotate": false, "name": "Shelf" },
                    { "length": 1200, "width": 600, "qty": 2, "rotate": true, "name": "Door" }
                ]
            }"#,
        )
        .unwrap()
    }

    fn parts_csv_request() -> LibCutRequest {
        serde_json::from_str(
            r#"{
                "sheet": { "length": 2800, "width": 2070 },
                "blade": 7,
                "padding": 10,
                "algorithm": "optimal",
                "parts": [
                    { "length": 2097, "width": 422, "qty": 2, "rotate": true, "name": "panel A" },
                    { "length": 572, "width": 423, "qty": 6, "rotate": true, "name": "panel B" },
                    { "length": 572, "width": 462, "qty": 2, "rotate": true, "name": "panel C" },
                    { "length": 2097, "width": 422, "qty": 4, "rotate": true, "name": "panel D" },
                    { "length": 281, "width": 462, "qty": 2, "rotate": true, "name": "panel E" },
                    { "length": 2553, "width": 100, "qty": 1, "rotate": true, "name": "panel H" },
                    { "length": 2200, "width": 600, "qty": 1, "rotate": true, "name": "panel L" },
                    { "length": 695, "width": 600, "qty": 1, "rotate": true, "name": "panel K" },
                    { "length": 100, "width": 930, "qty": 1, "rotate": true, "name": "panel M" },
                    { "length": 2553, "width": 118, "qty": 1, "rotate": true, "name": "panel T" },
                    { "length": 118, "width": 930, "qty": 1, "rotate": true, "name": "panel S" }
                ]
            }"#,
        )
        .unwrap()
    }

    #[test]
    fn test_engine_golden_sample_order() {
        let req = sample_request();
        let result = LibCutEngine::optimize(&req).unwrap();

        assert_eq!(result.sheets_used, 2, "Expected 2 sheets");
        assert_eq!(result.parts_placed, 19, "Expected 19 parts placed");
        assert_eq!(result.parts_total, 19, "Expected 19 total parts");
        assert!(
            (result.efficiency_percent - 83.6).abs() < 1.0,
            "Expected ~83.6% efficiency, got {}",
            result.efficiency_percent
        );
    }

    #[test]
    fn test_engine_golden_parts_csv() {
        let req = parts_csv_request();
        let result = LibCutEngine::optimize(&req).unwrap();

        assert_eq!(result.sheets_used, 3, "Expected 3 sheets");
        assert_eq!(result.parts_placed, 22, "Expected 22 parts placed");
        assert_eq!(result.parts_total, 22, "Expected 22 total parts");
        assert!(
            (result.efficiency_percent - 57.8).abs() < 1.0,
            "Expected ~57.8% efficiency, got {}",
            result.efficiency_percent
        );
    }
}
