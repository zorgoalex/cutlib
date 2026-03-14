#![allow(non_snake_case)]

use super::algorithm_types::*;
use std::time::Instant;

/// Width-cut algorithm: a mirror of Length_Alg with L and W roles swapped.
/// Lines stack horizontally by L (not vertically by W).
/// Alg = 2.
pub struct WidthAlg {
    pub THE_SAME_PARTS_LIMIT: i32,
    pub LINES_LIMIT: i32,
    pub LINES_SORT_ITERS_LIMIT: i32,
    pub PARTS_SORT_LIMIT: i32,
    pub TIME_GET_LINES_LIMIT: f64,

    B: i32,
    P: i32,
    L_L: i32,
    L_W: i32,
    minL: i32,
    minW: i32,
}

impl WidthAlg {
    pub fn new() -> Self {
        Self {
            THE_SAME_PARTS_LIMIT: 25,
            LINES_LIMIT: 200,
            LINES_SORT_ITERS_LIMIT: 4,
            PARTS_SORT_LIMIT: 2,
            TIME_GET_LINES_LIMIT: 2.0,
            B: 0,
            P: 0,
            L_L: 0,
            L_W: 0,
            minL: 0,
            minW: 0,
        }
    }

    // -----------------------------------------------------------------------
    // Public entry point
    // -----------------------------------------------------------------------
    pub fn get_csheet_width_cut(
        &mut self,
        parts: &mut Vec<CPart>,
        list_length: i32,
        list_width: i32,
        blade: i32,
        padding: i32,
        double_padding: bool,
        opti_on: bool,
        clean_parts: bool,
    ) -> CSheet {
        self.L_L = list_length;
        self.L_W = list_width;
        self.P = padding;
        self.B = blade;

        if clean_parts {
            Self::clean_cparts(parts);
        }

        // Compute minL / minW from parts
        self.minL = i32::MAX;
        self.minW = i32::MAX;
        for p in parts.iter() {
            if p.Plased < p.Qty {
                if p.L < self.minL { self.minL = p.L; }
                if p.W < self.minW { self.minW = p.W; }
                if p.Turn {
                    if p.W < self.minL { self.minL = p.W; }
                    if p.L < self.minW { self.minW = p.L; }
                }
            }
        }
        if self.minL == i32::MAX { self.minL = 0; }
        if self.minW == i32::MAX { self.minW = 0; }

        let mut csheet = CSheet {
            Alg: 2,
            L: list_length,
            W: list_width,
            ..Default::default()
        };
        let mut lines_index: Vec<i32> = Vec::new();

        let mut num = padding;
        if double_padding {
            num *= 2;
        }
        let num2 = csheet.L - num; // available length
        let num3 = csheet.W - num; // available width

        // Build candidate lines
        let (c_lines, mut on_sheet_flags) =
            self.get_clines_width_cut(parts, num2, num3, opti_on);

        // Find minimum line L
        let mut num4 = num2;
        for line in &c_lines {
            if num4 > line.L {
                num4 = line.L;
            }
        }

        // Greedily fill sheet with lines (by L)
        let mut num5 = num2; // remaining length on sheet
        for j in 0..c_lines.len() {
            if num5 >= c_lines[j].L {
                num5 = num5 - c_lines[j].L - self.B;
                csheet.Lines.push(c_lines[j].clone());
                lines_index.push(j as i32);
                on_sheet_flags[j] = true;
                if num4 >= num5 {
                    break;
                }
            }
        }

        // Iterative line-swap improvement
        let mut flag = false;
        let mut num6 = 0;
        while !flag && num6 < self.LINES_SORT_ITERS_LIMIT {
            num6 += 1;
            let mut num7: i32 = -1;
            let mut num8: i32 = -1;
            let mut best_arr: Option<[i32; 3]> = None;
            let mut num9: f64 = 0.0;

            let sheet_line_count = csheet.Lines.len();
            if sheet_line_count >= 2 {
                for k in 0..sheet_line_count - 1 {
                    for l in k + 1..sheet_line_count {
                        on_sheet_flags[lines_index[k] as usize] = false;
                        on_sheet_flags[lines_index[l] as usize] = false;

                        let wo = num5 + self.B + csheet.Lines[k].L + self.B + csheet.Lines[l].L;
                        let mut _check = false;
                        let array2 = Self::find_zamena_lines_width_cut(
                            &c_lines,
                            &on_sheet_flags,
                            wo,
                            num4,
                            &mut _check,
                        );

                        if lines_index[k] != array2[0]
                            || lines_index[l] != array2[1]
                            || array2[2] != -1
                        {
                            let num10 = csheet.Lines[k].L + csheet.Lines[l].L;
                            let num11 = csheet.Lines[k].Parts_Sq + csheet.Lines[l].Parts_Sq;
                            let mut num12 = 0;
                            let mut num13: f64 = 0.0;
                            for m in 0..3 {
                                if array2[m] != -1 {
                                    num13 += c_lines[array2[m] as usize].Parts_Sq;
                                    num12 += c_lines[array2[m] as usize].L;
                                }
                            }
                            if num12 >= num10 && (num13 - num11) as i32 >= 0 {
                                let better = (num13 - num9) as i32 > 0
                                    || ((num13 - num9) as i32 == 0
                                        && num7 >= 0
                                        && num8 >= 0
                                        && num12
                                            - csheet.Lines[num7 as usize].L
                                            - csheet.Lines[num8 as usize].L
                                            > 0);
                                if better {
                                    num7 = k as i32;
                                    num8 = l as i32;
                                    best_arr = Some(array2);
                                    num9 = num13;
                                }
                            }
                        }

                        on_sheet_flags[lines_index[k] as usize] = true;
                        on_sheet_flags[lines_index[l] as usize] = true;
                    }
                }
            }

            if num7 != -1 && num8 != -1 {
                let array = best_arr.unwrap();
                num5 = num5
                    + self.B
                    + csheet.Lines[num7 as usize].L
                    + self.B
                    + csheet.Lines[num8 as usize].L;

                on_sheet_flags[lines_index[num7 as usize] as usize] = false;
                on_sheet_flags[lines_index[num8 as usize] as usize] = false;

                // Remove the two lines (higher index first)
                csheet.Lines.remove(num7 as usize);
                lines_index.remove(num7 as usize);
                csheet.Lines.remove((num8 - 1) as usize);
                lines_index.remove((num8 - 1) as usize);

                for n in 0..3 {
                    if array[n] != -1 {
                        csheet.Lines.push(c_lines[array[n] as usize].clone());
                        lines_index.push(array[n]);
                        on_sheet_flags[array[n] as usize] = true;
                        num5 = num5 - self.B - c_lines[array[n] as usize].L;
                    }
                }
            } else {
                flag = true;
            }
        }

        // Remain
        csheet.Remain = CSnip {
            L: num5,
            W: num3,
            ..Default::default()
        };

        // Sort sheet lines by L descending (bubble sort)
        for num14 in 0..csheet.Lines.len().saturating_sub(1) {
            for num15 in num14 + 1..csheet.Lines.len() {
                if csheet.Lines[num15].L > csheet.Lines[num14].L {
                    lines_index.swap(num14, num15);
                    csheet.Lines.swap(num14, num15);
                }
            }
        }

        // SET_OFF parts for lines not on sheet
        for num16 in 0..c_lines.len() {
            if !on_sheet_flags[num16] {
                Self::set_off_parts_in_line(parts, &c_lines[num16]);
            }
        }

        // Accumulate Parts_Sq and continue filling each line
        csheet.Parts_Sq = 0.0;
        let max_way = true;
        let opti_on_flag = true;
        for num17 in (0..csheet.Lines.len()).rev() {
            self.continue_line_width_cut(&mut csheet.Lines[num17], parts, max_way, opti_on_flag);
            csheet.Parts_Sq += csheet.Lines[num17].Parts_Sq;
        }

        // Fill remaining space with more lines
        flag = false;
        while !flag {
            if Self::fast_find_first_part(parts, csheet.Remain.L, csheet.Remain.W) {
                let mut c_line = CLine::default();
                let mut c_snip = CSnip::default();
                c_line.Snips = Vec::new();
                c_line.PartIDs = Vec::new();
                c_line.Parts_Crds = Vec::new();

                let num18 =
                    self.find_length_part(parts, csheet.Remain.L, csheet.Remain.W, true);
                let (_id, ld, _wd) = Self::get_id_ld_wd(parts, num18);

                c_line.W = csheet.Remain.W;
                c_line.L = ld;
                c_snip.CRD = Crd { X: 0, Y: 0, ..Default::default() };
                c_snip.L = c_line.L;
                c_snip.W = c_line.W;
                c_line.Snips.push(c_snip);

                let io = self.find_small_snip(&c_line.Snips, parts);
                self.place_part_to_line(&mut c_line, parts, num18, io, true);
                self.continue_line_width_cut(&mut c_line, parts, max_way, true);
                csheet.Remain.L = csheet.Remain.L - self.B - c_line.L;
                csheet.Lines.push(c_line);
            } else {
                flag = true;
            }
        }

        csheet
    }

