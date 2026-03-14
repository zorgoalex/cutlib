#![allow(non_snake_case)]

use super::algorithm_types::*;
use super::types::*;

/// Convert Order Parts to algorithm CParts (x10 scaling)
pub fn convert_parts_to_cparts(parts: &[Part]) -> Vec<CPart> {
    let mut result = Vec::new();
    for (i, p) in parts.iter().enumerate() {
        if p.nPlaced < p.Amount {
            result.push(CPart {
                L: p.Length_mm * 10,
                W: p.Width_mm * 10,
                Qty: p.Amount - p.nPlaced,
                iD_in_Order: i as i32,
                Turn: p.Turn,
                Plased: 0,
            });
        }
    }
    result
}

/// Deep clone CParts list
pub fn copy_cparts(parts: &[CPart]) -> Vec<CPart> {
    parts.to_vec()
}

/// Check if any CPart fits in available space (x10 dimensions)
pub fn fast_find_first_cpart(parts: &[CPart], lo: i32, wo: i32) -> bool {
    if lo <= 0 || wo <= 0 {
        return false;
    }
    for p in parts.iter().rev() {
        if p.Plased < p.Qty
            && ((lo >= p.L && wo >= p.W) || (p.Turn && lo >= p.W && wo >= p.L))
        {
            return true;
        }
    }
    false
}

/// Check if any Part fits in available space (mm dimensions)
pub fn fast_find_first_part(parts: &[Part], lo: i32, wo: i32) -> bool {
    if lo <= 0 || wo <= 0 {
        return false;
    }
    for p in parts.iter() {
        if p.nPlaced < p.Amount
            && (((lo - p.Length_mm) * 100 >= 0 && (wo - p.Width_mm) * 100 >= 0)
                || (p.Turn
                    && (lo - p.Width_mm) * 100 >= 0
                    && (wo - p.Length_mm) * 100 >= 0))
        {
            return true;
        }
    }
    false
}

/// Write CSheets to Order: convert x10 coords back to mm, populate Coords and NSnips
pub fn write_sheets_to_order(order: &mut Order, sheets: &mut Vec<CSheet>) {
    let blade10 = order.parameters.Blade * 10;
    let padding10 = order.parameters.Padding * 10;

    for (i, sheet) in sheets.iter_mut().enumerate() {
        match sheet.Alg {
            3 => {
                // Alg 3 (Optimal): lines already have absolute coords
                for line in &sheet.Lines {
                    for (j, &part_id) in line.PartIDs.iter().enumerate() {
                        let is_turn = part_id < -1;
                        let crd = &line.Parts_Crds[j];
                        write_part(
                            &mut order.Parts[crd.id_in_order as usize],
                            line.crd.X + crd.X,
                            line.crd.Y + crd.Y,
                            (i + 1) as i32,
                            -1,
                            is_turn,
                        );
                        order.PartsPlaced += 1;
                    }
                    for snip in &line.Snips {
                        order.NSnips.push(write_nsnip(
                            snip.L,
                            snip.W,
                            line.crd.X + snip.CRD.X,
                            line.crd.Y + snip.CRD.Y,
                            (i + 1) as i32,
                            -1,
                        ));
                    }
                }
                let remain = &sheet.Remain;
                order.NSnips.push(write_nsnip(
                    remain.L,
                    remain.W,
                    remain.CRD.X,
                    remain.CRD.Y,
                    (i + 1) as i32,
                    -1,
                ));
                order.SheetCount += 1;
            }
            1 => {
                // Alg 1 (Length): sort lines by W descending, stack vertically
                let num6 = padding10;
                let mut num7 = padding10;
                while !sheet.Lines.is_empty() {
                    // Find line with max W (break ties by Parts_Sq)
                    let mut max_w = 0;
                    let mut best_idx: i32 = -1;
                    for (m, line) in sheet.Lines.iter().enumerate() {
                        if (line.W - max_w) * 100 > 0
                            || ((line.W - max_w) * 100 == 0
                                && best_idx >= 0
                                && ((line.Parts_Sq
                                    - sheet.Lines[best_idx as usize].Parts_Sq)
                                    * 100.0)
                                    as i32
                                    > 0)
                        {
                            best_idx = m as i32;
                            max_w = line.W;
                        }
                    }
                    let idx = best_idx as usize;

                    for (n, &part_id) in sheet.Lines[idx].PartIDs.iter().enumerate() {
                        let is_turn = part_id < -1;
                        let crd = &sheet.Lines[idx].Parts_Crds[n];
                        write_part(
                            &mut order.Parts[crd.id_in_order as usize],
                            num6 + crd.X,
                            num7 + crd.Y,
                            (i + 1) as i32,
                            -1,
                            is_turn,
                        );
                        order.PartsPlaced += 1;
                    }
                    for snip in &sheet.Lines[idx].Snips {
                        order.NSnips.push(write_nsnip(
                            snip.L,
                            snip.W,
                            num6 + snip.CRD.X,
                            num7 + snip.CRD.Y,
                            (i + 1) as i32,
                            -1,
                        ));
                    }
                    num7 = num7 + blade10 + sheet.Lines[idx].W;
                    sheet.Lines.remove(idx);
                }
                let remain = &sheet.Remain;
                order.NSnips.push(write_nsnip(
                    remain.L, remain.W, num6, num7, (i + 1) as i32, -1,
                ));
                order.SheetCount += 1;
            }
            2 => {
                // Alg 2 (Width): sort lines by L descending, stack horizontally
                let mut num3 = padding10;
                let num4 = padding10;
                while !sheet.Lines.is_empty() {
                    let mut max_l = 0;
                    let mut best_idx: i32 = -1;
                    for (j, line) in sheet.Lines.iter().enumerate() {
                        if (line.L - max_l) * 100 > 0
                            || ((line.L - max_l) * 100 == 0
                                && best_idx >= 0
                                && ((line.Parts_Sq
                                    - sheet.Lines[best_idx as usize].Parts_Sq)
                                    * 100.0)
                                    as i32
                                    > 0)
                        {
                            best_idx = j as i32;
                            max_l = line.L;
                        }
                    }
                    let idx = best_idx as usize;

                    for (k, &part_id) in sheet.Lines[idx].PartIDs.iter().enumerate() {
                        let is_turn = part_id < -1;
                        let crd = &sheet.Lines[idx].Parts_Crds[k];
                        write_part(
                            &mut order.Parts[crd.id_in_order as usize],
                            num3 + crd.X,
                            num4 + crd.Y,
                            (i + 1) as i32,
                            -1,
                            is_turn,
                        );
                        order.PartsPlaced += 1;
                    }
                    for snip in &sheet.Lines[idx].Snips {
                        order.NSnips.push(write_nsnip(
                            snip.L,
                            snip.W,
                            num3 + snip.CRD.X,
                            num4 + snip.CRD.Y,
                            (i + 1) as i32,
                            -1,
                        ));
                    }
                    num3 = num3 + blade10 + sheet.Lines[idx].L;
                    sheet.Lines.remove(idx);
                }
                let remain = &sheet.Remain;
                order.NSnips.push(write_nsnip(
                    remain.L, remain.W, num3, num4, (i + 1) as i32, -1,
                ));
                order.SheetCount += 1;
            }
            _ => {}
        }
    }
}

