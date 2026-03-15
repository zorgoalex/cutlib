#![allow(non_snake_case)]

use crate::contracts::*;
use crate::internal::types::*;

pub struct ResultMapper;

impl ResultMapper {
    pub fn map(order: &Order) -> LibCutResult {
        let sheets_used = order.SheetCount;
        let parts_total: i32 = order.Parts.iter().map(|p| p.Amount).sum();
        let parts_placed = order.PartsPlaced;

        let sheet_area = order.parameters.ListLength_mm as i64
            * order.parameters.ListWidth_mm as i64;
        let total_sheet_area = sheet_area * sheets_used as i64;

        let placed_sq: i64 = order
            .Parts
            .iter()
            .flat_map(|p| {
                p.Coords
                    .iter()
                    .filter(|c| c.Cutted)
                    .map(move |_| p.Sq)
            })
            .sum();

        let efficiency = if total_sheet_area > 0 {
            let raw = placed_sq as f64 / total_sheet_area as f64 * 100.0;
            (raw * 10.0).round() / 10.0
        } else {
            0.0
        };

        let mut sheets = Vec::new();
        for sheet_num in 1..=sheets_used {
            let mut placements = Vec::new();
            let mut offcuts = Vec::new();

            // Collect part placements for this sheet
            for part in &order.Parts {
                for coord in &part.Coords {
                    if coord.Cutted && coord.list == sheet_num {
                        placements.push(LibCutPartPlacement {
                            name: part.Name.clone(),
                            length: part.Length_mm,
                            width: part.Width_mm,
                            x: coord.X,
                            y: coord.Y,
                            rotated: coord.isTurn,
                        });
                    }
                }
            }

            // Collect offcuts (NSnips) for this sheet
            for snip in &order.NSnips {
                if snip.list == sheet_num && snip.Length_mm > 0 && snip.Width_mm > 0 {
                    offcuts.push(LibCutOffcut {
                        length: snip.Length_mm,
                        width: snip.Width_mm,
                        x: snip.X,
                        y: snip.Y,
                    });
                }
            }

            sheets.push(LibCutSheetResult {
                sheet: sheet_num,
                parts: placements,
                offcuts,
            });
        }

        LibCutResult {
            sheet_size: LibCutSheetSize {
                length: order.parameters.ListLength_mm,
                width: order.parameters.ListWidth_mm,
            },
            sheets_used,
            parts_placed,
            parts_total,
            efficiency_percent: efficiency,
            sheets,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_order_with_placements() -> Order {
        let mut order = Order::default();
        order.parameters.ListLength_mm = 2440;
        order.parameters.ListWidth_mm = 1220;
        order.SheetCount = 1;
        order.PartsPlaced = 2;

        order.Parts.push(Part {
            Npart: 0,
            Name: "Panel A".into(),
            Length_mm: 800,
            Width_mm: 400,
            Amount: 2,
            Sq: 320_000,
            Turn: true,
            nPlaced: 2,
            Coords: vec![
                Coord {
                    X: 10,
                    Y: 10,
                    list: 1,
                    Cutted: true,
                    isTurn: false,
                    ..Default::default()
                },
                Coord {
                    X: 814,
                    Y: 10,
                    list: 1,
                    Cutted: true,
                    isTurn: true,
                    ..Default::default()
                },
            ],
            ..Default::default()
        });

        order.NSnips.push(Snip {
            Length_mm: 100,
            Width_mm: 200,
            X: 1618,
            Y: 10,
            list: 1,
            ..Default::default()
        });

        order
    }

    #[test]
    fn test_result_mapper_basic() {
        let order = create_order_with_placements();
        let result = ResultMapper::map(&order);

        assert_eq!(result.sheet_size.length, 2440);
        assert_eq!(result.sheet_size.width, 1220);
        assert_eq!(result.sheets_used, 1);
        assert_eq!(result.parts_placed, 2);
        assert_eq!(result.parts_total, 2);
        assert!(result.efficiency_percent > 0.0);

        assert_eq!(result.sheets.len(), 1);
        assert_eq!(result.sheets[0].parts.len(), 2);
        assert_eq!(result.sheets[0].parts[0].name, "Panel A");
        assert_eq!(result.sheets[0].parts[0].x, 10);
        assert!(!result.sheets[0].parts[0].rotated);
        assert!(result.sheets[0].parts[1].rotated);

        assert_eq!(result.sheets[0].offcuts.len(), 1);
        assert_eq!(result.sheets[0].offcuts[0].length, 100);
    }

    #[test]
    fn test_efficiency_calculation() {
        let order = create_order_with_placements();
        let result = ResultMapper::map(&order);
        // 2 parts * 320000 sq = 640000 placed
        // 1 sheet * 2440 * 1220 = 2976800
        // efficiency = 640000 / 2976800 * 100 = 21.5%
        assert!((result.efficiency_percent - 21.5).abs() < 0.1);
    }
}
