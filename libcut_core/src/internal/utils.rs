#![allow(non_snake_case)]

use super::types::*;

/// Clear cutting information from Order (reset placement state)
pub fn clear_cutting_info(order: &mut Order) {
    order.SheetCount = 0;
    order.PartsPlaced = 0;
    order.NSnips.clear();

    for part in &mut order.Parts {
        part.nPlaced = 0;
        for coord in &mut part.Coords {
            coord.X = 0;
            coord.Y = 0;
            coord.list = -1;
            coord.nlist = -1;
            coord.Cutted = false;
            coord.isTurn = false;
            coord.onList = false;
        }
    }
}
