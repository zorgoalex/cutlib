#![allow(non_snake_case)]
#![allow(unused_assignments)]
#![allow(clippy::needless_range_loop)]
#![allow(clippy::comparison_chain)]

use super::algorithm_types::*;
use super::length_alg::LengthAlg;
use super::width_alg::WidthAlg;

pub struct OptAlg {
    pub THE_SAME_PARTS_LIMIT: i32,
    pub LINES_LIMIT: i32,
    pub LINES_SORT_ITERS_LIMIT: i32,
    pub PARTS_SORT_LIMIT: i32,
    pub GET_SHEET_ITER: i32,

    B: i32,
}

impl OptAlg {
    pub fn new() -> Self {
        Self {
            THE_SAME_PARTS_LIMIT: 40,
            LINES_LIMIT: 200,
            LINES_SORT_ITERS_LIMIT: 4,
            PARTS_SORT_LIMIT: 2,
            GET_SHEET_ITER: 3,
            B: 0,
        }
    }

    fn clean_cparts(parts: &mut Vec<CPart>) {
        let mut i: i32 = 0;
        while (i as usize) < parts.len() {
            let idx = i as usize;
            if parts[idx].Qty > parts[idx].Plased && parts[idx].Plased != 0 {
                parts[idx].Qty -= parts[idx].Plased;
                parts[idx].Plased = 0;
            } else if parts[idx].Qty == parts[idx].Plased {
                parts.remove(idx);
                i -= 1;
            }
            i += 1;
        }
    }

    pub fn get_sheet_opt_alg_2(
        &mut self,
        parts: &mut Vec<CPart>,
        list_length: i32,
        list_width: i32,
        blade: i32,
        padding: i32,
        double_padding: bool,
        _same_max: bool,
        _max_sq: bool,
        _opti_on: bool,
        _turn_on: bool,
        _alg: i32,
    ) -> CSheet {
        Self::clean_cparts(parts);
        self.B = blade;

        // --- Length algorithm ---
        let mut length_alg = LengthAlg::new();
        length_alg.THE_SAME_PARTS_LIMIT = self.THE_SAME_PARTS_LIMIT;
        length_alg.LINES_LIMIT = self.LINES_LIMIT;
        length_alg.LINES_SORT_ITERS_LIMIT = self.LINES_SORT_ITERS_LIMIT;
        length_alg.PARTS_SORT_LIMIT = self.PARTS_SORT_LIMIT;

        // --- Width algorithm ---
        let mut width_alg = WidthAlg::new();
        width_alg.THE_SAME_PARTS_LIMIT = self.THE_SAME_PARTS_LIMIT;
        width_alg.LINES_LIMIT = self.LINES_LIMIT;
        width_alg.LINES_SORT_ITERS_LIMIT = self.LINES_SORT_ITERS_LIMIT;
        width_alg.PARTS_SORT_LIMIT = self.PARTS_SORT_LIMIT;

        // Get length-cut sheet
        let csheet_length_cut = length_alg.get_csheet_length_cut(
            parts,
            list_length,
            list_width,
            blade,
            padding,
            double_padding,
            true,
            false,
        );

        let mut csheet = Self::create_csheet(list_length, list_width, blade, padding, double_padding);
        for i in 0..csheet_length_cut.Lines.len() {
            Self::put_line_to_sheet(&mut csheet, csheet_length_cut.Lines[i].clone(), blade);
        }
        csheet.Alg = 1;
        Self::set_off_parts_in_sheet(parts, &csheet);

        // Get width-cut sheet
        let csheet_width_cut = width_alg.get_csheet_width_cut(
            parts,
            list_length,
            list_width,
            blade,
            padding,
            double_padding,
            true,
            false,
        );

        let mut csheet2 = Self::create_csheet(list_length, list_width, blade, padding, double_padding);
        for j in 0..csheet_width_cut.Lines.len() {
            Self::put_line_to_sheet(&mut csheet2, csheet_width_cut.Lines[j].clone(), blade);
        }
        Self::set_off_parts_in_sheet(parts, &csheet2);
        csheet2.Alg = 2;

        let mut list: Vec<CSheet> = Vec::new();
        list.push(csheet);
        list.push(csheet2);

        let mut flag = false;
        let mut num = 0;
        while !flag && num < self.GET_SHEET_ITER {
            flag = true;
            num += 1;
            let count = list.len();
            let mut _num2 = 0;
            for k in 0..count {
                let csheet3 = &list[k];
                let mut csheet4 = Self::create_csheet(csheet3.L, csheet3.W, blade, padding, double_padding);
                if csheet3.Alg == 1 {
                    csheet4.Alg = 1;
                } else {
                    csheet4.Alg = 2;
                }

                if num >= csheet3.Lines.len() as i32 {
                    continue;
                }
                _num2 += 1;

                for l in 0..(num as usize) {
                    Self::put_line_to_sheet(&mut csheet4, list[k].Lines[l].clone(), blade);
                }

                Self::set_on_parts_in_sheet(parts, &csheet4);

                if Self::fast_find_first_part(parts, csheet4.Remain.L, csheet4.Remain.W) {
                    if csheet4.Alg == 1 {
                        let csheet_width_cut2 = width_alg.get_csheet_width_cut(
                            parts,
                            csheet4.Remain.L,
                            csheet4.Remain.W,
                            blade,
                            0,
                            false,
                            true,
                            false,
                        );
                        csheet4.Alg = 2;
                        Self::set_off_parts_in_sheet(parts, &csheet_width_cut2);
                        Self::set_off_parts_in_sheet(parts, &csheet4);
                        for m in 0..csheet_width_cut2.Lines.len() {
                            Self::put_line_to_sheet(&mut csheet4, csheet_width_cut2.Lines[m].clone(), blade);
                        }
                    } else if csheet4.Alg == 2 {
                        let csheet_length_cut2 = length_alg.get_csheet_length_cut(
                            parts,
                            csheet4.Remain.L,
                            csheet4.Remain.W,
                            blade,
                            0,
                            false,
                            true,
                            false,
                        );
                        csheet4.Alg = 1;
                        Self::set_off_parts_in_sheet(parts, &csheet_length_cut2);
                        Self::set_off_parts_in_sheet(parts, &csheet4);
                        for n in 0..csheet_length_cut2.Lines.len() {
                            Self::put_line_to_sheet(&mut csheet4, csheet_length_cut2.Lines[n].clone(), blade);
                        }
                    }
                    flag = false;
                    list.push(csheet4);
                } else {
                    Self::set_off_parts_in_sheet(parts, &csheet4);
                }
            }
        }

        // Pick best sheet by Parts_Sq, tie-break by larger remain area
        while list.len() != 1 {
            let last = list.len() - 1;
            if list[0].Parts_Sq > list[last].Parts_Sq {
                list.remove(last);
            } else if list[0].Parts_Sq < list[last].Parts_Sq {
                list.remove(0);
            } else if list[0].Parts_Sq == list[last].Parts_Sq
                && (list[0].Remain.L as i64 * list[0].Remain.W as i64)
                    >= (list[last].Remain.L as i64 * list[last].Remain.W as i64)
            {
                list.remove(last);
            } else {
                list.remove(0);
            }
        }

        Self::set_on_parts_in_sheet(parts, &list[0]);
        list[0].Alg = 3;
        list.remove(0)
    }

