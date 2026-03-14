#![allow(non_snake_case)]
#![allow(unused_assignments)]
#![allow(clippy::needless_range_loop)]
#![allow(clippy::comparison_chain)]

use super::algorithm_types::*;
use std::time::Instant;

pub struct Length2 {
    pub THE_SAME_PARTS_LIMIT: i32,
    pub LINES_LIMIT: i32,
    pub LINES_SORT_ITERS_LIMIT: i32,
    pub PARTS_SORT_LIMIT: i32,
    pub TIME_GET_LINES_LIMIT: f64,

    minL: i32,
    minW: i32,
    P: i32,
    B: i32,
    L_L: i32,
    L_W: i32,
}

impl Length2 {
    pub fn new() -> Self {
        Self {
            THE_SAME_PARTS_LIMIT: 30,
            LINES_LIMIT: 200,
            LINES_SORT_ITERS_LIMIT: 4,
            PARTS_SORT_LIMIT: 2,
            TIME_GET_LINES_LIMIT: 1.0,
            minL: 0,
            minW: 0,
            P: 0,
            B: 0,
            L_L: 0,
            L_W: 0,
        }
    }

    pub fn get_cparts_sq(parts: &[CPart]) -> f64 {
        let mut num = 0.0;
        for i in 0..parts.len() {
            num += parts[i].sq() * parts[i].Qty as f64;
        }
        num
    }

    /// Entry point. Returns (CSheet, PPSQ_OUT).
    pub fn get_csheet_length_cut(
        &mut self,
        parts: &mut Vec<CPart>,
        list_length: i32,
        list_width: i32,
        blade: i32,
        padding: i32,
        double_padding: bool,
        params: &LW16,
        _psq: f64,
        ppsq: f64,
    ) -> (Option<CSheet>, f64) {
        self.L_L = list_length;
        self.L_W = list_width;
        self.P = padding;
        self.B = blade;

        let mut csheet = CSheet::default();
        csheet.Alg = 1;
        csheet.Lines = Vec::new();
        csheet.L = self.L_L;
        csheet.W = self.L_W;

        let mut num = self.P;
        if double_padding {
            num *= 2;
        }
        let num2 = csheet.L - num;
        let num3 = csheet.W - num;

        let (clines, mut on_sheet_flags) =
            self.get_clines_length_cut(parts, num2, num3, params);

        let mut num4 = num3;
        let mut _num5 = 0i32;
        for i in 0..clines.len() {
            if num4 > clines[i].W {
                num4 = clines[i].W;
            }
            _num5 += clines[i].W;
        }

        let mut lines_index: Vec<usize> = Vec::new();
        let mut num6 = num3;
        for j in 0..clines.len() {
            if num6 >= clines[j].W {
                num6 = num6 - clines[j].W - self.B;
                csheet.Lines.push(clines[j].clone());
                lines_index.push(j);
                on_sheet_flags[j] = true;
                if num4 >= num6 {
                    break;
                }
            }
        }

        // Lines sort / zamena loop
        let mut flag = false;
        let mut num7 = 0;
        while !flag && num7 < self.LINES_SORT_ITERS_LIMIT {
            num7 += 1;
            let mut _check = false;
            let mut num8: i32 = -1;
            let mut num9: i32 = -1;
            let mut array: Option<[i32; 3]> = None;
            let mut num10: f64 = 0.0;

            let sheet_lines_count = csheet.Lines.len() as i32;
            if sheet_lines_count > 1 {
                for k in 0..(sheet_lines_count - 1) as usize {
                    for l in (k + 1)..sheet_lines_count as usize {
                        on_sheet_flags[lines_index[k]] = false;
                        on_sheet_flags[lines_index[l]] = false;

                        let w_o =
                            num6 + self.B + csheet.Lines[k].W + self.B + csheet.Lines[l].W;
                        let (array2, _check2) = self.find_zamena_lines_length_cut(
                            &clines,
                            &on_sheet_flags,
                            w_o,
                            num4,
                        );

                        if lines_index[k] as i32 != array2[0]
                            || lines_index[l] as i32 != array2[1]
                            || array2[2] != -1
                        {
                            let num11 = csheet.Lines[k].W + csheet.Lines[l].W;
                            let num12 = csheet.Lines[k].Parts_Sq + csheet.Lines[l].Parts_Sq;
                            let mut num13 = 0i32;
                            let mut num14 = 0.0f64;
                            for m in 0..3 {
                                if array2[m] != -1 {
                                    num14 += clines[array2[m] as usize].Parts_Sq;
                                    num13 += clines[array2[m] as usize].W;
                                }
                            }

                            if num13 >= num11
                                && (num14 - num12) as i64 >= 0
                                && ((num14 - num10) as i64 > 0
                                    || ((num14 - num10) as i64 == 0
                                        && num13
                                            - (csheet.Lines[num8 as usize].W
                                                + csheet.Lines[num9 as usize].W)
                                            > 0))
                            {
                                num8 = k as i32;
                                num9 = l as i32;
                                array = Some(array2);
                                num10 = num14;
                            }
                        }

                        on_sheet_flags[lines_index[k]] = true;
                        on_sheet_flags[lines_index[l]] = true;
                    }
                }
            }

            if num8 != -1 && num9 != -1 {
                num6 = num6
                    + self.B
                    + csheet.Lines[num8 as usize].W
                    + self.B
                    + csheet.Lines[num9 as usize].W;
                on_sheet_flags[lines_index[num8 as usize]] = false;
                on_sheet_flags[lines_index[num9 as usize]] = false;

                csheet.Lines.remove(num8 as usize);
                csheet.Lines.remove(num9 as usize - 1);
                lines_index.remove(num8 as usize);
                lines_index.remove(num9 as usize - 1);

                if let Some(arr) = array {
                    for n in 0..3 {
                        if arr[n] != -1 {
                            csheet.Lines.push(clines[arr[n] as usize].clone());
                            lines_index.push(arr[n] as usize);
                            on_sheet_flags[arr[n] as usize] = true;
                            num6 = num6 - self.B - clines[arr[n] as usize].W;
                        }
                    }
                }
            } else {
                flag = true;
            }
        }

        // Remain
        csheet.Remain = CSnip {
            L: num2,
            W: num6,
            ..CSnip::default()
        };

        // Sort lines by W descending (bubble sort)
        for num15 in 0..csheet.Lines.len().saturating_sub(1) {
            for num16 in (num15 + 1)..csheet.Lines.len() {
                if csheet.Lines[num16].W > csheet.Lines[num15].W {
                    lines_index.swap(num15, num16);
                    csheet.Lines.swap(num15, num16);
                }
            }
        }

        // SET_OFF parts for lines not on sheet
        for idx in 0..clines.len() {
            if !on_sheet_flags[idx] {
                Self::set_off_parts_in_line(parts, &clines[idx]);
            }
        }

        // Continue lines and accumulate Parts_Sq
        csheet.Parts_Sq = 0.0;
        for num18 in (0..csheet.Lines.len()).rev() {
            self.continue_line_length_cut(&mut csheet.Lines[num18], parts, params);
            csheet.Parts_Sq += csheet.Lines[num18].Parts_Sq;
        }

        // Try to fill remaining space with new lines
        flag = false;
        while !flag {
            if self.fast_find_first_part(parts, csheet.Remain.L, csheet.Remain.W) {
                let mut cline = CLine::default();
                cline.Snips = Vec::new();
                cline.PartIDs = Vec::new();
                cline.Parts_Crds = Vec::new();

                let num19 = self.find_width_part(parts, csheet.Remain.L, csheet.Remain.W, true);
                let (_id, _ld, wd) = Self::get_id_ld_wd(parts, num19);
                cline.W = wd;
                cline.L = csheet.Remain.L;

                let csnip = CSnip {
                    CRD: Crd { X: 0, Y: 0, ..Crd::default() },
                    L: cline.L,
                    W: cline.W,
                    ..CSnip::default()
                };
                cline.Snips.push(csnip);

                let io = self.find_small_snip(&cline.Snips, parts);
                self.place_part_to_line(&mut cline, parts, num19, io, false);
                self.continue_line_length_cut(&mut cline, parts, params);
                csheet.Remain.W = csheet.Remain.W - self.B - cline.W;
                csheet.Parts_Sq += cline.Parts_Sq;
                csheet.Lines.push(cline);
            } else {
                flag = true;
            }
        }

        let ppsq_out = ppsq + csheet.Parts_Sq;
        (Some(csheet), ppsq_out)
    }