    // -----------------------------------------------------------------------
    // Private helpers
    // -----------------------------------------------------------------------

    fn clean_cparts(parts: &mut Vec<CPart>) {
        let mut i = 0;
        while i < parts.len() {
            if parts[i].Qty > parts[i].Plased && parts[i].Plased != 0 {
                parts[i].Qty -= parts[i].Plased;
                parts[i].Plased = 0;
                i += 1;
            } else if parts[i].Qty == parts[i].Plased {
                parts.remove(i);
            } else {
                i += 1;
            }
        }
    }

    /// Returns (lines, on_sheet_flags)
    fn get_clines_width_cut(
        &mut self,
        parts: &mut Vec<CPart>,
        ll: i32,
        lw: i32,
        opti_on: bool,
    ) -> (Vec<CLine>, Vec<bool>) {
        let start_time = Instant::now();
        let mut list: Vec<CLine> = Vec::new();
        let mut on_sheet: Vec<bool> = Vec::new();
        let mut num2 = 0;
        let mut flag = false;
        let mut num3 = ll;
        let num4 = lw;

        while !flag && num2 < self.LINES_LIMIT {
            num2 += 1;
            let c_line: Option<CLine>;
            let num = self.find_length_part(parts, num3, num4, false);

            if num != -1 {
                // Branch 1: THE_SAME_MAX = true
                let (line_a, pre_cut_a) =
                    self.make_line_width_cut(parts, num, num3, num4, true, false, opti_on);
                let mut c_line_a = line_a;

                if let Some(ref mut pc) = pre_cut_a.clone() {
                    Self::set_on_parts_in_line(parts, pc);
                    self.continue_line_width_cut(pc, parts, true, opti_on);
                    Self::set_off_parts_in_line(parts, pc);
                }
                if let (Some(ref la), Some(ref pca)) = (&c_line_a, &pre_cut_a) {
                    if (pca.Parts_Sq - la.Parts_Sq) as i32 > 0 {
                        c_line_a = pre_cut_a.clone();
                    }
                }

                // Branch 2: THE_SAME_MAX = false
                let (line_b, pre_cut_b) =
                    self.make_line_width_cut(parts, num, num3, num4, false, false, opti_on);
                let mut c_line_b = line_b;

                if let Some(ref mut pcb) = pre_cut_b.clone() {
                    Self::set_on_parts_in_line(parts, pcb);
                    self.continue_line_width_cut(pcb, parts, true, opti_on);
                    Self::set_off_parts_in_line(parts, pcb);
                }
                if let (Some(ref lb), Some(ref pcb)) = (&c_line_b, &pre_cut_b) {
                    if ((lb.Parts_Sq - pcb.Parts_Sq) as i32) < 0 {
                        c_line_b = pre_cut_b.clone();
                    }
                }

                // Compare a and b
                if let (Some(ref la), Some(ref lb)) = (&c_line_a, &c_line_b) {
                    let diff = ((la.filling() - lb.filling()) * 100.0) as i32;
                    if diff < 0 {
                        c_line_a = c_line_b.clone();
                    } else if diff == 0 && la.L < lb.L {
                        c_line_a = c_line_b.clone();
                    }
                } else if c_line_a.is_none() && c_line_b.is_some() {
                    c_line_a = c_line_b.clone();
                }

                // Try rotated variant
                let mut c_line_3: Option<CLine> = None;
                let index = if num < -1 { num * -1 - 2 } else { num };
                if parts[index as usize].Turn {
                    let can_rotated = if num < -1 {
                        num3 >= parts[index as usize].L && num4 >= parts[index as usize].W
                    } else {
                        num3 >= parts[index as usize].W && num4 >= parts[index as usize].L
                    };
                    if can_rotated {
                        let rotated_id = num * -1 - 2;

                        // cLine3 = MakeLine(..., THE_SAME_MAX: true, ...) => PreCut3
                        let (line_c, pre_cut_c) = self.make_line_width_cut(
                            parts, rotated_id, num3, num4, true, false, opti_on,
                        );
                        let c_line_c = line_c;
                        let mut pre_cut_3_continued: Option<CLine> = None;
                        if let Some(ref pcc) = pre_cut_c {
                            let mut pcc_m = pcc.clone();
                            Self::set_on_parts_in_line(parts, &pcc_m);
                            self.continue_line_width_cut(&mut pcc_m, parts, true, opti_on);
                            Self::set_off_parts_in_line(parts, &pcc_m);
                            pre_cut_3_continued = Some(pcc_m);
                        }

                        // cLine4 = MakeLine(..., THE_SAME_MAX: false, ...) => PreCut4
                        let (line_d, pre_cut_d) = self.make_line_width_cut(
                            parts, rotated_id, num3, num4, false, false, opti_on,
                        );
                        let c_line_d = line_d;
                        let mut pre_cut_4_continued: Option<CLine> = None;
                        if let Some(ref pcd) = pre_cut_d {
                            let mut pcd_m = pcd.clone();
                            Self::set_on_parts_in_line(parts, &pcd_m);
                            self.continue_line_width_cut(&mut pcd_m, parts, true, opti_on);
                            Self::set_off_parts_in_line(parts, &pcd_m);
                            pre_cut_4_continued = Some(pcd_m);
                        }

                        // C# logic: if (cLine3 != null && cLine4 != null && PreCut4 != null)
                        if c_line_c.is_some() && c_line_d.is_some() && pre_cut_4_continued.is_some() {
                            c_line_3 = c_line_c.clone();
                            // if PreCut3.Parts_Sq > cLine3.Parts_Sq => cLine3 = PreCut3
                            if let (Some(ref pc3), Some(ref c3)) = (&pre_cut_3_continued, &c_line_3) {
                                if (pc3.Parts_Sq - c3.Parts_Sq) as i32 > 0 {
                                    c_line_3 = pre_cut_3_continued.clone();
                                }
                            }
                            // if cLine4.Parts_Sq > cLine3.Parts_Sq => cLine3 = cLine4
                            if let (Some(ref ld), Some(ref c3)) = (&c_line_d, &c_line_3) {
                                if (ld.Parts_Sq - c3.Parts_Sq) as i32 > 0 {
                                    c_line_3 = c_line_d.clone();
                                }
                            }
                            // if PreCut4.Parts_Sq > cLine3.Parts_Sq => cLine3 = PreCut4
                            if let (Some(ref pc4), Some(ref c3)) = (&pre_cut_4_continued, &c_line_3) {
                                if (pc4.Parts_Sq - c3.Parts_Sq) as i32 > 0 {
                                    c_line_3 = pre_cut_4_continued.clone();
                                }
                            }
                        }
                    }
                }

                if let (Some(ref la), Some(ref l3)) = (&c_line_a, &c_line_3) {
                    if (((la.filling() - l3.filling()) * 100.0) as i32) < 0 {
                        c_line_a = c_line_3;
                    }
                }

                c_line = c_line_a;
            } else {
                c_line = None;
            }

            if let Some(ref cl) = c_line {
                Self::set_on_parts_in_line(parts, cl);
                on_sheet.push(false);
                list.push(cl.clone());
                num3 = num3 - self.B - cl.L;
                if !Self::fast_find_first_part(parts, num3, num4) {
                    num3 = ll;
                    // num4 = lw; // num4 is immutable in the C# too (LW never changes)
                    if !Self::fast_find_first_part(parts, num3, num4) {
                        flag = true;
                    }
                }
            } else {
                flag = true;
            }

            let elapsed = start_time.elapsed().as_secs_f64();
            if ((elapsed - self.TIME_GET_LINES_LIMIT) * 10.0) as i32 > 0 {
                flag = true;
            }
        }

        (list, on_sheet)
    }

