#![allow(non_snake_case)]

use crate::contracts::LibCutRequest;
use crate::error::LibCutValidationError;
use crate::internal::types::*;

pub struct OrderFactory;

impl OrderFactory {
    pub fn create(request: &LibCutRequest) -> Result<Order, LibCutValidationError> {
        let opts = request.resolve_options()?;
        let sheet_req = request.sheet_ref().unwrap();

        let mut order = Order::default();

        order.sheet = Sheet {
            Length: sheet_req.length,
            Width: sheet_req.width,
        };

        order.parameters = Parameters {
            Algoritm: opts.algorithm as i32,
            ListLength_mm: sheet_req.length,
            ListWidth_mm: sheet_req.width,
            Padding: opts.padding_mm,
            Blade: opts.blade_mm,
            Units: 1,
            StartPoint: 1,
        };

        let parts = request.parts_list();
        let mut total_sq: i64 = 0;

        for (i, p) in parts.iter().enumerate() {
            let sq = p.length as i64 * p.width as i64;
            let coords: Vec<Coord> = (0..p.qty).map(|_| Coord::default()).collect();

            order.Parts.push(Part {
                Npart: i as i32,
                Name: p.name.clone(),
                Length_mm: p.length,
                Width_mm: p.width,
                Amount: p.qty,
                Sq: sq,
                Turn: p.rotate,
                nPlaced: 0,
                Coords: coords,
                ELength: 0,
                EWidth: 0,
            });

            total_sq += sq * p.qty as i64;
        }

        order.PartsSq = total_sq;

        Ok(order)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::contracts::*;

    fn sample_request() -> LibCutRequest {
        serde_json::from_str(
            r#"{
                "sheet": { "length": 2440, "width": 1220 },
                "blade": 4,
                "padding": 10,
                "algorithm": "optimal",
                "parts": [
                    { "length": 800, "width": 400, "qty": 5, "rotate": true, "name": "Panel A" },
                    { "length": 600, "width": 300, "qty": 8, "rotate": true, "name": "Panel B" }
                ]
            }"#,
        )
        .unwrap()
    }

    #[test]
    fn test_order_factory_creates_correct_order() {
        let req = sample_request();
        let order = OrderFactory::create(&req).unwrap();

        assert_eq!(order.parameters.ListLength_mm, 2440);
        assert_eq!(order.parameters.ListWidth_mm, 1220);
        assert_eq!(order.parameters.Blade, 4);
        assert_eq!(order.parameters.Padding, 10);
        assert_eq!(order.parameters.Algoritm, 3); // Optimal

        assert_eq!(order.Parts.len(), 2);
        assert_eq!(order.Parts[0].Name, "Panel A");
        assert_eq!(order.Parts[0].Length_mm, 800);
        assert_eq!(order.Parts[0].Width_mm, 400);
        assert_eq!(order.Parts[0].Amount, 5);
        assert_eq!(order.Parts[0].Sq, 320_000);
        assert!(order.Parts[0].Turn);
        assert_eq!(order.Parts[0].Coords.len(), 5);

        assert_eq!(order.Parts[1].Name, "Panel B");
        assert_eq!(order.Parts[1].Amount, 8);
        assert_eq!(order.Parts[1].Coords.len(), 8);

        // Total: 5*320000 + 8*180000 = 1600000 + 1440000 = 3040000
        assert_eq!(order.PartsSq, 3_040_000);
    }

    #[test]
    fn test_coords_initialized_unplaced() {
        let req = sample_request();
        let order = OrderFactory::create(&req).unwrap();

        for part in &order.Parts {
            for coord in &part.Coords {
                assert_eq!(coord.list, -1);
                assert!(!coord.Cutted);
                assert!(!coord.isTurn);
            }
        }
    }
}