    fn continue_line_length_cut(
        &self,
        line: &mut CLine,
        parts: &mut Vec<CPart>,
        params: &LW16,
    ) {
        let mut num = -1i32;
        loop {
            let num2 = self.find_small_snip(&line.Snips, parts);
            if num2 < 0 {
                break;
            }
            let l = line.Snips[num2 as usize].L;
            let w = line.Snips[num2 as usize].W;
            num = if !params.MAX_SQ {
                self.find_width_part(parts, l, w, true)
            } else {
                self.find_max_sq_part(parts, l, w)
            };
            if num != -1 {
                if params.OPTI_ON {
                    let array = self.check_part_for_last_in_line(parts, l, w, num);
                    if num != array[0] && array[0] != -1 {
                        self.place_2_parts_to_line(line, parts, &array, num2);
                    } else {
                        self.place_part_to_line(line, parts, num, num2, false);
                    }
                } else {
                    self.place_part_to_line(line, parts, num, num2, false);
                }
            } else {
                break;
            }
        }
    }

    /// Returns (lines, on_sheet_flags)
    fn get_clines_length_cut(
        &mut self,
        parts: &mut Vec<CPart>,
        ll: i32,
        lw: i32,
        params: &LW16,
    ) -> (Vec<CLine>, Vec<bool>) {
        let start_time = Instant::now();
        let mut list: Vec<CLine> = Vec::new();
        let mut on_sheet: Vec<bool> = Vec::new();
        let mut num = -1i32;
        let mut num2 = 0;
        let mut flag = false;
        let mut num3 = ll;
        let mut num4 = lw;

        while !flag && num2 < self.LINES_LIMIT {
            num2 += 1;
            let mut cline: Option<CLine> = None;
            let mut cline2: Option<CLine> = None;

            num = if !params.MAX_SQ {
                self.find_width_part(parts, num3, num4, false)
            } else {
                self.find_max_sq_part(parts, ll, lw)
            };

            if num != -1 {
                cline = Some(self.make_line_length_cut(parts, num, num3, num4, params));

                if params.TURN_ON {
                    let mut index = num;
                    if num < -1 {
                        index = num * -1 - 2;
                    }
                    let idx = index as usize;
                    if parts[idx].Turn
                        && ((num < -1
                            && num3 >= parts[idx].L
                            && num4 >= parts[idx].W)
                            || (num > -1
                                && num3 >= parts[idx].W
                                && num4 >= parts[idx].L))
                    {
                        let turned_id = num * -1 - 2;
                        cline2 = Some(self.make_line_length_cut(
                            parts, turned_id, num3, num4, params,
                        ));
                    }

                    if let (Some(ref c1), Some(ref c2)) = (&cline, &cline2) {
                        if ((c1.filling() - c2.filling()) * 100.0) < 0.0 {
                            cline = cline2.take();
                        }
                    }
                }
            }

            if let Some(ref cl) = cline {
                Self::set_on_parts_in_line(parts, cl);
                on_sheet.push(false);
                list.push(cl.clone());
                num4 = num4 - self.B - cl.W;
                if !self.fast_find_first_part(parts, num3, num4) {
                    num3 = ll;
                    num4 = lw;
                    if !self.fast_find_first_part(parts, num3, num4) {
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

    fn make_line_length_cut(
        &self,
        parts: &mut Vec<CPart>,
        start_part: i32,
        line_length: i32,
        _line_width: i32,
        params: &LW16,
    ) -> CLine {
        let mut cline = CLine::default();
        cline.Snips = Vec::new();
        cline.PartIDs = Vec::new();
        cline.Parts_Crds = Vec::new();

        let (_id, _ld, wd) = Self::get_id_ld_wd(parts, start_part);
        let rez = false;
        cline.W = wd;
        cline.L = line_length;

        let csnip = CSnip {
            CRD: Crd { X: 0, Y: 0, ..Crd::default() },
            L: cline.L,
            W: cline.W,
            ..CSnip::default()
        };
        cline.Snips.push(csnip);

        let mut io = 0i32;
        if params.SAME_MAX {
            let (fix_width, min_l, _total_length) =
                self.get_parts_with_fix_width(parts, wd, cline.W, params.TURN_ON);
            let start_parts = self.get_start_parts_for_line_length_cut(
                parts,
                fix_width,
                cline.L,
                min_l,
            );
            io = self.find_small_snip(&cline.Snips, parts);
            for i in 0..start_parts.len() {
                self.place_part_to_line(&mut cline, parts, start_parts[i], io, rez);
            }
        } else {
            self.place_part_to_line(&mut cline, parts, start_part, io, rez);
            let mut flag = false;
            let mut _num = -1i32;
            io = self.find_small_snip(&cline.Snips, parts);
            if io != -1 {
                while !flag {
                    let l = cline.Snips[io as usize].L;
                    let w = cline.Snips[io as usize].W;
                    _num = self.find_the_same_width_part(parts, l, w, params.TURN_ON);
                    if _num != -1 {
                        self.place_part_to_line(&mut cline, parts, _num, io, rez);
                    } else {
                        flag = true;
                    }
                }
            }
        }

        // PreCut not needed (returned but unused by caller in this port)
        let _pre_cut = Self::copy_line_without_marks(&cline);
        self.continue_line_length_cut(&mut cline, parts, params);
        Self::set_off_parts_in_line(parts, &cline);
        cline
    }

    fn copy_line_without_marks(line: &CLine) -> CLine {
        let mut c = CLine::default();
        c.Snips = Vec::new();
        c.PartIDs = Vec::new();
        c.Parts_Crds = Vec::new();
        c.L = line.L;
        c.W = line.W;
        c.Parts_Sq = line.Parts_Sq;
        for i in 0..line.PartIDs.len() {
            c.PartIDs.push(line.PartIDs[i]);
            c.Parts_Crds.push(Crd {
                X: line.Parts_Crds[i].X,
                Y: line.Parts_Crds[i].Y,
                id_in_order: line.Parts_Crds[i].id_in_order,
            });
        }
        for j in 0..line.Snips.len() {
            let s = CSnip {
                L: line.Snips[j].L,
                W: line.Snips[j].W,
                CRD: Crd {
                    X: line.Snips[j].CRD.X,
                    Y: line.Snips[j].CRD.Y,
                    ..Crd::default()
                },
                ..CSnip::default()
            };
            c.Snips.push(s);
        }
        c
    }

    fn find_2_lines(
        &self,
        lines: &[CLine],
        on_sheet: &[bool],
        size: i32,
        sq: f64,
        rez: bool,
    ) -> ([i32; 2], bool, f64) {
        let mut array = [-1i32, -1];
        let mut check = false;
        let mut num: i32 = -1;
        let mut num2: i32 = -1;
        let mut sq_zamena = 0.0f64;
        let mut num3: i32;
        let mut num4: i32;
        let mut num5 = sq;

        for i in 0..lines.len() {
            if on_sheet[i] {
                continue;
            }
            num3 = if rez { lines[i].L } else { lines[i].W };
            if size < num3 {
                continue;
            }
            for j in 0..lines.len() {
                if i != j && !on_sheet[j] {
                    num4 = if rez { lines[j].L } else { lines[j].W };
                    if size - (num3 + self.B + num4) >= 0
                        && (lines[i].Parts_Sq + lines[j].Parts_Sq - num5) as i64 > 0
                    {
                        num5 = lines[i].Parts_Sq + lines[j].Parts_Sq;
                        num = i as i32;
                        num2 = j as i32;
                        check = true;
                    }
                }
            }
        }
        array[0] = num;
        array[1] = num2;
        if check {
            sq_zamena = lines[num as usize].Parts_Sq + lines[num2 as usize].Parts_Sq;
        }
        (array, check, sq_zamena)
    }

    fn fast_find_first_part(&self, parts: &[CPart], lo: i32, wo: i32) -> bool {
        let mut result = false;
        if lo > 0 && wo > 0 {
            for num in (0..parts.len()).rev() {
                if parts[num].Plased < parts[num].Qty
                    && ((lo >= parts[num].L && wo >= parts[num].W)
                        || (parts[num].Turn && lo >= parts[num].W && wo >= parts[num].L))
                {
                    result = true;
                    break;
                }
            }
        }
        result
    }

    fn find_width_part(&self, parts: &[CPart], lo: i32, wo: i32, turn_on: bool) -> i32 {
        let mut result = -1i32;
        let mut num = 0i32;
        let mut num2 = 0.0f64;
        for i in 0..parts.len() {
            if parts[i].Plased >= parts[i].Qty {
                continue;
            }
            if !parts[i].Turn {
                if lo >= parts[i].L && wo >= parts[i].W {
                    if parts[i].W > num {
                        num2 = parts[i].sq();
                        num = parts[i].W;
                        result = i as i32;
                    } else if parts[i].W == num && (parts[i].sq() - num2) as i64 > 0 {
                        num2 = parts[i].sq();
                        num = parts[i].W;
                        result = i as i32;
                    }
                }
            } else {
                // parts[i].Turn == true
                let mut num3 = 0i32;
                if lo >= parts[i].L && wo >= parts[i].W && lo >= parts[i].W && wo >= parts[i].L {
                    num3 = if turn_on {
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
                    num3 = parts[i].W;
                } else if (lo < parts[i].L || wo < parts[i].W)
                    && lo >= parts[i].W
                    && wo >= parts[i].L
                {
                    num3 = parts[i].L;
                }
                if num3 > num {
                    num2 = parts[i].sq();
                    num = num3;
                    result = if parts[i].W != num3 {
                        -1 * i as i32 - 2
                    } else {
                        i as i32
                    };
                } else if num3 == num && (parts[i].sq() - num2) as i64 > 0 {
                    num2 = parts[i].sq();
                    num = num3;
                    result = if parts[i].W != num3 {
                        -1 * i as i32 - 2
                    } else {
                        i as i32
                    };
                }
            }
        }
        result
    }

    fn find_the_same_width_part(
        &self,
        parts: &[CPart],
        lo: i32,
        wo: i32,
        turn_on: bool,
    ) -> i32 {
        let mut result = -1i32;
        let mut num = 0.0f64;
        for i in 0..parts.len() {
            if parts[i].Plased >= parts[i].Qty {
                continue;
            }
            if !turn_on {
                if !parts[i].Turn {
                    if lo >= parts[i].L && wo == parts[i].W && (parts[i].sq() - num) as i64 > 0 {
                        num = parts[i].sq();
                        result = i as i32;
                    }
                } else {
                    // parts[i].Turn == true, !turn_on
                    if parts[i].W > parts[i].L
                        && lo >= parts[i].W
                        && wo == parts[i].L
                    {
                        if (parts[i].sq() - num) as i64 > 0 {
                            num = parts[i].sq();
                            result = i as i32 * -1 - 2;
                        }
                    } else if parts[i].L > parts[i].W
                        && lo >= parts[i].L
                        && wo == parts[i].W
                        && (parts[i].sq() - num) as i64 > 0
                    {
                        num = parts[i].sq();
                        result = i as i32;
                    }
                }
            } else {
                if lo >= parts[i].L && wo == parts[i].W {
                    if (parts[i].sq() - num) as i64 > 0 {
                        num = parts[i].sq();
                        result = i as i32;
                    }
                } else if parts[i].Turn
                    && lo >= parts[i].W
                    && wo == parts[i].L
                    && (parts[i].sq() - num) as i64 > 0
                {
                    num = parts[i].sq();
                    result = i as i32 * -1 - 2;
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
                if self.fast_find_first_part(parts, num, num2) {
                    let item = Self::create_csnip(x, y, num, num2);
                    line.Snips.push(item);
                    let sx = line.Snips[io].CRD.X;
                    let sy = line.Snips[io].CRD.Y + wd + self.B;
                    Self::resize_csnip(&mut line.Snips[io], sx, sy, ld, w - wd - self.B);
                } else {
                    let item2 = Self::create_csnip(x, y, num, wd);
                    line.Snips.push(item2);
                    let sx = line.Snips[io].CRD.X;
                    let sy = line.Snips[io].CRD.Y + wd + self.B;
                    Self::resize_csnip(&mut line.Snips[io], sx, sy, l, w - wd - self.B);
                }
            } else {
                let num3 = l;
                let num4 = w - wd - self.B;
                let x2 = line.Snips[io].CRD.X;
                let y2 = line.Snips[io].CRD.Y + wd + self.B;
                if self.fast_find_first_part(parts, num3, num4) {
                    let item3 = Self::create_csnip(x2, y2, num3, num4);
                    line.Snips.push(item3);
                    let sx = line.Snips[io].CRD.X + ld + self.B;
                    let sy = line.Snips[io].CRD.Y;
                    Self::resize_csnip(&mut line.Snips[io], sx, sy, l - ld - self.B, wd);
                } else {
                    let item4 = Self::create_csnip(x2, y2, ld, num4);
                    line.Snips.push(item4);
                    let sx = line.Snips[io].CRD.X + ld + self.B;
                    let sy = line.Snips[io].CRD.Y;
                    Self::resize_csnip(&mut line.Snips[io], sx, sy, l - ld - self.B, w);
                }
            }
        } else if ld == l && wd < w {
            let sx = line.Snips[io].CRD.X;
            let sy = line.Snips[io].CRD.Y + wd + self.B;
            Self::resize_csnip(&mut line.Snips[io], sx, sy, l, w - wd - self.B);
        } else if ld < l && wd == w {
            let sx = line.Snips[io].CRD.X + ld + self.B;
            let sy = line.Snips[io].CRD.Y;
            Self::resize_csnip(&mut line.Snips[io], sx, sy, l - ld - self.B, w);
        } else if ld == l && wd == w {
            let sx = line.Snips[io].CRD.X;
            let sy = line.Snips[io].CRD.Y;
            Self::resize_csnip(&mut line.Snips[io], sx, sy, 0, 0);
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
        let mut num = 0i32;
        let mut num2 = 0i32;
        let mut num3 = _2parts[0];
        let mut num4 = _2parts[1];

        let num5: i32;
        let num6: i32;
        if num3 >= 0 {
            num5 = parts[num3 as usize].L;
            num6 = parts[num3 as usize].W;
        } else {
            num3 = num3 * -1 - 2;
            num5 = parts[num3 as usize].W;
            num6 = parts[num3 as usize].L;
        }
        line.Parts_Sq += parts[num3 as usize].sq();

        if num4 != -1 {
            if num4 >= 0 {
                num = parts[num4 as usize].L;
                num2 = parts[num4 as usize].W;
            } else {
                num4 = num4 * -1 - 2;
                num = parts[num4 as usize].W;
                num2 = parts[num4 as usize].L;
            }
            line.Parts_Sq += parts[num4 as usize].sq();
        }

        let crd = Crd {
            X: line.Snips[io].CRD.X,
            Y: line.Snips[io].CRD.Y,
            id_in_order: parts[num3 as usize].iD_in_Order,
        };
        line.Parts_Crds.push(crd);
        line.PartIDs.push(_2parts[0]);
        parts[num3 as usize].Plased += 1;

        if num4 != -1 {
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
            let mut num13 = line.Snips[io].sq() - parts[num3 as usize].sq() - parts[num4 as usize].sq();
            let mut num14 = line.Snips[io].sq() - parts[num3 as usize].sq() - parts[num4 as usize].sq();

            if flag {
                if num6 > num2 {
                    // 4 layout variants
                    let mut lo = l;
                    let mut wo = w - num6 - self.B;
                    let mut lo2 = num;
                    let mut wo2 = num6 - num2 - self.B;
                    let mut lo3 = l - num5 - num - 2 * self.B;
                    let mut wo3 = num6;
                    let mut num15 = self.get_sq_parts_for_snips(parts, lo, wo, lo2, wo2, lo3, wo3);

                    lo = num5;
                    wo = w - num6 - self.B;
                    lo2 = num;
                    wo2 = w - num2 - self.B;
                    lo3 = l - num5 - num - 2 * self.B;
                    wo3 = w;
                    let sq_pfs = self.get_sq_parts_for_snips(parts, lo, wo, lo2, wo2, lo3, wo3);

                    lo = num5;
                    wo = w - num6 - self.B;
                    lo2 = l - num5 - self.B;
                    wo2 = w - num2 - self.B;
                    lo3 = l - num5 - num - 2 * self.B;
                    wo3 = num2;
                    let mut num16 = self.get_sq_parts_for_snips(parts, lo, wo, lo2, wo2, lo3, wo3);

                    lo = l;
                    wo = w - num6 - self.B;
                    lo2 = l - num5 - self.B;
                    wo2 = num6 - num2 - self.B;
                    lo3 = l - num5 - num - 2 * self.B;
                    wo3 = num2;
                    let sq_pfs2 = self.get_sq_parts_for_snips(parts, lo, wo, lo2, wo2, lo3, wo3);

                    let mut num17: i32 = -1;
                    if (num15 * 100.0) as i64 == 0 && (sq_pfs * 100.0) as i64 == 0 {
                        num17 = -1;
                    } else if (num15 * 100.0) as i64 != 0 && (sq_pfs * 100.0) as i64 == 0 {
                        num17 = 1;
                    } else if (num15 * 100.0) as i64 == 0 && (sq_pfs * 100.0) as i64 != 0 {
                        num17 = 2;
                        num15 = sq_pfs;
                    } else if (num15 * 100.0) as i64 != 0 && (sq_pfs * 100.0) as i64 != 0 {
                        if ((num15 - sq_pfs) * 100.0) as i64 >= 0 {
                            num17 = 1;
                        } else {
                            num17 = 2;
                            num15 = sq_pfs;
                        }
                    }

                    let mut num18: i32 = -1;
                    if (num16 * 100.0) as i64 == 0 && (sq_pfs2 * 100.0) as i64 == 0 {
                        num18 = -1;
                    } else if (num16 * 100.0) as i64 != 0 && (sq_pfs2 * 100.0) as i64 == 0 {
                        num18 = 3;
                    } else if (num16 * 100.0) as i64 == 0 && (sq_pfs2 * 100.0) as i64 != 0 {
                        num18 = 4;
                        num16 = sq_pfs2;
                    } else if (num16 * 100.0) as i64 != 0 && (sq_pfs2 * 100.0) as i64 != 0 {
                        if ((num16 - sq_pfs2) * 100.0) as i64 >= 0 {
                            num18 = 3;
                        } else {
                            num18 = 4;
                            num16 = sq_pfs2;
                        }
                    }

                    if num17 == -1 && num18 == -1 {
                        num17 = 1;
                        // num15 = num15; (noop)
                    } else if num17 != -1 && num18 == -1 {
                        // num17 = num17; num15 = num15; (noop)
                    } else if num17 == -1 && num18 != -1 {
                        num17 = num18;
                        num15 = num16;
                    } else if num17 != -1 && num18 != -1 && (((num15 - num16) * 100.0) as i64) < 0 {
                        num17 = num18;
                        num15 = num16;
                    }
                    num13 -= num15;
                    num11 = num17;
                } else {
                    // num6 <= num2
                    let mut lo4 = l;
                    let mut wo4 = w - num6 - self.B;
                    let mut lo5 = l - num5 - num - 2 * self.B;
                    let mut wo5 = num6;
                    let lo6 = 0;
                    let wo6 = 0;
                    let mut num19 = self.get_sq_parts_for_snips(parts, lo4, wo4, lo5, wo5, lo6, wo6);

                    lo4 = num5;
                    wo4 = w - num6 - self.B;
                    lo5 = num;
                    wo5 = w - num2 - self.B;
                    let lo6b = l - num5 - num - 2 * self.B;
                    let wo6b = w;
                    let sq_pfs3 = self.get_sq_parts_for_snips(parts, lo4, wo4, lo5, wo5, lo6b, wo6b);

                    let mut num20: i32 = -1;
                    if (num19 * 100.0) as i64 == 0 && (sq_pfs3 * 100.0) as i64 == 0 {
                        num20 = -1;
                    } else if (num19 * 100.0) as i64 != 0 && (sq_pfs3 * 100.0) as i64 == 0 {
                        num20 = 5;
                    } else if (num19 * 100.0) as i64 == 0 && (sq_pfs3 * 100.0) as i64 != 0 {
                        num20 = 6;
                        num19 = sq_pfs3;
                    } else if (num19 * 100.0) as i64 != 0 && (sq_pfs3 * 100.0) as i64 != 0 {
                        if ((num19 - sq_pfs3) * 100.0) as i64 >= 0 {
                            num20 = 5;
                        } else {
                            num20 = 6;
                            num19 = sq_pfs3;
                        }
                    }
                    if num20 == -1 {
                        num19 = 0.0;
                        num20 = 5;
                    }
                    num13 -= num19;
                    num11 = num20;
                }
            }

            if flag2 {
                if num5 > num {
                    let mut lo7 = l - num5 - self.B;
                    let mut wo7 = w;
                    let mut lo8 = num5 - num - self.B;
                    let mut wo8 = num2;
                    let mut lo9 = num5;
                    let mut wo9 = w - num6 - num2 - 2 * self.B;
                    let mut num21 = self.get_sq_parts_for_snips(parts, lo7, wo7, lo8, wo8, lo9, wo9);

                    lo7 = l - num5 - self.B;
                    wo7 = w;
                    lo8 = num5 - num - self.B;
                    wo8 = w - num6 - self.B;
                    lo9 = num;
                    wo9 = w - num6 - num2 - 2 * self.B;
                    let sq_pfs4 = self.get_sq_parts_for_snips(parts, lo7, wo7, lo8, wo8, lo9, wo9);

                    lo7 = l - num5 - self.B;
                    wo7 = num6;
                    lo8 = l - num - self.B;
                    wo8 = num2;
                    lo9 = l;
                    wo9 = w - num6 - num2 - 2 * self.B;
                    let mut num22 = self.get_sq_parts_for_snips(parts, lo7, wo7, lo8, wo8, lo9, wo9);

                    lo7 = l - num5 - self.B;
                    wo7 = num6;
                    lo8 = l - num - self.B;
                    wo8 = w - num6 - self.B;
                    lo9 = num;
                    wo9 = w - num6 - num2 - 2 * self.B;
                    let sq_pfs5 = self.get_sq_parts_for_snips(parts, lo7, wo7, lo8, wo8, lo9, wo9);

                    let mut num23: i32 = -1;
                    if (num21 * 100.0) as i64 == 0 && (sq_pfs4 * 100.0) as i64 == 0 {
                        num23 = -1;
                    } else if (num21 * 100.0) as i64 != 0 && (sq_pfs4 * 100.0) as i64 == 0 {
                        num23 = 1;
                    } else if (num21 * 100.0) as i64 == 0 && (sq_pfs4 * 100.0) as i64 != 0 {
                        num23 = 2;
                        num21 = sq_pfs4;
                    } else if (num21 * 100.0) as i64 != 0 && (sq_pfs4 * 100.0) as i64 != 0 {
                        if ((num21 - sq_pfs4) * 100.0) as i64 >= 0 {
                            num23 = 1;
                        } else {
                            num23 = 2;
                            num21 = sq_pfs4;
                        }
                    }

                    let mut num24: i32 = -1;
                    if (num22 * 100.0) as i64 == 0 && (sq_pfs5 * 100.0) as i64 == 0 {
                        num24 = -1;
                    } else if (num22 * 100.0) as i64 != 0 && (sq_pfs5 * 100.0) as i64 == 0 {
                        num24 = 3;
                    } else if (num22 * 100.0) as i64 == 0 && (sq_pfs5 * 100.0) as i64 != 0 {
                        num24 = 4;
                        num22 = sq_pfs5;
                    } else if (num22 * 100.0) as i64 != 0 && (sq_pfs5 * 100.0) as i64 != 0 {
                        if ((num22 - sq_pfs5) * 100.0) as i64 >= 0 {
                            num24 = 3;
                        } else {
                            num24 = 4;
                            num22 = sq_pfs5;
                        }
                    }

                    if num23 == -1 && num24 == -1 {
                        num23 = 1;
                    } else if num23 != -1 && num24 == -1 {
                        // noop
                    } else if num23 == -1 && num24 != -1 {
                        num23 = num24;
                        num21 = num22;
                    } else if num23 != -1 && num24 != -1 && (((num21 - num22) * 100.0) as i64) < 0 {
                        num23 = num24;
                        num21 = num22;
                    }
                    num14 -= num21;
                    num12 = num23;
                } else {
                    // num5 <= num
                    let mut lo10 = l - num5 - self.B;
                    let mut wo10 = w;
                    let mut lo11 = num;
                    let mut wo11 = w - num6 - num2 - 2 * self.B;
                    let lo12 = 0;
                    let wo12 = 0;
                    let mut num25 = self.get_sq_parts_for_snips(parts, lo10, wo10, lo11, wo11, lo12, wo12);

                    lo10 = l - num5 - self.B;
                    wo10 = num6;
                    lo11 = l - num5 - self.B;
                    wo11 = num2;
                    let lo12b = l;
                    let wo12b = w - num6 - num2 - 2 * self.B;
                    let sq_pfs6 = self.get_sq_parts_for_snips(parts, lo10, wo10, lo11, wo11, lo12b, wo12b);

                    let mut num26: i32 = -1;
                    if (num25 * 100.0) as i64 == 0 && (sq_pfs6 * 100.0) as i64 == 0 {
                        num26 = -1;
                    } else if (num25 * 100.0) as i64 != 0 && (sq_pfs6 * 100.0) as i64 == 0 {
                        num26 = 5;
                    } else if (num25 * 100.0) as i64 == 0 && (sq_pfs6 * 100.0) as i64 != 0 {
                        num26 = 6;
                        num25 = sq_pfs6;
                    } else if (num25 * 100.0) as i64 != 0 && (sq_pfs6 * 100.0) as i64 != 0 {
                        if ((num25 - sq_pfs6) * 100.0) as i64 >= 0 {
                            num26 = 5;
                        } else {
                            num26 = 6;
                            num25 = sq_pfs6;
                        }
                    }
                    if num26 == -1 {
                        num25 = 0.0;
                        num26 = 5;
                    }
                    num14 -= num25;
                    num12 = num26;
                }
            }

            let num27: i32;
            if flag && flag2 {
                if (num14 - num13) as i64 == 0 {
                    num27 = if l < w { num11 * -1 } else { num12 };
                } else if (num14 - num13) as i64 >= 0 {
                    num27 = num11 * -1;
                } else {
                    num27 = num12;
                }
            } else {
                if flag {
                    num27 = num11 * -1;
                } else if flag2 {
                    num27 = num12;
                } else {
                    num27 = 1; // default
                }
            }

            let mut crd2 = Crd::default();
            if num27 < 0 {
                crd2.X = x + num5 + self.B;
                crd2.Y = y;
            } else {
                crd2.X = x;
                crd2.Y = y + num6 + self.B;
            }
            crd2.id_in_order = parts[num4 as usize].iD_in_Order;
            line.Parts_Crds.push(crd2);
            line.PartIDs.push(_2parts[1]);
            parts[num4 as usize].Plased += 1;

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
                _ => { /* 0 => noop */ }
            }
        } else {
            // num4 == -1: single part placement with snip layout
            let lo13 = num5;
            let wo13 = w - num6 - self.B;
            let lo14 = l - num5 - self.B;
            let wo14 = w;
            let sq_pfs7 = self.get_sq_parts_for_snips(parts, lo13, wo13, lo14, wo14, 0, 0);

            let lo13b = l;
            let wo13b = w - num6 - self.B;
            let lo14b = l - num5 - self.B;
            let wo14b = num6;
            let sq_pfs8 = self.get_sq_parts_for_snips(parts, lo13b, wo13b, lo14b, wo14b, 0, 0);

            let mut num28: i32;
            if (sq_pfs7 * 100.0) as i64 == 0 && (sq_pfs8 * 100.0) as i64 == 0 {
                num28 = -1;
            } else if (sq_pfs7 * 100.0) as i64 != 0 && (sq_pfs8 * 100.0) as i64 == 0 {
                num28 = 1;
            } else if (sq_pfs7 * 100.0) as i64 == 0 && (sq_pfs8 * 100.0) as i64 != 0 {
                num28 = 2;
            } else {
                num28 = if ((sq_pfs7 - sq_pfs8) * 100.0) as i64 >= 0 { 1 } else { 2 };
            }
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
        id: i32,
    ) -> [i32; 2] {
        let mut array = [-1i32, -1];
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
        let sq = parts[num as usize].sq();
        let mut num4 = num2;
        let mut num5 = wo - num3 - self.B;
        let mut num6 = lo - num2 - self.B;
        let mut num7 = wo;
        let mut flag = false;
        let mut flag2 = false;
        if num4 >= self.minL && num5 >= self.minW {
            flag = self.fast_find_first_part(parts, num4, num5);
        }
        if num6 >= self.minL && num7 >= self.minW {
            flag2 = self.fast_find_first_part(parts, num6, num7);
        }
        if flag || flag2 {
            array[0] = id;
        } else {
            num4 = lo - num2 - self.B;
            num5 = num3;
            num6 = lo;
            num7 = wo - num3 - self.B;
            flag = false;
            flag2 = false;
            if num4 >= self.minL && num5 >= self.minW {
                flag = self.fast_find_first_part(parts, num4, num5);
            }
            if num6 >= self.minL && num7 >= self.minW {
                flag2 = self.fast_find_first_part(parts, num6, num7);
            }
            if flag || flag2 {
                array[0] = id;
            } else {
                let array2 = self.find_2_parts(parts, lo, wo);
                let mut num8 = 0.0f64;
                if array2[0] != -1 {
                    num8 = if array2[0] >= -1 {
                        num8 + parts[array2[0] as usize].sq()
                    } else {
                        num8 + parts[(array2[0] * -1 - 2) as usize].sq()
                    };
                }
                if array2[1] != -1 {
                    num8 = if array2[1] >= -1 {
                        num8 + parts[array2[1] as usize].sq()
                    } else {
                        num8 + parts[(array2[1] * -1 - 2) as usize].sq()
                    };
                }
                if (num8 - sq) as i64 > 0 {
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
            CRD: Crd { X: x, Y: y, ..Crd::default() },
            ..CSnip::default()
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
        let mut num = 0.0f64;
        let mut num2: i32 = -1;
        let mut num3: i32 = -1;
        let mut num4: i32 = -1;

        if lo1 >= self.minL && wo1 >= self.minW {
            num2 = self.find_max_sq_part(parts, lo1, wo1);
            if num2 != -1 {
                if num2 < -1 {
                    num2 = num2 * -1 - 2;
                }
                num += parts[num2 as usize].sq();
            }
        }
        if num2 != -1 {
            parts[num2 as usize].Plased += 1;
        }

        if lo2 >= self.minL && wo2 >= self.minW {
            num3 = self.find_max_sq_part(parts, lo2, wo2);
            if num3 != -1 {
                if num3 < -1 {
                    num3 = num3 * -1 - 2;
                }
                num += parts[num3 as usize].sq();
            }
        }
        if num3 != -1 {
            parts[num3 as usize].Plased += 1;
        }

        if lo3 >= self.minL && wo3 >= self.minW {
            num4 = self.find_max_sq_part(parts, lo3, wo3);
            if num4 != -1 {
                if num4 < -1 {
                    num4 = num4 * -1 - 2;
                }
                num += parts[num4 as usize].sq();
            }
        }

        if num2 != -1 {
            parts[num2 as usize].Plased -= 1;
        }
        if num3 != -1 {
            parts[num3 as usize].Plased -= 1;
        }
        num
    }

    fn find_max_sq_part(&self, parts: &[CPart], lo: i32, wo: i32) -> i32 {
        let mut result = -1i32;
        let mut num = 0.0f64;
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

    fn find_max_sq_part_krome(&self, parts: &[CPart], lo: i32, wo: i32, krome: i32) -> i32 {
        let mut result = -1i32;
        let mut num = 0.0f64;
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
        let mut result = -1i32;
        let mut num = 1000000000.0f64;
        for i in 0..snips.len() {
            if (num - snips[i].sq()) as i64 > 0
                && self.fast_find_first_part(parts, snips[i].L, snips[i].W)
            {
                num = snips[i].sq();
                result = i as i32;
            }
        }
        result
    }

    fn find_2_parts(&self, parts: &mut Vec<CPart>, lo: i32, wo: i32) -> [i32; 3] {
        let mut array = [-1i32, -1, 0];
        let mut array2 = [-1i32, -1, 1];
        let mut num: f64;
        let mut num2: f64;
        let mut num3: f64;
        let mut num4: f64;
        let mut num5 = 0.0f64;
        let mut num6 = 0.0f64;
        let mut num7: i32;

        for i in 0..parts.len() {
            num = 0.0;
            num2 = 0.0;
            num3 = 0.0;
            num4 = 0.0;
            if parts[i].Plased >= parts[i].Qty {
                continue;
            }
            if lo >= parts[i].L && wo >= parts[i].W {
                num = parts[i].sq();
                let num8 = lo - self.B - parts[i].L;
                if num8 >= self.minL {
                    num7 = self.find_max_sq_part_krome(parts, num8, wo, i as i32);
                    num2 = if num7 == -1 {
                        0.0
                    } else if num7 >= -1 {
                        parts[num7 as usize].sq()
                    } else {
                        parts[(num7 * -1 - 2) as usize].sq()
                    };
                } else {
                    num7 = -1;
                    num2 = 0.0;
                }
                if ((num5 - (num + num2)) as i64) < 0 {
                    num5 = num + num2;
                    array[0] = i as i32;
                    array[1] = num7;
                }
            }
            if parts[i].Turn && wo >= parts[i].L && lo >= parts[i].W {
                num = parts[i].sq();
                let num9 = lo - self.B - parts[i].W;
                if num9 >= self.minL {
                    num7 = self.find_max_sq_part_krome(parts, num9, wo, i as i32);
                    num2 = if num7 == -1 {
                        0.0
                    } else if num7 >= -1 {
                        parts[num7 as usize].sq()
                    } else {
                        parts[(num7 * -1 - 2) as usize].sq()
                    };
                } else {
                    num7 = -1;
                    num2 = 0.0;
                }
                if ((num5 - (num + num2)) as i64) < 0 {
                    num5 = num + num2;
                    array[0] = i as i32 * -1 - 2;
                    array[1] = num7;
                }
            }
            if lo >= parts[i].L && wo >= parts[i].W {
                num3 = parts[i].sq();
                let num10 = wo - self.B - parts[i].W;
                if num10 >= self.minL {
                    num7 = self.find_max_sq_part_krome(parts, lo, num10, i as i32);
                    num4 = if num7 == -1 {
                        0.0
                    } else if num7 >= -1 {
                        parts[num7 as usize].sq()
                    } else {
                        parts[(num7 * -1 - 2) as usize].sq()
                    };
                } else {
                    num7 = -1;
                    num4 = 0.0;
                }
                if ((num6 - (num3 + num4)) as i64) < 0 {
                    num6 = num3 + num4;
                    array2[0] = i as i32;
                    array2[1] = num7;
                }
            } else if parts[i].Turn && wo >= parts[i].L && lo >= parts[i].W {
                num3 = parts[i].sq();
                let num11 = wo - self.B - parts[i].L;
                if num11 >= self.minL {
                    num7 = self.find_max_sq_part_krome(parts, lo, num11, i as i32);
                    num4 = if num7 == -1 {
                        0.0
                    } else if num7 >= -1 {
                        parts[num7 as usize].sq()
                    } else {
                        parts[(num7 * -1 - 2) as usize].sq()
                    };
                } else {
                    num7 = -1;
                    num4 = 0.0;
                }
                if ((num6 - (num3 + num4)) as i64) < 0 {
                    num6 = num3 + num4;
                    array2[0] = i as i32 * -1 - 2;
                    array2[1] = num7;
                }
            }
        }

        // Sort array by W descending
        if array[0] != -1 && array[1] != -1 {
            let mut num12 = array[0];
            let num13: i32;
            if num12 < -1 {
                num12 = num12 * -1 - 2;
                num13 = parts[num12 as usize].L;
            } else {
                num13 = parts[num12 as usize].W;
            }
            let mut num14 = array[1];
            let num15: i32;
            if num14 < -1 {
                num14 = num14 * -1 - 2;
                num15 = parts[num14 as usize].L;
            } else {
                num15 = parts[num14 as usize].W;
            }
            if num15 > num13 {
                let tmp = array[0];
                array[0] = array[1];
                array[1] = tmp;
            }
        }

        if array2[0] != -1 && array2[1] != -1 {
            let mut num17 = array2[0];
            let num18: i32;
            if num17 < -1 {
                num17 = num17 * -1 - 2;
                num18 = parts[num17 as usize].W;
            } else {
                num18 = parts[num17 as usize].L;
            }
            let mut num19 = array2[1];
            let num20: i32;
            if num19 < -1 {
                num19 = num19 * -1 - 2;
                num20 = parts[num19 as usize].W;
            } else {
                num20 = parts[num19 as usize].L;
            }
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

    fn find_zamena_lines_length_cut(
        &self,
        lines: &[CLine],
        on_sheet: &[bool],
        wo: i32,
        minimal_w: i32,
    ) -> ([i32; 3], bool) {
        let mut array = [-1i32, -1, -1];
        let mut check = false;
        let mut num = 0i32;

        for i in 0..lines.len() {
            let w = lines[i].W;
            if on_sheet[i] || wo < w {
                continue;
            }
            if w > num {
                array[0] = i as i32;
                array[1] = -1;
                array[2] = -1;
                num = w;
                check = true;
            } else if w == num {
                let mut num2 = 0.0f64;
                for j in 0..3 {
                    if array[j] != -1 {
                        num2 += lines[array[j] as usize].Parts_Sq;
                    }
                }
                if (lines[i].Parts_Sq - num2) as i64 >= 0 {
                    array[0] = i as i32;
                    array[1] = -1;
                    array[2] = -1;
                    num = w;
                    check = true;
                }
            }

            if wo - w - self.B - minimal_w < 0 {
                continue;
            }

            for k in (i + 1)..lines.len() {
                let w2 = lines[k].W;
                if on_sheet[k] || wo < w2 {
                    continue;
                }
                if wo - w - self.B - w2 >= 0 {
                    if w + w2 - num > 0 {
                        array[0] = i as i32;
                        array[1] = k as i32;
                        array[2] = -1;
                        num = w + w2;
                        check = true;
                    } else if w + w2 - num == 0 {
                        let mut num3 = 0.0f64;
                        for ll in 0..3 {
                            if array[ll] != -1 {
                                num3 += lines[array[ll] as usize].Parts_Sq;
                            }
                        }
                        if (lines[i].Parts_Sq + lines[k].Parts_Sq - num3) as i64 >= 0 {
                            array[0] = i as i32;
                            array[1] = k as i32;
                            array[2] = -1;
                            num = w + w2;
                            check = true;
                        }
                    }
                }

                if wo - w - self.B - w2 - self.B - minimal_w < 0 {
                    continue;
                }

                for m in (k + 1)..lines.len() {
                    let w3 = lines[m].W;
                    if on_sheet[m] || wo < w3 || wo - w - self.B - w2 - self.B - w3 < 0 {
                        continue;
                    }
                    if w + w2 + w3 - num > 0 {
                        array[0] = i as i32;
                        array[1] = k as i32;
                        array[2] = m as i32;
                        num = w + w2 + w3;
                        check = true;
                    } else if w + w2 + w3 - num == 0 {
                        let mut num4 = 0.0f64;
                        for n in 0..3 {
                            if array[n] != -1 {
                                num4 += lines[array[n] as usize].Parts_Sq;
                            }
                        }
                        if (lines[i].Parts_Sq + lines[k].Parts_Sq + lines[m].Parts_Sq - num4)
                            as i64
                            >= 0
                        {
                            array[0] = i as i32;
                            array[1] = k as i32;
                            array[2] = m as i32;
                            num = w + w2 + w3;
                            check = true;
                        }
                    }
                }
            }
        }
        (array, check)
    }

    fn find_zamena_parts_length_cut(
        &self,
        fix: &[i32],
        parts: &[CPart],
        wo: i32,
        max_l_in: i32,
        minimal_l: i32,
    ) -> ([i32; 3], bool) {
        let mut array = [-1i32, -1, -1];
        let mut check = false;
        let mut max_l = max_l_in;

        for i in 0..fix.len() {
            let num = if fix[i] <= -1 {
                parts[(fix[i] * -1 - 2) as usize].W
            } else {
                parts[fix[i] as usize].L
            };
            if wo < num {
                continue;
            }
            if num > max_l {
                array[0] = i as i32;
                array[1] = -1;
                array[2] = -1;
                max_l = num;
                check = true;
            }
            if wo - num - self.B - minimal_l < 0 {
                continue;
            }
            for j in (i + 1)..fix.len() {
                let num2 = if fix[j] <= -1 {
                    parts[(fix[j] * -1 - 2) as usize].W
                } else {
                    parts[fix[j] as usize].L
                };
                if wo - num - self.B - num2 >= 0 && num + num2 - max_l > 0 {
                    array[0] = i as i32;
                    array[1] = j as i32;
                    array[2] = -1;
                    max_l = num + num2;
                    check = true;
                }
            }
        }
        (array, check)
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

    /// Returns (fix_width_list, Min_L, Total_Length)
    fn get_parts_with_fix_width(
        &self,
        parts: &[CPart],
        w: i32,
        _min_in: i32,
        turn_on: bool,
    ) -> (Vec<i32>, i32, i32) {
        let mut list: Vec<i32> = Vec::new();
        let mut min_l = _min_in;
        let mut total_length = 0i32;

        for i in 0..parts.len() {
            let cp = &parts[i];
            if cp.Qty <= cp.Plased {
                continue;
            }
            if !turn_on {
                if !cp.Turn {
                    if cp.W == w {
                        for _j in 0..(cp.Qty - cp.Plased) {
                            list.push(i as i32);
                            total_length += cp.L;
                        }
                        if min_l > cp.L {
                            min_l = cp.L;
                        }
                    }
                } else {
                    // cp.Turn == true, !turn_on
                    let num: f32;
                    let mut flag = false;
                    if cp.W <= cp.L {
                        num = cp.W as f32;
                        flag = false;
                    } else {
                        num = cp.L as f32;
                        flag = true;
                    }
                    if num == w as f32 {
                        for _k in 0..(cp.Qty - cp.Plased) {
                            if flag {
                                list.push(i as i32 * -1 - 2);
                                total_length += cp.W;
                            } else {
                                list.push(i as i32);
                                total_length += cp.L;
                            }
                        }
                        if flag {
                            if min_l > cp.W {
                                min_l = cp.W;
                            } else if min_l > cp.L {
                                min_l = cp.L;
                            }
                        }
                    }
                }
            } else {
                if cp.W == w {
                    for _l in 0..(cp.Qty - cp.Plased) {
                        list.push(i as i32);
                        total_length += cp.L;
                    }
                    if min_l > cp.L {
                        min_l = cp.L;
                    }
                } else if cp.Turn && cp.L == w {
                    for _m in 0..(cp.Qty - cp.Plased) {
                        list.push(i as i32 * -1 - 2);
                        total_length += cp.W;
                    }
                    if min_l > cp.W {
                        min_l = cp.W;
                    }
                }
            }
            if list.len() as i32 > self.THE_SAME_PARTS_LIMIT {
                break;
            }
        }
        (list, min_l, total_length)
    }

    fn get_start_parts_for_line_length_cut(
        &self,
        parts: &[CPart],
        mut fix_width: Vec<i32>,
        line_length: i32,
        minimal_l: i32,
    ) -> Vec<i32> {
        let mut list: Vec<i32> = Vec::new();
        let mut num = line_length;

        let mut i: i32 = 0;
        while (i as usize) < fix_width.len() {
            let (_id, ld, _wd) = Self::get_id_ld_wd(parts, fix_width[i as usize]);
            if num >= ld {
                num = num - ld - self.B;
                list.push(fix_width[i as usize]);
                fix_width.remove(i as usize);
                i -= 1;
                if num < minimal_l {
                    break;
                }
            }
            i += 1;
        }

        let mut flag = false;
        let mut num2 = 0;
        let mut num3: i32;
        while !flag && num2 < self.PARTS_SORT_LIMIT {
            num2 += 1;
            num3 = 0;
            let mut _check = false;
            let mut num4: i32 = -1;
            let mut num5: i32 = -1;
            let mut num6 = 0i32;
            let mut num7 = 0i32;
            let mut array: Option<[i32; 3]> = None;

            if list.len() > 1 {
                for j in 0..list.len() - 1 {
                    for k in (j + 1)..list.len() {
                        fix_width.push(list[j]);
                        fix_width.push(list[k]);

                        let num8 = if list[j] <= -1 {
                            parts[(list[j] * -1 - 2) as usize].W
                        } else {
                            parts[list[j] as usize].L
                        };
                        let num9 = if list[k] <= -1 {
                            parts[(list[k] * -1 - 2) as usize].W
                        } else {
                            parts[list[k] as usize].L
                        };

                        let wo = num + self.B + num8 + self.B + num9;
                        let (array2, check2) = self.find_zamena_parts_length_cut(
                            &fix_width,
                            parts,
                            wo,
                            num8 + num9,
                            minimal_l,
                        );

                        if check2 {
                            let mut num10 = 0i32;
                            for ll in 0..array2.len() {
                                if array2[ll] != -1 {
                                    let (_id, ld2, _wd) =
                                        Self::get_id_ld_wd(parts, fix_width[array2[ll] as usize]);
                                    num10 = num10 + self.B + ld2;
                                }
                            }
                            if num10 > num3 {
                                num4 = j as i32;
                                num5 = k as i32;
                                num6 = num8;
                                num7 = num9;
                                array = Some(array2);
                                num3 = num10;
                            }
                        }

                        fix_width.pop();
                        fix_width.pop();
                    }
                }
            }

            if num4 != -1 && num5 != -1 {
                fix_width.push(list[num4 as usize]);
                fix_width.push(list[num5 as usize]);
                list.remove(num4 as usize);
                list.remove(num5 as usize - 1);
                num = num + self.B + num6 + self.B + num7;

                if let Some(arr) = array {
                    for m in 0..arr.len() {
                        if arr[m] != -1 {
                            list.push(fix_width[arr[m] as usize]);
                            let (_id, ld3, _wd) =
                                Self::get_id_ld_wd(parts, fix_width[arr[m] as usize]);
                            num = num - self.B - ld3;
                        }
                    }
                    // Remove used items from fix_width (descending order)
                    let mut num11 = 0i32;
                    for n in 0..arr.len() {
                        if arr[n] != -1 {
                            fix_width.remove((arr[n] - num11) as usize);
                            num11 += 1;
                        }
                    }
                }
            } else {
                flag = true;
            }
        }

        // Sort list by LD descending (bubble sort)
        for num12 in 0..list.len().saturating_sub(1) {
            for num13 in (num12 + 1)..list.len() {
                let (_id1, ld4, _wd1) = Self::get_id_ld_wd(parts, list[num12]);
                let (_id2, ld5, _wd2) = Self::get_id_ld_wd(parts, list[num13]);
                if ld5 > ld4 {
                    list.swap(num12, num13);
                }
            }
        }
        list
    }
}