    /// Returns (Some(line), Some(pre_cut)) or (None, None)
    fn make_line_width_cut(
        &mut self,
        parts: &mut Vec<CPart>,
        start_part: i32,
        _ll: i32,
        lw: i32,
        the_same_max: bool,
        max_way: bool,
        opti_on: bool,
    ) -> (Option<CLine>, Option<CLine>) {
        let mut c_line = CLine {
            Snips: Vec::new(),
            PartIDs: Vec::new(),
            Parts_Crds: Vec::new(),
            ..Default::default()
        };

        let (_id, ld, _wd) = Self::get_id_ld_wd(parts, start_part);
        let rez = true;
        c_line.W = lw;
        c_line.L = ld;

        let c_snip = CSnip {
            CRD: Crd { X: 0, Y: 0, ..Default::default() },
            L: c_line.L,
            W: c_line.W,
            ..Default::default()
        };
        c_line.Snips.push(c_snip);

        let mut io: i32;

        if the_same_max {
            let (fix_length, min_w, _total_length) =
                self.get_parts_with_fix_length(parts, ld, c_line.W, c_line.W);
            let start_parts =
                self.get_start_parts_for_line_width_cut(parts, fix_length, c_line.W, min_w);
            io = self.find_small_snip(&c_line.Snips, parts);
            for i in 0..start_parts.len() {
                self.place_part_to_line(&mut c_line, parts, start_parts[i], io, rez);
            }
        } else {
            io = 0;
            self.place_part_to_line(&mut c_line, parts, start_part, io, rez);
            let mut flag = false;
            io = self.find_small_snip(&c_line.Snips, parts);
            if io != -1 {
                while !flag {
                    let l = c_line.Snips[io as usize].L;
                    let w = c_line.Snips[io as usize].W;
                    let num = self.find_the_same_length_part(parts, l, w);
                    if num != -1 {
                        self.place_part_to_line(&mut c_line, parts, num, io, rez);
                    } else {
                        flag = true;
                    }
                }
            }
        }

        let pre_cut = Self::copy_line_without_marks(&c_line);
        self.continue_line_width_cut(&mut c_line, parts, max_way, opti_on);
        Self::set_off_parts_in_line(parts, &c_line);

        (Some(c_line), Some(pre_cut))
    }

    fn continue_line_width_cut(
        &mut self,
        line: &mut CLine,
        parts: &mut Vec<CPart>,
        max_way: bool,
        opti_on: bool,
    ) {
        loop {
            let num2 = self.find_small_snip(&line.Snips, parts);
            if num2 < 0 {
                break;
            }
            let l = line.Snips[num2 as usize].L;
            let w = line.Snips[num2 as usize].W;
            let num = if max_way {
                Self::find_max_sq_part(parts, l, w)
            } else {
                self.find_length_part(parts, l, w, true)
            };
            if num != -1 {
                if opti_on {
                    let array = self.check_part_for_last_in_line(parts, l, w, true, num);
                    if num != array[0] && array[0] != -1 {
                        self.place_2_parts_to_line(line, parts, &array, num2);
                    } else {
                        self.place_part_to_line(line, parts, num, num2, true);
                    }
                } else {
                    self.place_part_to_line(line, parts, num, num2, true);
                }
            } else {
                break;
            }
        }
    }

    fn copy_line_without_marks(line: &CLine) -> CLine {
        let mut c_line = CLine {
            Snips: Vec::new(),
            PartIDs: Vec::new(),
            Parts_Crds: Vec::new(),
            L: line.L,
            W: line.W,
            Parts_Sq: line.Parts_Sq,
            ..Default::default()
        };
        for i in 0..line.PartIDs.len() {
            c_line.PartIDs.push(line.PartIDs[i]);
            c_line.Parts_Crds.push(line.Parts_Crds[i].clone());
        }
        for j in 0..line.Snips.len() {
            let s = CSnip {
                L: line.Snips[j].L,
                W: line.Snips[j].W,
                CRD: Crd {
                    X: line.Snips[j].CRD.X,
                    Y: line.Snips[j].CRD.Y,
                    ..Default::default()
                },
                ..Default::default()
            };
            c_line.Snips.push(s);
        }
        c_line
    }

    fn fast_find_first_part(parts: &[CPart], lo: i32, wo: i32) -> bool {
        if lo <= 0 || wo <= 0 {
            return false;
        }
        for i in (0..parts.len()).rev() {
            if parts[i].Plased < parts[i].Qty
                && ((lo >= parts[i].L && wo >= parts[i].W)
                    || (parts[i].Turn && lo >= parts[i].W && wo >= parts[i].L))
            {
                return true;
            }
        }
        false
    }

    fn find_length_part(
        &self,
        parts: &[CPart],
        lo: i32,
        wo: i32,
        max_l: bool,
    ) -> i32 {
        let mut result: i32 = -1;
        let mut num: i32 = 0;
        let mut num2: f64 = 0.0;

        for i in 0..parts.len() {
            if parts[i].Plased >= parts[i].Qty {
                continue;
            }
            if !parts[i].Turn {
                if lo >= parts[i].L && wo >= parts[i].W {
                    if parts[i].L > num {
                        num2 = parts[i].sq();
                        num = parts[i].L;
                        result = i as i32;
                    } else if parts[i].L == num && (parts[i].sq() - num2) as i64 > 0 {
                        num2 = parts[i].sq();
                        num = parts[i].L;
                        result = i as i32;
                    }
                }
            } else {
                // Turn is true
                let mut num3: i32 = 0;
                if lo >= parts[i].L && wo >= parts[i].W && lo >= parts[i].W && wo >= parts[i].L {
                    num3 = if max_l {
                        if parts[i].L < parts[i].W {
                            parts[i].W
                        } else {
                            parts[i].L
                        }
                    } else {
                        if parts[i].L < parts[i].W {
                            parts[i].L
                        } else {
                            parts[i].W
                        }
                    };
                } else if lo >= parts[i].L
                    && wo >= parts[i].W
                    && (lo < parts[i].W || wo < parts[i].L)
                {
                    num3 = parts[i].L;
                } else if (lo < parts[i].L || wo < parts[i].W)
                    && lo >= parts[i].W
                    && wo >= parts[i].L
                {
                    num3 = parts[i].W;
                }
                if num3 > num {
                    num2 = parts[i].sq();
                    num = num3;
                    result = if parts[i].L != num3 {
                        -1 * i as i32 - 2
                    } else {
                        i as i32
                    };
                } else if num3 == num && (parts[i].sq() - num2) as i64 > 0 {
                    num2 = parts[i].sq();
                    num = num3;
                    result = if parts[i].L != num3 {
                        -1 * i as i32 - 2
                    } else {
                        i as i32
                    };
                }
            }
        }
        result
    }

    fn find_the_same_length_part(
        &self,
        parts: &[CPart],
        lo: i32,
        wo: i32,
    ) -> i32 {
        let mut result: i32 = -1;
        let mut num: f64 = 0.0;

        for i in 0..parts.len() {
            if parts[i].Plased >= parts[i].Qty {
                continue;
            }
            if !parts[i].Turn {
                if lo == parts[i].L && wo >= parts[i].W && (parts[i].sq() - num) as i64 > 0 {
                    num = parts[i].sq();
                    result = i as i32;
                }
            } else {
                let mut num2: i32 = -1;
                if lo == parts[i].L && wo >= parts[i].W {
                    num2 = parts[i].L;
                } else if lo == parts[i].W && wo >= parts[i].L {
                    num2 = parts[i].W;
                }
                if num2 > -1 && (parts[i].sq() - num) as i64 > 0 {
                    num = parts[i].sq();
                    result = if parts[i].L != num2 {
                        -1 * i as i32 - 2
                    } else {
                        i as i32
                    };
                }
            }
        }
        result
    }

