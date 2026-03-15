#![allow(non_snake_case)]
#![allow(unused_assignments)]
#![allow(clippy::needless_range_loop)]
#![allow(clippy::comparison_chain)]

use super::algorithm_types::*;
use std::time::Instant;

pub struct LengthAlg {
    pub THE_SAME_PARTS_LIMIT: i32,
    pub LINES_LIMIT: i32,
    pub LINES_SORT_ITERS_LIMIT: i32,
    pub PARTS_SORT_LIMIT: i32,
    pub TIME_GET_LINES_LIMIT: f64,

    P: i32,
    B: i32,
    L_L: i32,
    L_W: i32,
    minL: i32,
    minW: i32,
}

impl LengthAlg {
    pub fn new() -> Self {
        Self {
            THE_SAME_PARTS_LIMIT: 25,
            LINES_LIMIT: 200,
            LINES_SORT_ITERS_LIMIT: 4,
            PARTS_SORT_LIMIT: 2,
            TIME_GET_LINES_LIMIT: 2.0,
            P: 0,
            B: 0,
            L_L: 0,
            L_W: 0,
            minL: 0,
            minW: 0,
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

    pub fn get_csheet_length_cut(
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

        let (clines, mut on_sheet_flags) = self.get_clines_length_cut(parts, num2, num3, opti_on);

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

        let mut flag = false;
        let mut num7 = 0;
        while !flag && num7 < self.LINES_SORT_ITERS_LIMIT {
            num7 += 1;
            let mut num8: i32 = -1;
            let mut num9: i32 = -1;
            let mut array: Option<[i32; 3]> = None;
            let mut num10: f64 = 0.0;

            let sheet_lines_count = csheet.Lines.len() as i32;
            if sheet_lines_count >= 2 {
                for k in 0..(sheet_lines_count - 1) as usize {
                    for l in (k + 1)..sheet_lines_count as usize {
                        on_sheet_flags[lines_index[k]] = false;
                        on_sheet_flags[lines_index[l]] = false;

                        let w_o =
                            num6 + self.B + csheet.Lines[k].W + self.B + csheet.Lines[l].W;
                        let mut _check = false;
                        let array2 = self.find_zamena_lines_length_cut(
                            &clines,
                            &on_sheet_flags,
                            w_o,
                            num4,
                            &mut _check,
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
                                && ((num14 - num12) as i32) >= 0
                                && (((num14 - num10) as i32) > 0
                                    || (((num14 - num10) as i32) == 0
                                        && num8 >= 0
                                        && num9 >= 0
                                        && num13
                                            - csheet.Lines[num8 as usize].W
                                            - csheet.Lines[num9 as usize].W
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
                csheet.Lines.remove((num9 - 1) as usize);
                lines_index.remove(num8 as usize);
                lines_index.remove((num9 - 1) as usize);

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

        csheet.Remain = CSnip {
            L: num2,
            W: num6,
            ..CSnip::default()
        };

        // Sort lines by W descending
        {
            let len = csheet.Lines.len();
            for i in 0..len {
                for j in (i + 1)..len {
                    if csheet.Lines[j].W > csheet.Lines[i].W {
                        lines_index.swap(i, j);
                        csheet.Lines.swap(i, j);
                    }
                }
            }
        }

        // SET_OFF parts not on sheet
        for idx in 0..clines.len() {
            if !on_sheet_flags[idx] {
                Self::set_off_parts_in_line(parts, &clines[idx]);
            }
        }

        csheet.Parts_Sq = 0.0;
        let max_way = true;
        let opti_on_flag = true;
        for i in (0..csheet.Lines.len()).rev() {
            self.continue_line_length_cut(&mut csheet.Lines[i], parts, max_way, opti_on_flag);
            csheet.Parts_Sq += csheet.Lines[i].Parts_Sq;
        }

        flag = false;
        while !flag {
            if self.fast_find_first_part(parts, csheet.Remain.L, csheet.Remain.W) {
                let mut cline = CLine::default();
                let mut csnip = CSnip::default();
                cline.Snips = Vec::new();
                cline.PartIDs = Vec::new();
                cline.Parts_Crds = Vec::new();

                let num19 = self.find_width_part(parts, csheet.Remain.L, csheet.Remain.W, true);
                let (_, _, wd) = Self::get_id_ld_wd(parts, num19);
                cline.W = wd;
                cline.L = csheet.Remain.L;
                csnip.CRD = Crd::default();
                csnip.CRD.X = 0;
                csnip.CRD.Y = 0;
                csnip.L = cline.L;
                csnip.W = cline.W;
                cline.Snips.push(csnip);
                let io = self.find_small_snip(&cline.Snips, parts);
                let rez = false;
                self.place_part_to_line(&mut cline, parts, num19, io, rez);
                self.continue_line_length_cut(&mut cline, parts, max_way, true);
                csheet.Remain.W = csheet.Remain.W - self.B - cline.W;
                csheet.Lines.push(cline);
            } else {
                flag = true;
            }
        }

        csheet
    }

    fn continue_line_length_cut(
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
            let num = if !max_way {
                self.find_width_part(parts, l, w, true)
            } else {
                self.find_max_sq_part(parts, l, w)
            };
            if num != -1 {
                if opti_on {
                    let array = self.check_part_for_last_in_line(parts, l, w, false, num);
                    if num != array[0] && array[0] != -1 {
                        self.place_2_parts_to_line(line, parts, &array, num2 as usize);
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

    fn get_clines_length_cut(
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
        let mut num4 = lw;

        while !flag && num2 < self.LINES_LIMIT {
            num2 += 1;
            let mut cline: Option<CLine>;
            let mut cline2: Option<CLine>;
            let mut cline3: Option<CLine> = None;

            let num = self.find_width_part(parts, num3, num4, false);
            if num != -1 {
                // Variant 1: THE_SAME_MAX=true
                let (made1, pre_cut1) =
                    self.make_line_length_cut(parts, num, num3, num4, true, false, opti_on);
                cline = made1;
                if let Some(mut pc1) = pre_cut1.clone() {
                    Self::set_on_parts_in_line(parts, &pc1);
                    self.continue_line_length_cut(&mut pc1, parts, true, opti_on);
                    Self::set_off_parts_in_line(parts, &pc1);
                    if let Some(ref c1) = cline {
                        if ((pc1.Parts_Sq - c1.Parts_Sq) as i32) > 0 {
                            cline = Some(pc1);
                        }
                    }
                }

                // Variant 2: THE_SAME_MAX=false
                let (made2, pre_cut2) =
                    self.make_line_length_cut(parts, num, num3, num4, false, false, opti_on);
                cline2 = made2;
                if let Some(mut pc2) = pre_cut2.clone() {
                    Self::set_on_parts_in_line(parts, &pc2);
                    self.continue_line_length_cut(&mut pc2, parts, true, opti_on);
                    Self::set_off_parts_in_line(parts, &pc2);
                    if let Some(ref c2) = cline2 {
                        if ((c2.Parts_Sq - pc2.Parts_Sq) as i32) < 0 {
                            cline2 = Some(pc2);
                        }
                    }
                }

                // Compare cline and cline2
                match (&cline, &cline2) {
                    (Some(c1), Some(c2)) => {
                        if (((c1.filling() - c2.filling()) * 100.0) as i32) < 0 {
                            cline = cline2;
                        } else if (((c1.filling() - c2.filling()) * 100.0) as i32) == 0
                            && c1.L < c2.L
                        {
                            cline = cline2;
                        }
                    }
                    (None, Some(_)) => {
                        cline = cline2;
                    }
                    _ => {}
                }

                // Try rotated variant
                let index = if num < -1 { (num * -1 - 2) as usize } else { num as usize };
                if parts[index].Turn {
                    let can_rotate = if num < -1 {
                        num3 >= parts[index].L && num4 >= parts[index].W
                    } else {
                        num3 >= parts[index].W && num4 >= parts[index].L
                    };

                    if can_rotate {
                        let rotated_id = num * -1 - 2;
                        let (made3, pre_cut3) = self.make_line_length_cut(
                            parts, rotated_id, num3, num4, true, false, opti_on,
                        );
                        cline3 = made3;
                        let mut pc3_for_compare: Option<CLine> = None;
                        if let Some(mut pc3) = pre_cut3 {
                            Self::set_on_parts_in_line(parts, &pc3);
                            self.continue_line_length_cut(&mut pc3, parts, true, opti_on);
                            Self::set_off_parts_in_line(parts, &pc3);
                            pc3_for_compare = Some(pc3);
                        }

                        let (made4, pre_cut4) = self.make_line_length_cut(
                            parts, rotated_id, num3, num4, false, false, opti_on,
                        );
                        let cline4 = made4;
                        let mut pc4_for_compare: Option<CLine> = None;
                        if let Some(mut pc4) = pre_cut4 {
                            Self::set_on_parts_in_line(parts, &pc4);
                            self.continue_line_length_cut(&mut pc4, parts, true, opti_on);
                            Self::set_off_parts_in_line(parts, &pc4);
                            pc4_for_compare = Some(pc4);
                        }

                        if cline3.is_some() && cline4.is_some() && pc4_for_compare.is_some() {
                            if let Some(ref pc3) = pc3_for_compare {
                                let c3 = cline3.as_ref().unwrap();
                                if ((pc3.Parts_Sq - c3.Parts_Sq) as i32) > 0 {
                                    cline3 = pc3_for_compare.clone();
                                }
                            }

                            let c4 = cline4.as_ref().unwrap();
                            let c3 = cline3.as_ref().unwrap();
                            if ((c4.Parts_Sq - c3.Parts_Sq) as i32) > 0 {
                                cline3 = cline4;
                            }

                            let pc4 = pc4_for_compare.as_ref().unwrap();
                            let c3 = cline3.as_ref().unwrap();
                            if ((pc4.Parts_Sq - c3.Parts_Sq) as i32) > 0 {
                                cline3 = pc4_for_compare;
                            }
                        }
                    }
                }

                if let (Some(ref c), Some(ref c3)) = (&cline, &cline3) {
                    if (((c.filling() - c3.filling()) * 100.0) as i32) < 0 {
                        cline = cline3;
                    }
                }
            } else {
                cline = None;
            }

            if let Some(cl) = cline {
                Self::set_on_parts_in_line(parts, &cl);
                on_sheet.push(false);
                list.push(cl);
                let last_w = list.last().unwrap().W;
                num4 = num4 - self.B - last_w;
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
            if (((elapsed - self.TIME_GET_LINES_LIMIT) * 10.0) as i32) > 0 {
                flag = true;
            }
        }

        (list, on_sheet)
    }

    fn make_line_length_cut(
        &mut self,
        parts: &mut Vec<CPart>,
        start_part: i32,
        line_length: i32,
        _line_width: i32,
        the_same_max: bool,
        max_way: bool,
        opti_on: bool,
    ) -> (Option<CLine>, Option<CLine>) {
        let mut cline = CLine::default();
        cline.Snips = Vec::new();
        cline.PartIDs = Vec::new();
        cline.Parts_Crds = Vec::new();

        let (_, _, wd) = Self::get_id_ld_wd(parts, start_part);
        let rez = false;
        cline.W = wd;
        cline.L = line_length;

        let mut csnip = CSnip::default();
        csnip.CRD = Crd::default();
        csnip.CRD.X = 0;
        csnip.CRD.Y = 0;
        csnip.L = cline.L;
        csnip.W = cline.W;
        cline.Snips.push(csnip);

        if the_same_max {
            let (fix_width, min_l, _total_length) =
                self.get_parts_with_fix_width(parts, wd, cline.W);
            let start_parts =
                self.get_start_parts_for_line_length_cut(parts, fix_width, cline.L, min_l);
            let io = self.find_small_snip(&cline.Snips, parts);
            for i in 0..start_parts.len() {
                self.place_part_to_line(&mut cline, parts, start_parts[i], io, rez);
            }
        } else {
            let io = 0;
            self.place_part_to_line(&mut cline, parts, start_part, io, rez);
            let mut flag = false;
            let io2 = self.find_small_snip(&cline.Snips, parts);
            if io2 != -1 {
                while !flag {
                    let l = cline.Snips[io2 as usize].L;
                    let w = cline.Snips[io2 as usize].W;
                    let found = self.find_the_same_width_part(parts, l, w);
                    if found != -1 {
                        self.place_part_to_line(&mut cline, parts, found, io2, rez);
                    } else {
                        flag = true;
                    }
                }
            }
        }

        let pre_cut = Some(self.copy_line_without_marks(&cline));
        self.continue_line_length_cut(&mut cline, parts, max_way, opti_on);
        Self::set_off_parts_in_line(parts, &cline);

        (Some(cline), pre_cut)
    }

    fn copy_line_without_marks(&self, line: &CLine) -> CLine {
        let mut cline = CLine::default();
        cline.Snips = Vec::new();
        cline.PartIDs = Vec::new();
        cline.Parts_Crds = Vec::new();
        cline.L = line.L;
        cline.W = line.W;
        cline.Parts_Sq = line.Parts_Sq;

        for i in 0..line.PartIDs.len() {
            cline.PartIDs.push(line.PartIDs[i]);
            cline.Parts_Crds.push(line.Parts_Crds[i].clone());
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
            cline.Snips.push(s);
        }

        cline
    }

    fn fast_find_first_part(&self, parts: &[CPart], lo: i32, wo: i32) -> bool {
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

    fn find_width_part(&self, parts: &[CPart], lo: i32, wo: i32, max_w: bool) -> i32 {
        let mut result: i32 = -1;
        let mut num: i32 = 0;
        let mut num2: f64 = 0.0;

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
                    } else if parts[i].W == num && ((parts[i].sq() - num2) as i64) > 0 {
                        num2 = parts[i].sq();
                        num = parts[i].W;
                        result = i as i32;
                    }
                }
            } else {
                let mut num3: i32 = 0;
                if lo >= parts[i].L && wo >= parts[i].W && lo >= parts[i].W && wo >= parts[i].L {
                    num3 = if max_w {
                        if parts[i].L < parts[i].W { parts[i].W } else { parts[i].L }
                    } else {
                        if parts[i].L < parts[i].W { parts[i].L } else { parts[i].W }
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
                    result = if parts[i].W != num3 { -1 * i as i32 - 2 } else { i as i32 };
                } else if num3 == num && ((parts[i].sq() - num2) as i64) > 0 {
                    num2 = parts[i].sq();
                    num = num3;
                    result = if parts[i].W != num3 { -1 * i as i32 - 2 } else { i as i32 };
                }
            }
        }

        result
    }

    fn find_the_same_width_part(&self, parts: &[CPart], lo: i32, wo: i32) -> i32 {
        let mut result: i32 = -1;
        let mut num: f64 = 0.0;

        for i in 0..parts.len() {
            if parts[i].Plased >= parts[i].Qty {
                continue;
            }
            if !parts[i].Turn {
                if lo >= parts[i].L && wo == parts[i].W && ((parts[i].sq() - num) as i64) > 0 {
                    num = parts[i].sq();
                    result = i as i32;
                }
            } else {
                let mut num2: i32 = 0;
                if lo >= parts[i].L && wo == parts[i].W {
                    num2 = parts[i].W;
                } else if lo >= parts[i].W && wo == parts[i].L {
                    num2 = parts[i].L;
                }
                if num2 > 0 && ((parts[i].sq() - num) as i64) > 0 {
                    num = parts[i].sq();
                    result = if parts[i].W != num2 { -1 * i as i32 - 2 } else { i as i32 };
                }
            }
        }

        result
    }

    fn place_part_to_line(
        &mut self,
        line: &mut CLine,
        parts: &mut Vec<CPart>,
        part_id: i32,
        io: i32,
        _rez: bool,
    ) {
        let io = io as usize;
        let (id, ld, wd) = Self::get_id_ld_wd(parts, part_id);
        let id = id as usize;
        line.Parts_Sq += parts[id].sq();
        line.PartIDs.push(part_id);

        let crd = Crd {
            X: line.Snips[io].CRD.X,
            Y: line.Snips[io].CRD.Y,
            id_in_order: parts[id].iD_in_Order,
        };
        line.Parts_Crds.push(crd);
        parts[id].Plased += 1;

        let l = line.Snips[io].L;
        let w = line.Snips[io].W;

        if l > ld && w > wd {
            if _rez {
                let sn = l - ld - self.B;
                let sw = w;
                let sx = line.Snips[io].CRD.X + ld + self.B;
                let sy = line.Snips[io].CRD.Y;
                if self.fast_find_first_part(parts, sn, sw) {
                    let item = Self::create_csnip(sx, sy, sn, sw);
                    line.Snips.push(item);
                    let rx = line.Snips[io].CRD.X;
                    let ry = line.Snips[io].CRD.Y + wd + self.B;
                    Self::resize_csnip(&mut line.Snips[io], rx, ry, ld, w - wd - self.B);
                } else {
                    let item2 = Self::create_csnip(sx, sy, sn, wd);
                    line.Snips.push(item2);
                    let rx = line.Snips[io].CRD.X;
                    let ry = line.Snips[io].CRD.Y + wd + self.B;
                    Self::resize_csnip(&mut line.Snips[io], rx, ry, l, w - wd - self.B);
                }
            } else {
                let sn = l;
                let sw = w - wd - self.B;
                let sx = line.Snips[io].CRD.X;
                let sy = line.Snips[io].CRD.Y + wd + self.B;
                if self.fast_find_first_part(parts, sn, sw) {
                    let item3 = Self::create_csnip(sx, sy, sn, sw);
                    line.Snips.push(item3);
                    let rx = line.Snips[io].CRD.X + ld + self.B;
                    let ry = line.Snips[io].CRD.Y;
                    Self::resize_csnip(&mut line.Snips[io], rx, ry, l - ld - self.B, wd);
                } else {
                    let item4 = Self::create_csnip(sx, sy, ld, sw);
                    line.Snips.push(item4);
                    let rx = line.Snips[io].CRD.X + ld + self.B;
                    let ry = line.Snips[io].CRD.Y;
                    Self::resize_csnip(&mut line.Snips[io], rx, ry, l - ld - self.B, w);
                }
            }
        } else if ld == l && wd < w {
            let rx = line.Snips[io].CRD.X;
            let ry = line.Snips[io].CRD.Y + wd + self.B;
            Self::resize_csnip(&mut line.Snips[io], rx, ry, l, w - wd - self.B);
        } else if ld < l && wd == w {
            let rx = line.Snips[io].CRD.X + ld + self.B;
            let ry = line.Snips[io].CRD.Y;
            Self::resize_csnip(&mut line.Snips[io], rx, ry, l - ld - self.B, w);
        } else if ld == l && wd == w {
            let rx = line.Snips[io].CRD.X;
            let ry = line.Snips[io].CRD.Y;
            Self::resize_csnip(&mut line.Snips[io], rx, ry, 0, 0);
        }
    }

    fn place_2_parts_to_line(
        &mut self,
        line: &mut CLine,
        parts: &mut Vec<CPart>,
        _2parts: &[i32; 2],
        io: usize,
    ) {
        let l = line.Snips[io].L;
        let w = line.Snips[io].W;
        let x = line.Snips[io].CRD.X;
        let y = line.Snips[io].CRD.Y;
        let _ = (l - self.B) / 2;
        let _ = (w - self.B) / 2;

        let mut num: i32 = 0;
        let mut num2: i32 = 0;

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
            let mut num13: f64 =
                line.Snips[io].sq() - parts[num3 as usize].sq() - parts[num4 as usize].sq();
            let mut num14: f64 =
                line.Snips[io].sq() - parts[num3 as usize].sq() - parts[num4 as usize].sq();

            if flag {
                if num6 > num2 {
                    let (mut lo1, mut wo1, mut lo2, mut wo2, mut lo3, mut wo3);

                    lo1 = l; wo1 = w - num6 - self.B;
                    lo2 = num; wo2 = num6 - num2 - self.B;
                    lo3 = l - num5 - num - 2 * self.B; wo3 = num6;
                    let mut num15 = self.get_sq_parts_for_snips(parts, lo1, wo1, lo2, wo2, lo3, wo3);

                    lo1 = num5; wo1 = w - num6 - self.B;
                    lo2 = num; wo2 = w - num2 - self.B;
                    lo3 = l - num5 - num - 2 * self.B; wo3 = w;
                    let sq_snips = self.get_sq_parts_for_snips(parts, lo1, wo1, lo2, wo2, lo3, wo3);

                    lo1 = num5; wo1 = w - num6 - self.B;
                    lo2 = l - num5 - self.B; wo2 = w - num2 - self.B;
                    lo3 = l - num5 - num - 2 * self.B; wo3 = num2;
                    let mut num16 = self.get_sq_parts_for_snips(parts, lo1, wo1, lo2, wo2, lo3, wo3);

                    lo1 = l; wo1 = w - num6 - self.B;
                    lo2 = l - num5 - self.B; wo2 = num6 - num2 - self.B;
                    lo3 = l - num5 - num - 2 * self.B; wo3 = num2;
                    let sq_snips2 = self.get_sq_parts_for_snips(parts, lo1, wo1, lo2, wo2, lo3, wo3);

                    let mut num17 = self.compare_two_sq(num15, sq_snips, 1, 2);
                    if num17 == 2 { num15 = sq_snips; }

                    let num18 = self.compare_two_sq(num16, sq_snips2, 3, 4);
                    if num18 == 4 { num16 = sq_snips2; }

                    if num17 == -1 && num18 == -1 {
                        num17 = 1;
                    } else if num17 == -1 && num18 != -1 {
                        num17 = num18;
                        num15 = num16;
                    } else if num17 != -1 && num18 != -1 && (((num15 - num16) * 100.0) as i64) < 0
                    {
                        num17 = num18;
                        num15 = num16;
                    }

                    num13 -= num15;
                    num11 = num17;
                } else {
                    let lo1 = l; let wo1 = w - num6 - self.B;
                    let lo2 = l - num5 - num - 2 * self.B; let wo2 = num6;
                    let mut num19 = self.get_sq_parts_for_snips(parts, lo1, wo1, lo2, wo2, 0, 0);

                    let lo1b = num5; let wo1b = w - num6 - self.B;
                    let lo2b = num; let wo2b = w - num2 - self.B;
                    let lo3b = l - num5 - num - 2 * self.B; let wo3b = w;
                    let sq_snips3 = self.get_sq_parts_for_snips(parts, lo1b, wo1b, lo2b, wo2b, lo3b, wo3b);

                    let mut num20 = self.compare_two_sq(num19, sq_snips3, 5, 6);
                    if num20 == 6 { num19 = sq_snips3; }
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
                    let (mut lo7, mut wo7, mut lo8, mut wo8, mut lo9, mut wo9);

                    lo7 = l - num5 - self.B; wo7 = w;
                    lo8 = num5 - num - self.B; wo8 = num2;
                    lo9 = num5; wo9 = w - num6 - num2 - 2 * self.B;
                    let mut num21 = self.get_sq_parts_for_snips(parts, lo7, wo7, lo8, wo8, lo9, wo9);

                    lo7 = l - num5 - self.B; wo7 = w;
                    lo8 = num5 - num - self.B; wo8 = w - num6 - self.B;
                    lo9 = num; wo9 = w - num6 - num2 - 2 * self.B;
                    let sq_snips4 = self.get_sq_parts_for_snips(parts, lo7, wo7, lo8, wo8, lo9, wo9);

                    lo7 = l - num5 - self.B; wo7 = num6;
                    lo8 = l - num - self.B; wo8 = num2;
                    lo9 = l; wo9 = w - num6 - num2 - 2 * self.B;
                    let mut num22 = self.get_sq_parts_for_snips(parts, lo7, wo7, lo8, wo8, lo9, wo9);

                    lo7 = l - num5 - self.B; wo7 = num6;
                    lo8 = l - num - self.B; wo8 = w - num6 - self.B;
                    lo9 = num; wo9 = w - num6 - num2 - 2 * self.B;
                    let sq_snips5 = self.get_sq_parts_for_snips(parts, lo7, wo7, lo8, wo8, lo9, wo9);

                    let mut num23 = self.compare_two_sq(num21, sq_snips4, 1, 2);
                    if num23 == 2 { num21 = sq_snips4; }

                    let num24 = self.compare_two_sq(num22, sq_snips5, 3, 4);
                    if num24 == 4 { num22 = sq_snips5; }

                    if num23 == -1 && num24 == -1 {
                        num23 = 1;
                    } else if num23 == -1 && num24 != -1 {
                        num23 = num24;
                        num21 = num22;
                    } else if num23 != -1 && num24 != -1 && (((num21 - num22) * 100.0) as i64) < 0
                    {
                        num23 = num24;
                        num21 = num22;
                    }

                    num14 -= num21;
                    num12 = num23;
                } else {
                    let lo10 = l - num5 - self.B; let wo10 = w;
                    let lo11 = num; let wo11 = w - num6 - num2 - 2 * self.B;
                    let mut num25 = self.get_sq_parts_for_snips(parts, lo10, wo10, lo11, wo11, 0, 0);

                    let lo10b = l - num5 - self.B; let wo10b = num6;
                    let lo11b = l - num5 - self.B; let wo11b = num2;
                    let lo12b = l; let wo12b = w - num6 - num2 - 2 * self.B;
                    let sq_snips6 = self.get_sq_parts_for_snips(parts, lo10b, wo10b, lo11b, wo11b, lo12b, wo12b);

                    let mut num26 = self.compare_two_sq(num25, sq_snips6, 5, 6);
                    if num26 == 6 { num25 = sq_snips6; }
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
                if ((num14 - num13) as i64) == 0 {
                    num27 = if l < w { num11 * -1 } else { num12 };
                } else if ((num14 - num13) as i64) >= 0 {
                    num27 = num11 * -1;
                } else {
                    num27 = num12;
                }
            } else if flag {
                num27 = num11 * -1;
            } else if flag2 {
                num27 = num12;
            } else {
                num27 = 1;
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
                _ => {}
            }
        } else {
            // num4 == -1, only one part placed
            let lo13 = num5;
            let wo13 = w - num6 - self.B;
            let lo14 = l - num5 - self.B;
            let wo14 = w;
            let sq7 = self.get_sq_parts_for_snips(parts, lo13, wo13, lo14, wo14, 0, 0);

            let lo13b = l;
            let wo13b = w - num6 - self.B;
            let lo14b = l - num5 - self.B;
            let wo14b = num6;
            let sq8 = self.get_sq_parts_for_snips(parts, lo13b, wo13b, lo14b, wo14b, 0, 0);

            let mut num28: i32;
            if ((sq7 * 100.0) as i64) == 0 && ((sq8 * 100.0) as i64) == 0 {
                num28 = -1;
            } else if ((sq7 * 100.0) as i64) != 0 && ((sq8 * 100.0) as i64) == 0 {
                num28 = 1;
            } else if ((sq7 * 100.0) as i64) == 0 && ((sq8 * 100.0) as i64) != 0 {
                num28 = 2;
            } else {
                num28 = if ((sq7 - sq8) as i64) >= 0 { 1 } else { 2 };
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

    /// Helper to compare two sq values and return which layout variant to use.
    fn compare_two_sq(&self, a: f64, b: f64, id_a: i32, id_b: i32) -> i32 {
        let a100 = (a * 100.0) as i64;
        let b100 = (b * 100.0) as i64;
        if a100 == 0 && b100 == 0 {
            -1
        } else if a100 != 0 && b100 == 0 {
            id_a
        } else if a100 == 0 && b100 != 0 {
            id_b
        } else if (((a - b) * 100.0) as i64) >= 0 {
            id_a
        } else {
            id_b
        }
    }

    fn check_part_for_last_in_line(
        &mut self,
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

        let sq = parts[num as usize].sq();
        let num4 = num2;
        let num5 = wo - num3 - self.B;
        let num6 = lo - num2 - self.B;
        let num7 = wo;

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
            let num4b = lo - num2 - self.B;
            let num5b = num3;
            let num6b = lo;
            let num7b = wo - num3 - self.B;
            let mut flag3 = false;
            let mut flag4 = false;

            if num4b >= self.minL && num5b >= self.minW {
                flag3 = self.fast_find_first_part(parts, num4b, num5b);
            }
            if num6b >= self.minL && num7b >= self.minW {
                flag4 = self.fast_find_first_part(parts, num6b, num7b);
            }

            if flag3 || flag4 {
                array[0] = id;
            } else {
                let array2 = self.find_2_parts(parts, lo, wo);
                let mut num8: f64 = 0.0;
                if array2[0] != -1 {
                    if array2[0] >= 0 {
                        num8 += parts[array2[0] as usize].sq();
                    } else {
                        num8 += parts[(array2[0] * -1 - 2) as usize].sq();
                    }
                }
                if array2[1] != -1 {
                    if array2[1] >= 0 {
                        num8 += parts[array2[1] as usize].sq();
                    } else {
                        num8 += parts[(array2[1] * -1 - 2) as usize].sq();
                    }
                }

                if ((num8 - sq) as i64) > 0 {
                    array = array2;
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
                ..Crd::default()
            },
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
        let mut total: f64 = 0.0;
        let mut idx1: i32 = -1;
        let mut idx2: i32 = -1;

        if lo1 >= self.minL && wo1 >= self.minW {
            idx1 = self.find_max_sq_part(parts, lo1, wo1);
            if idx1 != -1 {
                if idx1 < -1 {
                    idx1 = idx1 * -1 - 2;
                }
                total += parts[idx1 as usize].sq();
            }
        }
        if idx1 != -1 {
            parts[idx1 as usize].Plased += 1;
        }

        if lo2 >= self.minL && wo2 >= self.minW {
            idx2 = self.find_max_sq_part(parts, lo2, wo2);
            if idx2 != -1 {
                if idx2 < -1 {
                    idx2 = idx2 * -1 - 2;
                }
                total += parts[idx2 as usize].sq();
            }
        }
        if idx2 != -1 {
            parts[idx2 as usize].Plased += 1;
        }

        if lo3 >= self.minL && wo3 >= self.minW {
            let mut idx3 = self.find_max_sq_part(parts, lo3, wo3);
            if idx3 != -1 {
                if idx3 < -1 {
                    idx3 = idx3 * -1 - 2;
                }
                total += parts[idx3 as usize].sq();
            }
        }

        if idx1 != -1 {
            parts[idx1 as usize].Plased -= 1;
        }
        if idx2 != -1 {
            parts[idx2 as usize].Plased -= 1;
        }

        total
    }

    fn find_max_sq_part(&self, parts: &[CPart], lo: i32, wo: i32) -> i32 {
        let mut result: i32 = -1;
        let mut num: f64 = 0.0;

        if lo > 0 && wo > 0 {
            for i in 0..parts.len() {
                if parts[i].Plased >= parts[i].Qty {
                    continue;
                }
                if parts[i].L <= lo && parts[i].W <= wo {
                    if ((parts[i].sq() - num) as i64) > 0 {
                        result = i as i32;
                        num = parts[i].sq();
                    }
                } else if parts[i].Turn
                    && parts[i].L <= wo
                    && parts[i].W <= lo
                    && ((parts[i].sq() - num) as i64) > 0
                {
                    result = i as i32 * -1 - 2;
                    num = parts[i].sq();
                }
            }
        }

        result
    }

    fn find_max_sq_part_krome(
        &self,
        parts: &[CPart],
        lo: i32,
        wo: i32,
        krome: i32,
    ) -> i32 {
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
                    if ((parts[i].sq() - num) as i64) > 0 {
                        result = i as i32;
                        num = parts[i].sq();
                    }
                } else if parts[i].Turn
                    && parts[i].L <= wo
                    && parts[i].W <= lo
                    && ((parts[i].sq() - num) as i64) > 0
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
        let mut num: f64 = 100_000_000_000.0;

        for i in 0..snips.len() {
            if ((num - snips[i].sq()) as i64) > 0
                && self.fast_find_first_part(parts, snips[i].L, snips[i].W)
            {
                num = snips[i].sq();
                result = i as i32;
            }
        }

        result
    }

    fn find_2_parts(&self, parts: &mut Vec<CPart>, lo: i32, wo: i32) -> [i32; 2] {
        let mut array: [i32; 3] = [-1, -1, 0];
        let mut array2: [i32; 3] = [-1, -1, 1];
        let mut num5: f64 = 0.0;
        let mut num6: f64 = 0.0;

        for i in 0..parts.len() {
            if parts[i].Plased >= parts[i].Qty {
                continue;
            }

            if lo >= parts[i].L && wo >= parts[i].W {
                let sq_val = parts[i].sq();
                let num8 = lo - self.B - parts[i].L;
                let (found, sq2);
                if num8 >= self.minL {
                    found = self.find_max_sq_part_krome(parts, num8, wo, i as i32);
                    sq2 = if found == -1 {
                        0.0
                    } else if found >= 0 {
                        parts[found as usize].sq()
                    } else {
                        parts[(found * -1 - 2) as usize].sq()
                    };
                } else {
                    found = -1;
                    sq2 = 0.0;
                }
                if ((num5 - (sq_val + sq2)) as i64) < 0 {
                    num5 = sq_val + sq2;
                    array[0] = i as i32;
                    array[1] = found;
                }
            }

            if parts[i].Turn && wo >= parts[i].L && lo >= parts[i].W {
                let sq_val = parts[i].sq();
                let num9 = lo - self.B - parts[i].W;
                let (found, sq2);
                if num9 >= self.minL {
                    found = self.find_max_sq_part_krome(parts, num9, wo, i as i32);
                    sq2 = if found == -1 {
                        0.0
                    } else if found >= 0 {
                        parts[found as usize].sq()
                    } else {
                        parts[(found * -1 - 2) as usize].sq()
                    };
                } else {
                    found = -1;
                    sq2 = 0.0;
                }
                if ((num5 - (sq_val + sq2)) as i64) < 0 {
                    num5 = sq_val + sq2;
                    array[0] = i as i32 * -1 - 2;
                    array[1] = found;
                }
            }

            if lo >= parts[i].L && wo >= parts[i].W {
                let sq_val = parts[i].sq();
                let num10 = wo - self.B - parts[i].W;
                let (found, sq2);
                if num10 >= self.minL {
                    found = self.find_max_sq_part_krome(parts, lo, num10, i as i32);
                    sq2 = if found == -1 {
                        0.0
                    } else if found >= 0 {
                        parts[found as usize].sq()
                    } else {
                        parts[(found * -1 - 2) as usize].sq()
                    };
                } else {
                    found = -1;
                    sq2 = 0.0;
                }
                if ((num6 - (sq_val + sq2)) as i64) < 0 {
                    num6 = sq_val + sq2;
                    array2[0] = i as i32;
                    array2[1] = found;
                }
            } else if parts[i].Turn && wo >= parts[i].L && lo >= parts[i].W {
                let sq_val = parts[i].sq();
                let num11 = wo - self.B - parts[i].L;
                let (found, sq2);
                if num11 >= self.minL {
                    found = self.find_max_sq_part_krome(parts, lo, num11, i as i32);
                    sq2 = if found == -1 {
                        0.0
                    } else if found >= 0 {
                        parts[found as usize].sq()
                    } else {
                        parts[(found * -1 - 2) as usize].sq()
                    };
                } else {
                    found = -1;
                    sq2 = 0.0;
                }
                if ((num6 - (sq_val + sq2)) as i64) < 0 {
                    num6 = sq_val + sq2;
                    array2[0] = i as i32 * -1 - 2;
                    array2[1] = found;
                }
            }
        }

        // Sort array pair by WD descending
        if array[0] != -1 && array[1] != -1 {
            let mut idx0 = array[0];
            let wd0 = if idx0 < -1 {
                idx0 = idx0 * -1 - 2;
                parts[idx0 as usize].L
            } else {
                parts[idx0 as usize].W
            };
            let mut idx1 = array[1];
            let wd1 = if idx1 < -1 {
                idx1 = idx1 * -1 - 2;
                parts[idx1 as usize].L
            } else {
                parts[idx1 as usize].W
            };
            if wd1 > wd0 {
                let tmp = array[0];
                array[0] = array[1];
                array[1] = tmp;
            }
        }

        // Sort array2 pair by LD descending
        if array2[0] != -1 && array2[1] != -1 {
            let mut idx0 = array2[0];
            let ld0 = if idx0 < -1 {
                idx0 = idx0 * -1 - 2;
                parts[idx0 as usize].W
            } else {
                parts[idx0 as usize].L
            };
            let mut idx1 = array2[1];
            let ld1 = if idx1 < -1 {
                idx1 = idx1 * -1 - 2;
                parts[idx1 as usize].W
            } else {
                parts[idx1 as usize].L
            };
            if ld1 > ld0 {
                let tmp = array2[0];
                array2[0] = array2[1];
                array2[1] = tmp;
            }
        }

        if ((num5 - num6) as i64) > 0 {
            [array[0], array[1]]
        } else {
            [array2[0], array2[1]]
        }
    }

    fn find_zamena_lines_length_cut(
        &self,
        lines: &[CLine],
        on_sheet: &[bool],
        wo: i32,
        minimal_w: i32,
        check: &mut bool,
    ) -> [i32; 3] {
        let mut array: [i32; 3] = [-1, -1, -1];
        *check = false;
        let mut num: i32 = 0;

        for i in 0..lines.len() {
            let w = lines[i].W;
            if on_sheet[i] || wo < w {
                continue;
            }

            if w > num {
                array = [i as i32, -1, -1];
                num = w;
                *check = true;
            } else if w == num {
                let mut sum_sq: f64 = 0.0;
                for j in 0..3 {
                    if array[j] != -1 {
                        sum_sq += lines[array[j] as usize].Parts_Sq;
                    }
                }
                if ((lines[i].Parts_Sq - sum_sq) as i32) >= 0 {
                    array = [i as i32, -1, -1];
                    num = w;
                    *check = true;
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
                        array = [i as i32, k as i32, -1];
                        num = w + w2;
                        *check = true;
                    } else if w + w2 - num == 0 {
                        let mut sum_sq: f64 = 0.0;
                        for idx in 0..3 {
                            if array[idx] != -1 {
                                sum_sq += lines[array[idx] as usize].Parts_Sq;
                            }
                        }
                        if ((lines[i].Parts_Sq + lines[k].Parts_Sq - sum_sq) as i32) >= 0 {
                            array = [i as i32, k as i32, -1];
                            num = w + w2;
                            *check = true;
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
                        array = [i as i32, k as i32, m as i32];
                        num = w + w2 + w3;
                        *check = true;
                    } else if w + w2 + w3 - num == 0 {
                        let mut sum_sq: f64 = 0.0;
                        for idx in 0..3 {
                            if array[idx] != -1 {
                                sum_sq += lines[array[idx] as usize].Parts_Sq;
                            }
                        }
                        if ((lines[i].Parts_Sq + lines[k].Parts_Sq + lines[m].Parts_Sq - sum_sq)
                            as i32)
                            >= 0
                        {
                            array = [i as i32, k as i32, m as i32];
                            num = w + w2 + w3;
                            *check = true;
                        }
                    }
                }
            }
        }

        array
    }

    fn find_zamena_parts_length_cut(
        &self,
        fix: &[i32],
        parts: &[CPart],
        wo: i32,
        mut max_l: i32,
        minimal_l: i32,
        check: &mut bool,
    ) -> [i32; 3] {
        let mut array: [i32; 3] = [-1, -1, -1];
        *check = false;

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
                array = [i as i32, -1, -1];
                max_l = num;
                *check = true;
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
                    array = [i as i32, j as i32, -1];
                    max_l = num + num2;
                    *check = true;
                }

                if wo - num - self.B - num2 - self.B - minimal_l < 0 {
                    continue;
                }

                for k in (j + 1)..fix.len() {
                    let num3 = if fix[k] <= -1 {
                        parts[(fix[k] * -1 - 2) as usize].W
                    } else {
                        parts[fix[k] as usize].L
                    };

                    if wo - num - self.B - num2 - self.B - num3 >= 0
                        && num + num2 + num3 - max_l > 0
                    {
                        array = [i as i32, j as i32, k as i32];
                        max_l = num + num2 + num3;
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

    fn get_parts_with_fix_width(
        &self,
        parts: &[CPart],
        w: i32,
        l: i32,
    ) -> (Vec<i32>, i32, i32) {
        let mut list: Vec<i32> = Vec::new();
        let mut min_l = l;
        let mut total_length: i32 = 0;

        for i in 0..parts.len() {
            let cp = &parts[i];
            if cp.Qty <= cp.Plased {
                continue;
            }
            if cp.W == w {
                for _ in 0..(cp.Qty - cp.Plased) {
                    list.push(i as i32);
                    total_length += cp.L;
                }
                if min_l > cp.L {
                    min_l = cp.L;
                }
            } else if cp.Turn && cp.L == w {
                for _ in 0..(cp.Qty - cp.Plased) {
                    list.push(i as i32 * -1 - 2);
                    total_length += cp.W;
                }
                if min_l > cp.W {
                    min_l = cp.W;
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

        {
            let mut i: i32 = 0;
            while (i as usize) < fix_width.len() {
                let (_, ld, _) = Self::get_id_ld_wd(parts, fix_width[i as usize]);
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
        }

        let mut flag = false;
        let mut num2 = 0;

        while !flag && num2 < self.PARTS_SORT_LIMIT {
            num2 += 1;
            let mut best_total = 0i32;
            let mut check = false;
            let mut num4: i32 = -1;
            let mut num5: i32 = -1;
            let mut num6: i32 = 0;
            let mut num7: i32 = 0;
            let mut array: Option<[i32; 3]> = None;

            let list_len = list.len() as i32;
            if list_len >= 2 {
                for j in 0..(list_len - 1) as usize {
                    for k in (j + 1)..list_len as usize {
                        fix_width.push(list[j]);
                        fix_width.push(list[k]);

                        let (_, ld2, _) = Self::get_id_ld_wd(parts, list[j]);
                        let (_, ld3, _) = Self::get_id_ld_wd(parts, list[k]);

                        let w_o = num + self.B + ld2 + self.B + ld3;
                        let array2 = self.find_zamena_parts_length_cut(
                            &fix_width,
                            parts,
                            w_o,
                            ld2 + ld3,
                            minimal_l,
                            &mut check,
                        );

                        if check {
                            let mut total = 0i32;
                            for idx in 0..array2.len() {
                                if array2[idx] != -1 {
                                    let (_, ld4, _) =
                                        Self::get_id_ld_wd(parts, fix_width[array2[idx] as usize]);
                                    total = total + self.B + ld4;
                                }
                            }
                            if total > best_total {
                                num4 = j as i32;
                                num5 = k as i32;
                                num6 = ld2;
                                num7 = ld3;
                                array = Some(array2);
                                best_total = total;
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
                list.remove((num5 - 1) as usize);
                num = num + self.B + num6 + self.B + num7;

                if let Some(arr) = array {
                    for m in 0..arr.len() {
                        if arr[m] != -1 {
                            list.push(fix_width[arr[m] as usize]);
                            let (_, ld5, _) =
                                Self::get_id_ld_wd(parts, fix_width[arr[m] as usize]);
                            num = num - self.B - ld5;
                        }
                    }

                    let mut removed = 0i32;
                    for n in 0..arr.len() {
                        if arr[n] != -1 {
                            fix_width.remove((arr[n] - removed) as usize);
                            removed += 1;
                        }
                    }
                }
            } else {
                flag = true;
            }
        }

        // Sort list by LD descending
        let len = list.len();
        for i in 0..len {
            for j in (i + 1)..len {
                let (_, ld_i, _) = Self::get_id_ld_wd(parts, list[i]);
                let (_, ld_j, _) = Self::get_id_ld_wd(parts, list[j]);
                if ld_j > ld_i {
                    list.swap(i, j);
                }
            }
        }

        list
    }
}