/// Write a placed part's coordinates to the Order (x10 -> mm)
fn write_part(part: &mut Part, x: i32, y: i32, list: i32, nlist: i32, is_turn: bool) {
    let idx = part.nPlaced as usize;
    part.Coords[idx].X = x / 10;
    part.Coords[idx].Y = y / 10;
    part.Coords[idx].isTurn = is_turn;
    part.Coords[idx].list = list;
    part.Coords[idx].nlist = nlist;
    part.Coords[idx].Cutted = true;
    part.Coords[idx].onList = true;
    part.nPlaced += 1;
}

/// Create an offcut Snip from x10 dimensions
fn write_nsnip(l: i32, w: i32, x: i32, y: i32, list: i32, nlist: i32) -> Snip {
    let length_mm = l / 10;
    let width_mm = w / 10;
    Snip {
        Length_mm: length_mm,
        Width_mm: width_mm,
        onList: true,
        Sq: length_mm as i64 * width_mm as i64,
        list,
        nlist,
        Amount: 1,
        X: x / 10,
        Y: y / 10,
        ..Default::default()
    }
}

/// Remove fully-placed parts from list
pub fn clean_cparts(parts: &[CPart]) -> Vec<CPart> {
    parts
        .iter()
        .filter(|p| p.Plased < p.Qty)
        .cloned()
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_convert_parts_to_cparts() {
        let parts = vec![
            Part {
                Length_mm: 800,
                Width_mm: 400,
                Amount: 5,
                nPlaced: 2,
                Turn: true,
                ..Default::default()
            },
            Part {
                Length_mm: 600,
                Width_mm: 300,
                Amount: 3,
                nPlaced: 3, // fully placed
                Turn: false,
                ..Default::default()
            },
        ];
        let cparts = convert_parts_to_cparts(&parts);
        assert_eq!(cparts.len(), 1); // only first part has remaining
        assert_eq!(cparts[0].L, 8000); // 800 * 10
        assert_eq!(cparts[0].W, 4000); // 400 * 10
        assert_eq!(cparts[0].Qty, 3); // 5 - 2
        assert_eq!(cparts[0].iD_in_Order, 0);
        assert!(cparts[0].Turn);
    }

    #[test]
    fn test_copy_cparts() {
        let parts = vec![CPart {
            L: 8000,
            W: 4000,
            Qty: 5,
            Plased: 1,
            Turn: true,
            iD_in_Order: 0,
        }];
        let copy = copy_cparts(&parts);
        assert_eq!(copy.len(), 1);
        assert_eq!(copy[0].L, 8000);
        assert_eq!(copy[0].Plased, 1);
    }

    #[test]
    fn test_fast_find_first_cpart() {
        let parts = vec![CPart {
            L: 8000,
            W: 4000,
            Qty: 5,
            Plased: 0,
            Turn: true,
            iD_in_Order: 0,
        }];
        assert!(fast_find_first_cpart(&parts, 8000, 4000));
        assert!(fast_find_first_cpart(&parts, 4000, 8000)); // rotated
        assert!(!fast_find_first_cpart(&parts, 3000, 3000));
        assert!(!fast_find_first_cpart(&parts, 0, 8000));
    }

    #[test]
    fn test_clean_cparts() {
        let parts = vec![
            CPart { Qty: 5, Plased: 5, ..Default::default() }, // fully placed
            CPart { Qty: 3, Plased: 1, ..Default::default() }, // remaining
        ];
        let cleaned = clean_cparts(&parts);
        assert_eq!(cleaned.len(), 1);
        assert_eq!(cleaned[0].Qty, 3);
    }
}