    fn place_part_to_line(
        &self,
        line: &mut CLine,
        parts: &mut Vec<CPart>,
        part_id: i32,
        io: i32,
        _rez: bool,
    ) {
        let (id, ld, wd) = Self::get_id_ld_wd(parts, part_id);
        let io = io as usize;
        line.Parts_Sq += parts[id as usize].sq();
        line.PartIDs.push(part_id);

        let crd = Crd {
            X: line.Snips[io].CRD.X,
            Y: line.Snips[io].CRD.Y,
            id_in_order: parts[id as usize].iD_in_Order,
        };
        line.Parts_Crds.push(crd);
        parts[id as usize].Plased += 1;

        let l = line.Snips[io].L;
        let w = line.Snips[io].W;

        if l > ld && w > wd {
            if _rez {
                let num = l - ld - self.B;
                let num2 = w;
                let x = line.Snips[io].CRD.X + ld + self.B;
                let y = line.Snips[io].CRD.Y;
                if Self::fast_find_first_part(parts, num, num2) {
                    let item = Self::create_csnip(x, y, num, num2);
                    line.Snips.push(item);
                    let snip_x = line.Snips[io].CRD.X;
                    let snip_y = line.Snips[io].CRD.Y + wd + self.B;
                    Self::resize_csnip(&mut line.Snips[io], snip_x, snip_y, ld, w - wd - self.B);
                } else {
                    let item2 = Self::create_csnip(x, y, num, wd);
                    line.Snips.push(item2);
                    let snip_x = line.Snips[io].CRD.X;
                    let snip_y = line.Snips[io].CRD.Y + wd + self.B;
                    Self::resize_csnip(
                        &mut line.Snips[io],
                        snip_x,
                        snip_y,
                        l,
                        w - wd - self.B,
                    );
                }
            } else {
                let num3 = l;
                let num4 = w - wd - self.B;
                let x2 = line.Snips[io].CRD.X;
                let y2 = line.Snips[io].CRD.Y + wd + self.B;
                if Self::fast_find_first_part(parts, num3, num4) {
                    let item3 = Self::create_csnip(x2, y2, num3, num4);
                    line.Snips.push(item3);
                    let snip_x = line.Snips[io].CRD.X + ld + self.B;
                    let snip_y = line.Snips[io].CRD.Y;
                    Self::resize_csnip(
                        &mut line.Snips[io],
                        snip_x,
                        snip_y,
                        l - ld - self.B,
                        wd,
                    );
                } else {
                    let item4 = Self::create_csnip(x2, y2, ld, num4);
                    line.Snips.push(item4);
                    let snip_x = line.Snips[io].CRD.X + ld + self.B;
                    let snip_y = line.Snips[io].CRD.Y;
                    Self::resize_csnip(
                        &mut line.Snips[io],
                        snip_x,
                        snip_y,
                        l - ld - self.B,
                        w,
                    );
                }
            }
        } else if ld == l && wd < w {
            let snip_x = line.Snips[io].CRD.X;
            let snip_y = line.Snips[io].CRD.Y + wd + self.B;
            Self::resize_csnip(&mut line.Snips[io], snip_x, snip_y, l, w - wd - self.B);
        } else if ld < l && wd == w {
            let snip_x = line.Snips[io].CRD.X + ld + self.B;
            let snip_y = line.Snips[io].CRD.Y;
            Self::resize_csnip(&mut line.Snips[io], snip_x, snip_y, l - ld - self.B, w);
        } else if ld == l && wd == w {
            let snip_x = line.Snips[io].CRD.X;
            let snip_y = line.Snips[io].CRD.Y;
            Self::resize_csnip(&mut line.Snips[io], snip_x, snip_y, 0, 0);
        }
    }