    fn put_line_to_sheet(sheet: &mut CSheet, mut line: CLine, b: i32) {
        line.crd = Crd::default();
        line.crd.X = sheet.Remain.CRD.X;
        line.crd.Y = sheet.Remain.CRD.Y;

        if line.L == sheet.Remain.L && line.W < sheet.Remain.W {
            sheet.Remain.CRD.Y += line.W + b;
            sheet.Remain.W -= line.W + b;
        } else if line.L < sheet.Remain.L && line.W == sheet.Remain.W {
            sheet.Remain.CRD.X += line.L + b;
            sheet.Remain.L -= line.L + b;
        } else if line.L == sheet.Remain.L && line.W == sheet.Remain.W {
            sheet.Remain.CRD.X += line.L + b;
            sheet.Remain.L -= line.L + b;
            sheet.Remain.CRD.Y += line.W + b;
            sheet.Remain.W -= line.W + b;
        }

        sheet.Parts_Sq += line.Parts_Sq;
        sheet.Lines.push(line);
    }

    fn create_csheet(
        list_length: i32,
        list_width: i32,
        _blade: i32,
        padding: i32,
        double_padding: bool,
    ) -> CSheet {
        let mut csheet = CSheet {
            Alg: 3,
            L: list_length,
            W: list_width,
            ..Default::default()
        };

        let mut num = padding;
        if double_padding {
            num *= 2;
        }

        csheet.Remain = CSnip {
            L: csheet.L - num,
            W: csheet.W - num,
            CRD: if double_padding {
                Crd { X: padding, Y: padding, ..Default::default() }
            } else {
                Crd { X: 0, Y: 0, ..Default::default() }
            },
            ..Default::default()
        };

        csheet
    }

    fn fast_find_first_part(parts: &[CPart], lo: i32, wo: i32) -> bool {
        if lo > 0 && wo > 0 {
            for num in (0..parts.len()).rev() {
                if parts[num].Plased < parts[num].Qty
                    && ((lo >= parts[num].L && wo >= parts[num].W)
                        || (parts[num].Turn && lo >= parts[num].W && wo >= parts[num].L))
                {
                    return true;
                }
            }
        }
        false
    }

    fn set_on_parts_in_line(parts: &mut Vec<CPart>, line: &CLine) {
        for i in 0..line.PartIDs.len() {
            let id = line.PartIDs[i];
            if id < -1 {
                let idx = (id * -1 - 2) as usize;
                parts[idx].Plased += 1;
            } else {
                let idx = id as usize;
                parts[idx].Plased += 1;
            }
        }
    }

    fn set_off_parts_in_line(parts: &mut Vec<CPart>, line: &CLine) {
        for i in 0..line.PartIDs.len() {
            let id = line.PartIDs[i];
            if id < -1 {
                let idx = (id * -1 - 2) as usize;
                parts[idx].Plased -= 1;
            } else {
                let idx = id as usize;
                parts[idx].Plased -= 1;
            }
        }
    }

    fn set_on_parts_in_sheet(parts: &mut Vec<CPart>, sheet: &CSheet) {
        for i in 0..sheet.Lines.len() {
            Self::set_on_parts_in_line(parts, &sheet.Lines[i]);
        }
    }

    fn set_off_parts_in_sheet(parts: &mut Vec<CPart>, sheet: &CSheet) {
        for i in 0..sheet.Lines.len() {
            Self::set_off_parts_in_line(parts, &sheet.Lines[i]);
        }
    }
}