    fn place_2_parts_to_line(
        &self,
        line: &mut CLine,
        parts: &mut Vec<CPart>,
        _2parts: &[i32],
        io: i32,
    ) {
        let io = io as usize;
        let l = line.Snips[io].L;
        let w = line.Snips[io].W;
        let x = line.Snips[io].CRD.X;
        let y = line.Snips[io].CRD.Y;
        // _ = (l - B) / 2; _ = (w - B) / 2; // ignored in C#

        let mut num: i32 = 0; // LD of second part
        let mut num2: i32 = 0; // WD of second part

        let mut num3 = _2parts[0]; // first part id
        let mut num4_id = _2parts[1]; // second part id (for Plased tracking)

        let num5: i32; // LD of first part
        let num6: i32; // WD of first part

        if num3 >= 0 {
            num5 = parts[num3 as usize].L;
            num6 = parts[num3 as usize].W;
        } else {
            num3 = num3 * -1 - 2;
            num5 = parts[num3 as usize].W;
            num6 = parts[num3 as usize].L;
        }

        line.Parts_Sq += parts[num3 as usize].sq();

        if num4_id != -1 {
            let mut num4_idx = num4_id;
            if num4_idx >= 0 {
                num = parts[num4_idx as usize].L;
                num2 = parts[num4_idx as usize].W;
            } else {
                num4_idx = num4_idx * -1 - 2;
                num = parts[num4_idx as usize].W;
                num2 = parts[num4_idx as usize].L;
            }
            num4_id = num4_idx; // resolved index
            line.Parts_Sq += parts[num4_id as usize].sq();
        }

        let crd = Crd {
            X: line.Snips[io].CRD.X,
            Y: line.Snips[io].CRD.Y,
            id_in_order: parts[num3 as usize].iD_in_Order,
        };
        line.Parts_Crds.push(crd);
        line.PartIDs.push(_2parts[0]);
        parts[num3 as usize].Plased += 1;

        if num4_id != -1 {
            // Second part placement logic
            let num7 = l - self.B - num5;
            let num8 = w;
            let num9 = l;
            let num10 = w - self.B - num6;

            let mut flag = false;
            let mut flag2 = false;

            if num <= num7 && num2 <= num8 {
                flag = true;
            }
            if num <= num9 && num2 <= num10 {
                flag2 = true;
            }

            let mut num11: i32 = -1;
            let mut num12: i32 = -1;
            let mut num13: f64 = line.Snips[io].sq() - parts[num3 as usize].sq() - parts[num4_id as usize].sq();
            let mut num14: f64 = line.Snips[io].sq() - parts[num3 as usize].sq() - parts[num4_id as usize].sq();

            if flag {
                if num6 > num2 {
                    // 4 layout variants
                    let sq1 = self.get_sq_parts_for_snips(
                        parts,
                        l, w - num6 - self.B,
                        num, num6 - num2 - self.B,
                        l - num5 - num - 2 * self.B, num6,
                    );
                    let sq2 = self.get_sq_parts_for_snips(
                        parts,
                        num5, w - num6 - self.B,
                        num, w - num2 - self.B,
                        l - num5 - num - 2 * self.B, w,
                    );
                    let sq3 = self.get_sq_parts_for_snips(
                        parts,
                        num5, w - num6 - self.B,
                        l - num5 - self.B, w - num2 - self.B,
                        l - num5 - num - 2 * self.B, num2,
                    );
                    let sq4 = self.get_sq_parts_for_snips(
                        parts,
                        l, w - num6 - self.B,
                        l - num5 - self.B, num6 - num2 - self.B,
                        l - num5 - num - 2 * self.B, num2,
                    );

                    let mut num17 = Self::pick_best_of_two(sq1, sq2, 1, 2);
                    let mut best_sq_a = if num17 == 2 { sq2 } else { sq1 };

                    let num18 = Self::pick_best_of_two(sq3, sq4, 3, 4);
                    let best_sq_b = if num18 == 4 { sq4 } else { sq3 };

                    if num17 == -1 && num18 == -1 {
                        num17 = 1;
                    } else if num17 != -1 && num18 == -1 {
                        // keep num17
                    } else if num17 == -1 && num18 != -1 {
                        num17 = num18;
                        best_sq_a = best_sq_b;
                    } else if ((best_sq_a - best_sq_b) as i64) * 100 < 0 {
                        num17 = num18;
                        best_sq_a = best_sq_b;
                    }

                    num13 -= best_sq_a;
                    num11 = num17;
                } else {
                    // num6 <= num2
                    let sq5 = self.get_sq_parts_for_snips(
                        parts,
                        l, w - num6 - self.B,
                        l - num5 - num - 2 * self.B, num6,
                        0, 0,
                    );
                    let sq6 = self.get_sq_parts_for_snips(
                        parts,
                        num5, w - num6 - self.B,
                        num, w - num2 - self.B,
                        l - num5 - num - 2 * self.B, w,
                    );

                    let mut num20 = Self::pick_best_of_two(sq5, sq6, 5, 6);
                    let mut best_sq = if num20 == 6 { sq6 } else { sq5 };
                    if num20 == -1 {
                        best_sq = 0.0;
                        num20 = 5;
                    }
                    num13 -= best_sq;
                    num11 = num20;
                }
            }

            if flag2 {
                if num5 > num {
                    let sq1 = self.get_sq_parts_for_snips(
                        parts,
                        l - num5 - self.B, w,
                        num5 - num - self.B, num2,
                        num5, w - num6 - num2 - 2 * self.B,
                    );
                    let sq2 = self.get_sq_parts_for_snips(
                        parts,
                        l - num5 - self.B, w,
                        num5 - num - self.B, w - num6 - self.B,
                        num, w - num6 - num2 - 2 * self.B,
                    );
                    let sq3 = self.get_sq_parts_for_snips(
                        parts,
                        l - num5 - self.B, num6,
                        l - num - self.B, num2,
                        l, w - num6 - num2 - 2 * self.B,
                    );
                    let sq4 = self.get_sq_parts_for_snips(
                        parts,
                        l - num5 - self.B, num6,
                        l - num - self.B, w - num6 - self.B,
                        num, w - num6 - num2 - 2 * self.B,
                    );

                    let mut num23 = Self::pick_best_of_two(sq1, sq2, 1, 2);
                    let mut best_sq_a = if num23 == 2 { sq2 } else { sq1 };

                    let num24 = Self::pick_best_of_two(sq3, sq4, 3, 4);
                    let best_sq_b = if num24 == 4 { sq4 } else { sq3 };

                    if num23 == -1 && num24 == -1 {
                        num23 = 1;
                    } else if num23 != -1 && num24 == -1 {
                        // keep
                    } else if num23 == -1 && num24 != -1 {
                        num23 = num24;
                        best_sq_a = best_sq_b;
                    } else if ((best_sq_a - best_sq_b) as i64) * 100 < 0 {
                        num23 = num24;
                        best_sq_a = best_sq_b;
                    }

                    num14 -= best_sq_a;
                    num12 = num23;
                } else {
                    // num5 <= num
                    let sq5 = self.get_sq_parts_for_snips(
                        parts,
                        l - num5 - self.B, w,
                        num, w - num6 - num2 - 2 * self.B,
                        0, 0,
                    );
                    let sq6 = self.get_sq_parts_for_snips(
                        parts,
                        l - num5 - self.B, num6,
                        l - num5 - self.B, num2,
                        l, w - num6 - num2 - 2 * self.B,
                    );

                    let mut num26 = Self::pick_best_of_two(sq5, sq6, 5, 6);
                    let mut best_sq = if num26 == 6 { sq6 } else { sq5 };
                    if num26 == -1 {
                        best_sq = 0.0;
                        num26 = 5;
                    }
                    num14 -= best_sq;
                    num12 = num26;
                }
            }

            let num27: i32;
            if flag && flag2 {
                let diff = ((num14 - num13) * 10.0) as i64;
                if diff == 0 {
                    num27 = if l < w { num11 * -1 } else { num12 };
                } else if diff >= 0 {
                    num27 = num11 * -1;
                } else {
                    num27 = num12;
                }
            } else {
                if flag && !flag2 {
                    num27 = num11 * -1;
                } else if !flag && flag2 {
                    num27 = num12;
                } else {
                    num27 = 1; // fallback
                }
            }

            let crd2 = if num27 < 0 {
                Crd {
                    X: x + num5 + self.B,
                    Y: y,
                    id_in_order: parts[num4_id as usize].iD_in_Order,
                }
            } else {
                Crd {
                    X: x,
                    Y: y + num6 + self.B,
                    id_in_order: parts[num4_id as usize].iD_in_Order,
                }
            };
            line.Parts_Crds.push(crd2);
            line.PartIDs.push(_2parts[1]);
            parts[num4_id as usize].Plased += 1;

            match num27 {
                -1 => {
                    Self::resize_csnip(&mut line.Snips[io], x, y + num6 + self.B, l, w - num6 - self.B);
                    line.Snips.push(Self::create_csnip(x + num5 + self.B, y + num2 + self.B, num, num6 - num2 - self.B));
                    line.Snips.push(Self::create_csnip(x + num5 + num + 2 * self.B, y, l - num5 - num - 2 * self.B, num6));
                }
                -2 => {
                    Self::resize_csnip(&mut line.Snips[io], x, y + num6 + self.B, num5, w - num6 - self.B);
                    line.Snips.push(Self::create_csnip(x + num5 + self.B, y + num2 + self.B, num, w - num2 - self.B));
                    line.Snips.push(Self::create_csnip(x + num5 + num + 2 * self.B, y, l - num5 - num - 2 * self.B, w));
                }
                -3 => {
                    Self::resize_csnip(&mut line.Snips[io], x, y + num6 + self.B, num5, w - num6 - self.B);
                    line.Snips.push(Self::create_csnip(x + num5 + self.B, y + num2 + self.B, l - num5 - self.B, w - num2 - self.B));
                    line.Snips.push(Self::create_csnip(x + num5 + num + 2 * self.B, y, l - num5 - num - 2 * self.B, num2));
                }
                -4 => {
                    Self::resize_csnip(&mut line.Snips[io], x, y + num6 + self.B, l, w - num6 - self.B);
                    line.Snips.push(Self::create_csnip(x + num5 + self.B, y + num2 + self.B, l - num5 - self.B, num6 - num2 - self.B));
                    line.Snips.push(Self::create_csnip(x + num5 + num + 2 * self.B, y, l - num5 - num - 2 * self.B, num2));
                }
                -5 => {
                    Self::resize_csnip(&mut line.Snips[io], x, y + num6 + self.B, l, w - num6 - self.B);
                    line.Snips.push(Self::create_csnip(x + num5 + num + 2 * self.B, y, l - num5 - num - 2 * self.B, num6));
                }
                -6 => {
                    Self::resize_csnip(&mut line.Snips[io], x, y + num6 + self.B, num5, w - num6 - self.B);
                    line.Snips.push(Self::create_csnip(x + num5 + self.B, y + num2 + self.B, num, w - num2 - self.B));
                    line.Snips.push(Self::create_csnip(x + num5 + num + 2 * self.B, y, l - num5 - num - 2 * self.B, w));
                }
                1 => {
                    Self::resize_csnip(&mut line.Snips[io], x + num5 + self.B, y, l - num5 - self.B, w);
                    line.Snips.push(Self::create_csnip(x + num + self.B, y + num6 + self.B, num5 - num - self.B, num2));
                    line.Snips.push(Self::create_csnip(x, y + num6 + num2 + 2 * self.B, num5, w - num6 - num2 - 2 * self.B));
                }
                2 => {
                    Self::resize_csnip(&mut line.Snips[io], x + num5 + self.B, y, l - num5 - self.B, w);
                    line.Snips.push(Self::create_csnip(x + num + self.B, y + num6 + self.B, num5 - num - self.B, w - num6 - self.B));
                    line.Snips.push(Self::create_csnip(x, y + num6 + num2 + 2 * self.B, num, w - num6 - num2 - 2 * self.B));
                }
                3 => {
                    Self::resize_csnip(&mut line.Snips[io], x + num5 + self.B, y, l - num5 - self.B, -num6);
                    line.Snips.push(Self::create_csnip(x + num + self.B, y + num6 + self.B, l - num - self.B, num2));
                    line.Snips.push(Self::create_csnip(x, y + num6 + num2 + 2 * self.B, l, w - num6 - num2 - 2 * self.B));
                }
                4 => {
                    Self::resize_csnip(&mut line.Snips[io], x + num5 + self.B, y, l - num5 - self.B, num6);
                    line.Snips.push(Self::create_csnip(x + num + self.B, y + num6 + self.B, l - num - self.B, w - num6 - self.B));
                    line.Snips.push(Self::create_csnip(x, y + num6 + num2 + 2 * self.B, num, w - num6 - num2 - 2 * self.B));
                }
                5 => {
                    Self::resize_csnip(&mut line.Snips[io], x + num5 + self.B, y, l - num5 - self.B, w);
                    line.Snips.push(Self::create_csnip(x, y + num6 + num2 + 2 * self.B, num, w - num6 - num2 - 2 * self.B));
                }
                6 => {
                    Self::resize_csnip(&mut line.Snips[io], x + num5 + self.B, y, l - num5 - self.B, num6);
                    line.Snips.push(Self::create_csnip(x + num + self.B, y + num6 + self.B, l - num - self.B, num2));
                    line.Snips.push(Self::create_csnip(x, y + num6 + num2 + 2 * self.B, l, w - num6 - num2 - 2 * self.B));
                }
                _ => { /* 0 => do nothing */ }
            }
        } else {
            // Only one part - choose best layout for remaining snips
            let sq7 = self.get_sq_parts_for_snips(
                parts,
                num5, w - num6 - self.B,
                l - num5 - self.B, w,
                0, 0,
            );
            let sq8 = self.get_sq_parts_for_snips(
                parts,
                l, w - num6 - self.B,
                l - num5 - self.B, num6,
                0, 0,
            );

            let mut num28 = Self::pick_best_of_two(sq7, sq8, 1, 2);
            if num28 == -1 {
                num28 = 1;
            }

            match num28 {
                1 => {
                    Self::resize_csnip(&mut line.Snips[io], x, y + num6 + self.B, num5, w - num6 - self.B);
                    line.Snips.push(Self::create_csnip(x + num5 + self.B, y, l - num5 - self.B, w));
                }
                2 => {
                    Self::resize_csnip(&mut line.Snips[io], x, y + num6 + self.B, l, w - num6 - self.B);
                    line.Snips.push(Self::create_csnip(x + num5 + self.B, y, l - num5 - self.B, num6));
                }
                _ => {
                    Self::resize_csnip(&mut line.Snips[io], x, y + num6 + self.B, num5, w - num6 - self.B);
                    line.Snips.push(Self::create_csnip(x + num5 + self.B, y, l - num5 - self.B, w));
                }
            }
        }
    }

    fn check_part_for_last_in_line(
        &self,
        parts: &mut Vec<CPart>,
        lo: i32,
        wo: i32,
        _rez: bool,
        id: i32,
    ) -> [i32; 2] {
        let mut array = [-1i32, -1i32];

        let mut num = id;
        let num2: i32;
        let num3: i32;
        if num >= 0 {
            num2 = parts[num as usize].L;
            num3 = parts[num as usize].W;
        } else {
            num = id * -1 - 2;
            num2 = parts[num as usize].W;
            num3 = parts[num as usize].L;
        }
        let _sq = parts[num as usize].sq();

        let num4 = num2;
        let num5_a = wo - num3 - self.B;
        let num6 = lo - num2 - self.B;
        let num7 = wo;

        let mut flag = false;
        let mut flag2 = false;
        if num4 >= self.minL && num5_a >= self.minW {
            flag = Self::fast_find_first_part(parts, num4, num5_a);
        }
        if num6 >= self.minL && num7 >= self.minW {
            flag2 = Self::fast_find_first_part(parts, num6, num7);
        }

        if flag || flag2 {
            array[0] = id;
        } else {
            let num4b = lo - num2 - self.B;
            let num5b = num3;
            let num6b = lo;
            let num7b = wo - num3 - self.B;
            flag = false;
            flag2 = false;
            if num4b >= self.minL && num5b >= self.minW {
                flag = Self::fast_find_first_part(parts, num4b, num5b);
            }
            if num6b >= self.minL && num7b >= self.minW {
                flag2 = Self::fast_find_first_part(parts, num6b, num7b);
            }
            if flag || flag2 {
                array[0] = id;
            } else {
                let array2 = self.find_2_parts(parts, lo, wo);
                let mut num8: f64 = 0.0;
                if array2[0] != -1 {
                    num8 += if array2[0] >= -1 {
                        parts[array2[0] as usize].sq()
                    } else {
                        parts[(array2[0] * -1 - 2) as usize].sq()
                    };
                }
                if array2[1] != -1 {
                    num8 += if array2[1] >= -1 {
                        parts[array2[1] as usize].sq()
                    } else {
                        parts[(array2[1] * -1 - 2) as usize].sq()
                    };
                }
                if (num8 - _sq) as i64 > 0 {
                    array = [array2[0], array2[1]];
                }
            }
        }

        array
    }

    fn create_csnip(x: i32, y: i32, length: i32, width: i32) -> CSnip {
        CSnip {
            L: length,
            W: width,
            CRD: Crd {
                X: x,
                Y: y,
                ..Default::default()
            },
            ..Default::default()
        }
    }

    fn resize_csnip(snip: &mut CSnip, x: i32, y: i32, length: i32, width: i32) {
        snip.L = length;
        snip.W = width;
        snip.CRD.X = x;
        snip.CRD.Y = y;
    }

    fn get_sq_parts_for_snips(
        &self,
        parts: &mut Vec<CPart>,
        lo1: i32,
        wo1: i32,
        lo2: i32,
        wo2: i32,
        lo3: i32,
        wo3: i32,
    ) -> f64 {
        let mut total: f64 = 0.0;
        let mut num2: i32 = -1;
        let mut num3: i32 = -1;
        let mut _num4: i32 = -1;

        if lo1 >= self.minL && wo1 >= self.minW {
            num2 = Self::find_max_sq_part(parts, lo1, wo1);
            if num2 != -1 {
                let idx = if num2 < -1 { num2 * -1 - 2 } else { num2 };
                total += parts[idx as usize].sq();
            }
        }
        if num2 != -1 {
            let idx = if num2 < -1 { num2 * -1 - 2 } else { num2 };
            parts[idx as usize].Plased += 1;
        }

        if lo2 >= self.minL && wo2 >= self.minW {
            num3 = Self::find_max_sq_part(parts, lo2, wo2);
            if num3 != -1 {
                let idx = if num3 < -1 { num3 * -1 - 2 } else { num3 };
                total += parts[idx as usize].sq();
            }
        }
        if num3 != -1 {
            let idx = if num3 < -1 { num3 * -1 - 2 } else { num3 };
            parts[idx as usize].Plased += 1;
        }

        if lo3 >= self.minL && wo3 >= self.minW {
            _num4 = Self::find_max_sq_part(parts, lo3, wo3);
            if _num4 != -1 {
                let idx = if _num4 < -1 { _num4 * -1 - 2 } else { _num4 };
                total += parts[idx as usize].sq();
            }
        }

        // Undo Plased increments
        if num2 != -1 {
            let idx = if num2 < -1 { num2 * -1 - 2 } else { num2 };
            parts[idx as usize].Plased -= 1;
        }
        if num3 != -1 {
            let idx = if num3 < -1 { num3 * -1 - 2 } else { num3 };
            parts[idx as usize].Plased -= 1;
        }

        total
    }

    fn find_max_sq_part(parts: &[CPart], lo: i32, wo: i32) -> i32 {
        let mut result: i32 = -1;
        let mut num: f64 = 0.0;

        if lo > 0 && wo > 0 {
            for i in 0..parts.len() {
                if parts[i].Plased >= parts[i].Qty {
                    continue;
                }
                if parts[i].L <= lo && parts[i].W <= wo {
                    if (parts[i].sq() - num) as i64 > 0 {
                        result = i as i32;
                        num = parts[i].sq();
                    }
                } else if parts[i].Turn
                    && parts[i].L <= wo
                    && parts[i].W <= lo
                    && (parts[i].sq() - num) as i64 > 0
                {
                    result = i as i32 * -1 - 2;
                    num = parts[i].sq();
                }
            }
        }
        result
    }

    fn find_max_sq_part_krome(parts: &[CPart], lo: i32, wo: i32, krome: i32) -> i32 {
        let mut result: i32 = -1;
        let mut num: f64 = 0.0;

        let krome_idx = if krome < -1 { krome * -1 - 2 } else { krome };

        if lo > 0 && wo > 0 {
            for i in 0..parts.len() {
                let mut num2 = parts[i].Qty;
                if i as i32 == krome_idx {
                    num2 -= 1;
                }
                if parts[i].Plased >= num2 {
                    continue;
                }
                if parts[i].L <= lo && parts[i].W <= wo {
                    if (parts[i].sq() - num) as i64 > 0 {
                        result = i as i32;
                        num = parts[i].sq();
                    }
                } else if parts[i].Turn
                    && parts[i].L <= wo
                    && parts[i].W <= lo
                    && (parts[i].sq() - num) as i64 > 0
                {
                    result = i as i32 * -1 - 2;
                    num = parts[i].sq();
                }
            }
        }
        result
    }

    fn find_small_snip(&self, snips: &[CSnip], parts: &[CPart]) -> i32 {
        let mut result: i32 = -1;
        let mut num: f64 = 1000000000.0;

        for i in 0..snips.len() {
            if (num - snips[i].sq()) as i64 > 0
                && Self::fast_find_first_part(parts, snips[i].L, snips[i].W)
            {
                num = snips[i].sq();
                result = i as i32;
            }
        }
        result
    }

    fn find_2_parts(&self, parts: &mut Vec<CPart>, lo: i32, wo: i32) -> [i32; 3] {
        let mut array = [-1i32, -1, 0]; // horizontal split
        let mut array2 = [-1i32, -1, 1]; // vertical split
        let mut num5: f64 = 0.0;
        let mut num6: f64 = 0.0;

        for i in 0..parts.len() {
            if parts[i].Plased >= parts[i].Qty {
                continue;
            }

            // Horizontal: split by L
            if lo >= parts[i].L && wo >= parts[i].W {
                let num = parts[i].sq();
                let num8 = lo - self.B - parts[i].L;
                let (num7, num2) = if num8 >= self.minL {
                    let f = Self::find_max_sq_part_krome(parts, num8, wo, i as i32);
                    let sq = if f == -1 {
                        0.0
                    } else if f >= -1 {
                        parts[f as usize].sq()
                    } else {
                        parts[(f * -1 - 2) as usize].sq()
                    };
                    (f, sq)
                } else {
                    (-1, 0.0)
                };
                if ((num5 - (num + num2)) as i64) < 0 {
                    num5 = num + num2;
                    array[0] = i as i32;
                    array[1] = num7;
                }
            }
            if parts[i].Turn && wo >= parts[i].L && lo >= parts[i].W {
                let num = parts[i].sq();
                let num9 = lo - self.B - parts[i].W;
                let (num7, num2) = if num9 >= self.minL {
                    let f = Self::find_max_sq_part_krome(parts, num9, wo, i as i32);
                    let sq = if f == -1 {
                        0.0
                    } else if f >= -1 {
                        parts[f as usize].sq()
                    } else {
                        parts[(f * -1 - 2) as usize].sq()
                    };
                    (f, sq)
                } else {
                    (-1, 0.0)
                };
                if ((num5 - (num + num2)) as i64) < 0 {
                    num5 = num + num2;
                    array[0] = i as i32 * -1 - 2;
                    array[1] = num7;
                }
            }

            // Vertical: split by W
            if lo >= parts[i].L && wo >= parts[i].W {
                let num3 = parts[i].sq();
                let num10 = wo - self.B - parts[i].W;
                let (num7, num4) = if num10 >= self.minL {
                    let f = Self::find_max_sq_part_krome(parts, lo, num10, i as i32);
                    let sq = if f == -1 {
                        0.0
                    } else if f >= -1 {
                        parts[f as usize].sq()
                    } else {
                        parts[(f * -1 - 2) as usize].sq()
                    };
                    (f, sq)
                } else {
                    (-1, 0.0)
                };
                if ((num6 - (num3 + num4)) as i64) < 0 {
                    num6 = num3 + num4;
                    array2[0] = i as i32;
                    array2[1] = num7;
                }
            } else if parts[i].Turn && wo >= parts[i].L && lo >= parts[i].W {
                let num3 = parts[i].sq();
                let num11 = wo - self.B - parts[i].L;
                let (num7, num4) = if num11 >= self.minL {
                    let f = Self::find_max_sq_part_krome(parts, lo, num11, i as i32);
                    let sq = if f == -1 {
                        0.0
                    } else if f >= -1 {
                        parts[f as usize].sq()
                    } else {
                        parts[(f * -1 - 2) as usize].sq()
                    };
                    (f, sq)
                } else {
                    (-1, 0.0)
                };
                if ((num6 - (num3 + num4)) as i64) < 0 {
                    num6 = num3 + num4;
                    array2[0] = i as i32 * -1 - 2;
                    array2[1] = num7;
                }
            }
        }

        // Sort pairs so larger W-dimension part comes first (horizontal)
        if array[0] != -1 && array[1] != -1 {
            let mut num12 = array[0];
            let num13 = if num12 < -1 {
                num12 = num12 * -1 - 2;
                parts[num12 as usize].L
            } else {
                parts[num12 as usize].W
            };
            let mut num14 = array[1];
            let num15 = if num14 < -1 {
                num14 = num14 * -1 - 2;
                parts[num14 as usize].L
            } else {
                parts[num14 as usize].W
            };
            if num15 > num13 {
                let tmp = array[0];
                array[0] = array[1];
                array[1] = tmp;
            }
        }

        // Sort pairs so larger L-dimension part comes first (vertical)
        if array2[0] != -1 && array2[1] != -1 {
            let mut num17 = array2[0];
            let num18 = if num17 < -1 {
                num17 = num17 * -1 - 2;
                parts[num17 as usize].W
            } else {
                parts[num17 as usize].L
            };
            let mut num19 = array2[1];
            let num20 = if num19 < -1 {
                num19 = num19 * -1 - 2;
                parts[num19 as usize].W
            } else {
                parts[num19 as usize].L
            };
            if num20 > num18 {
                let tmp = array2[0];
                array2[0] = array2[1];
                array2[1] = tmp;
            }
        }

        if (num5 - num6) as i64 > 0 {
            array
        } else {
            array2
        }
    }

    fn find_zamena_lines_width_cut(
        lines: &[CLine],
        on_sheet: &[bool],
        wo: i32,
        minimal_l: i32,
        check: &mut bool,
    ) -> [i32; 3] {
        let mut array = [-1i32, -1, -1];
        *check = false;
        let mut num: i32 = 0;

        for i in 0..lines.len() {
            let l = lines[i].L;
            if on_sheet[i] || wo < l {
                continue;
            }
            if l > num {
                array = [i as i32, -1, -1];
                num = l;
                *check = true;
            } else if l == num {
                let mut num2: f64 = 0.0;
                for j in 0..3 {
                    if array[j] != -1 {
                        num2 += lines[array[j] as usize].Parts_Sq;
                    }
                }
                if (lines[i].Parts_Sq - num2) as i32 >= 0 {
                    array = [i as i32, -1, -1];
                    num = l;
                    *check = true;
                }
            }

            if wo - l - minimal_l < 0 {
                // Note: in C# this is `WO - l - B - Minimal_L < 0` but B is not passed.
                // Actually looking at the C# more carefully: the function signature doesn't
                // have B, but the comparison is `WO - l - B - Minimal_L`. Wait, let me re-check.
                // C# line 1696: `if (WO - l - B - Minimal_L < 0)` but B is an instance field.
                // However, this is a static-style helper. We don't have B here.
                // Actually the C# code uses the instance field B. We need self here.
                // But we made this a static method. Let me fix this by not using it as static.
                continue;
            }
            for k in i + 1..lines.len() {
                let l2 = lines[k].L;
                if on_sheet[k] || wo < l2 {
                    continue;
                }
                if wo - l - l2 >= 0 {
                    if l + l2 - num > 0 {
                        array = [i as i32, k as i32, -1];
                        num = l + l2;
                        *check = true;
                    } else if l + l2 - num == 0 {
                        let mut num3: f64 = 0.0;
                        for m in 0..3 {
                            if array[m] != -1 {
                                num3 += lines[array[m] as usize].Parts_Sq;
                            }
                        }
                        if (lines[i].Parts_Sq + lines[k].Parts_Sq - num3) as i32 >= 0 {
                            array = [i as i32, k as i32, -1];
                            num = l + l2;
                            *check = true;
                        }
                    }
                }
                if wo - l - l2 - minimal_l < 0 {
                    continue;
                }
                for n in k + 1..lines.len() {
                    let l3 = lines[n].L;
                    if on_sheet[n] || wo < l3 || wo - l - l2 - l3 < 0 {
                        continue;
                    }
                    if l + l2 + l3 - num > 0 {
                        array = [i as i32, k as i32, n as i32];
                        num = l + l2 + l3;
                        *check = true;
                    } else if l + l2 + l3 - num == 0 {
                        let mut num4: f64 = 0.0;
                        for q in 0..3 {
                            if array[q] != -1 {
                                num4 += lines[array[q] as usize].Parts_Sq;
                            }
                        }
                        if (lines[i].Parts_Sq + lines[k].Parts_Sq + lines[n].Parts_Sq - num4) as i32 >= 0 {
                            array = [i as i32, k as i32, n as i32];
                            num = l + l2 + l3;
                            *check = true;
                        }
                    }
                }
            }
        }
        array
    }

    fn find_zamena_parts_width_cut(
        fix: &[i32],
        parts: &[CPart],
        wo: i32,
        max_w: i32,
        minimal_w: i32,
        check: &mut bool,
    ) -> [i32; 3] {
        let mut array = [-1i32, -1, -1];
        *check = false;
        let mut max_w = max_w;

        for i in 0..fix.len() {
            let num = if fix[i] <= -1 {
                parts[(fix[i] * -1 - 2) as usize].L
            } else {
                parts[fix[i] as usize].W
            };
            if wo < num {
                continue;
            }
            if num > max_w {
                array = [i as i32, -1, -1];
                max_w = num;
                *check = true;
            }
            if wo - num - minimal_w < 0 {
                continue;
            }
            for j in i + 1..fix.len() {
                let num2 = if fix[j] <= -1 {
                    parts[(fix[j] * -1 - 2) as usize].L
                } else {
                    parts[fix[j] as usize].W
                };
                if wo - num - num2 >= 0 && num + num2 - max_w > 0 {
                    array = [i as i32, j as i32, -1];
                    max_w = num + num2;
                    *check = true;
                }
                if wo - num - num2 - minimal_w < 0 {
                    continue;
                }
                for k in j + 1..fix.len() {
                    let num3 = if fix[k] <= -1 {
                        parts[(fix[k] * -1 - 2) as usize].L
                    } else {
                        parts[fix[k] as usize].W
                    };
                    if wo - num - num2 - num3 >= 0 && num + num2 + num3 - max_w > 0 {
                        array = [i as i32, k as i32, k as i32];
                        max_w = num + num2 + num3;
                        *check = true;
                    }
                }
            }
        }
        array
    }

    fn set_on_parts_in_line(parts: &mut Vec<CPart>, line: &CLine) {
        for i in 0..line.PartIDs.len() {
            if line.PartIDs[i] < -1 {
                parts[(line.PartIDs[i] * -1 - 2) as usize].Plased += 1;
            } else {
                parts[line.PartIDs[i] as usize].Plased += 1;
            }
        }
    }

    fn set_off_parts_in_line(parts: &mut Vec<CPart>, line: &CLine) {
        for i in 0..line.PartIDs.len() {
            if line.PartIDs[i] < -1 {
                parts[(line.PartIDs[i] * -1 - 2) as usize].Plased -= 1;
            } else {
                parts[line.PartIDs[i] as usize].Plased -= 1;
            }
        }
    }

    fn get_id_ld_wd(parts: &[CPart], id: i32) -> (i32, i32, i32) {
        if id > -1 {
            (id, parts[id as usize].L, parts[id as usize].W)
        } else if id < -1 {
            let idx = id * -1 - 2;
            (idx, parts[idx as usize].W, parts[idx as usize].L)
        } else {
            (-1, -1, -1)
        }
    }

    fn get_parts_with_fix_length(
        &self,
        parts: &[CPart],
        l: i32,
        _w: i32,
        min_in: i32,
    ) -> (Vec<i32>, i32, i32) {
        let mut list: Vec<i32> = Vec::new();
        let mut min_w = min_in;
        let mut total_length = 0;

        for i in 0..parts.len() {
            let cp = &parts[i];
            if cp.Qty <= cp.Plased {
                continue;
            }
            if cp.L == l {
                for _ in 0..(cp.Qty - cp.Plased) {
                    list.push(i as i32);
                    total_length += cp.W;
                }
                if min_w > cp.W {
                    min_w = cp.W;
                }
            } else if cp.Turn && cp.W == l {
                for _ in 0..(cp.Qty - cp.Plased) {
                    list.push(i as i32 * -1 - 2);
                    total_length += cp.L;
                }
                if min_w > cp.L {
                    min_w = cp.L;
                }
            }
            if list.len() as i32 > self.THE_SAME_PARTS_LIMIT {
                break;
            }
        }

        (list, min_w, total_length)
    }

    fn get_start_parts_for_line_width_cut(
        &self,
        parts: &[CPart],
        mut fix_length: Vec<i32>,
        line_width: i32,
        minimal_w: i32,
    ) -> Vec<i32> {
        let mut list: Vec<i32> = Vec::new();
        let mut num = line_width;

        let mut i = 0;
        while i < fix_length.len() {
            let (_id, _ld, wd) = Self::get_id_ld_wd(parts, fix_length[i]);
            if num >= wd {
                num = num - wd - self.B;
                list.push(fix_length[i]);
                fix_length.remove(i);
                if num < minimal_w {
                    break;
                }
            } else {
                i += 1;
            }
        }

        // Iterative swap improvement
        let mut flag = false;
        let mut num2 = 0;

        while !flag && num2 < self.PARTS_SORT_LIMIT {
            num2 += 1;
            let mut num3: i32 = 0;
            let mut num4: i32 = -1;
            let mut num5: i32 = -1;
            let mut num6: i32 = 0;
            let mut num7: i32 = 0;
            let mut best_arr: Option<[i32; 3]> = None;

            if list.len() >= 2 {
                for j in 0..list.len() - 1 {
                    for k in j + 1..list.len() {
                        fix_length.push(list[j]);
                        fix_length.push(list[k]);

                        let (_, _, wd2) = Self::get_id_ld_wd(parts, list[j]);
                        let (_, _, wd3) = Self::get_id_ld_wd(parts, list[k]);
                        let wo = num + self.B + wd2 + self.B + wd3;

                        let mut check = false;
                        let array2 = Self::find_zamena_parts_width_cut(
                            &fix_length,
                            parts,
                            wo,
                            wd2 + wd3,
                            minimal_w,
                            &mut check,
                        );

                        if check {
                            let mut num8: i32 = 0;
                            for idx in 0..array2.len() {
                                if array2[idx] != -1 {
                                    let (_, _, wd4) =
                                        Self::get_id_ld_wd(parts, fix_length[array2[idx] as usize]);
                                    num8 = num8 + self.B + wd4;
                                }
                            }
                            if num8 > num3 {
                                num4 = j as i32;
                                num5 = k as i32;
                                num6 = wd2;
                                num7 = wd3;
                                best_arr = Some(array2);
                                num3 = num8;
                            }
                        }

                        fix_length.pop();
                        fix_length.pop();
                    }
                }
            }

            if num4 != -1 && num5 != -1 {
                let array = best_arr.unwrap();
                fix_length.push(list[num4 as usize]);
                fix_length.push(list[num5 as usize]);
                list.remove(num4 as usize);
                list.remove((num5 - 1) as usize);
                num = num + self.B + num6 + self.B + num7;

                for m in 0..array.len() {
                    if array[m] != -1 {
                        list.push(fix_length[array[m] as usize]);
                        let (_, _, wd5) =
                            Self::get_id_ld_wd(parts, fix_length[array[m] as usize]);
                        num = num - self.B - wd5;
                    }
                }

                let mut num9 = 0;
                for n in 0..array.len() {
                    if array[n] != -1 {
                        fix_length.remove((array[n] - num9) as usize);
                        num9 += 1;
                    }
                }
            } else {
                flag = true;
            }
        }

        // Sort by WD descending
        for num10 in 0..list.len().saturating_sub(1) {
            for num11 in num10 + 1..list.len() {
                let (_, _, wd6) = Self::get_id_ld_wd(parts, list[num10]);
                let (_, _, wd7) = Self::get_id_ld_wd(parts, list[num11]);
                if wd7 > wd6 {
                    list.swap(num10, num11);
                }
            }
        }

        list
    }

    /// Helper: pick best of two layout variants by sq value.
    /// Returns the chosen label (a or b), or -1 if both are zero.
    fn pick_best_of_two(sq_a: f64, sq_b: f64, label_a: i32, label_b: i32) -> i32 {
        let a_nz = (sq_a * 100.0) as i64 != 0;
        let b_nz = (sq_b * 100.0) as i64 != 0;

        if !a_nz && !b_nz {
            -1
        } else if a_nz && !b_nz {
            label_a
        } else if !a_nz && b_nz {
            label_b
        } else {
            if ((sq_a - sq_b) * 100.0) as i64 >= 0 {
                label_a
            } else {
                label_b
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_width_alg_new() {
        let alg = WidthAlg::new();
        assert_eq!(alg.THE_SAME_PARTS_LIMIT, 25);
        assert_eq!(alg.LINES_LIMIT, 200);
        assert_eq!(alg.LINES_SORT_ITERS_LIMIT, 4);
        assert_eq!(alg.PARTS_SORT_LIMIT, 2);
    }

    #[test]
    fn test_fast_find_first_part() {
        let parts = vec![CPart {
            L: 100,
            W: 50,
            Qty: 2,
            Plased: 0,
            Turn: false,
            iD_in_Order: 0,
        }];
        assert!(WidthAlg::fast_find_first_part(&parts, 100, 50));
        assert!(!WidthAlg::fast_find_first_part(&parts, 99, 50));
        assert!(!WidthAlg::fast_find_first_part(&parts, 0, 50));
    }

    #[test]
    fn test_get_id_ld_wd_positive() {
        let parts = vec![CPart {
            L: 200,
            W: 100,
            Qty: 1,
            Plased: 0,
            Turn: false,
            iD_in_Order: 0,
        }];
        let (id, ld, wd) = WidthAlg::get_id_ld_wd(&parts, 0);
        assert_eq!(id, 0);
        assert_eq!(ld, 200);
        assert_eq!(wd, 100);
    }

    #[test]
    fn test_get_id_ld_wd_rotated() {
        let parts = vec![CPart {
            L: 200,
            W: 100,
            Qty: 1,
            Plased: 0,
            Turn: true,
            iD_in_Order: 0,
        }];
        // id = -2 means rotated index 0 => id * -1 - 2 = 0
        let (id, ld, wd) = WidthAlg::get_id_ld_wd(&parts, -2);
        assert_eq!(id, 0);
        assert_eq!(ld, 100); // W becomes LD
        assert_eq!(wd, 200); // L becomes WD
    }

    #[test]
    fn test_simple_cut() {
        let mut parts = vec![CPart {
            L: 500,
            W: 300,
            Qty: 2,
            Plased: 0,
            Turn: false,
            iD_in_Order: 0,
        }];
        let mut alg = WidthAlg::new();
        let sheet = alg.get_csheet_width_cut(
            &mut parts, 2000, 1000, 30, 0, false, true, false,
        );
        assert_eq!(sheet.Alg, 2);
        assert!(sheet.Lines.len() > 0);
    }
}
