#![allow(non_snake_case)]
#![allow(unused_assignments)]
#![allow(dead_code)]
#![allow(clippy::needless_range_loop)]
#![allow(clippy::comparison_chain)]

use super::algorithm_types::*;
use std::time::Instant;

/// Width2 algorithm: a mirror of Length2 with L/W swapped.
/// Uses LW16 parameter variants for 16 pre-computed sorted part lists.
/// Alg = 2.
pub struct Width2 {
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
    PartsCount: i32,
    PartsCutted: i32,
    PartsSq: f64,
    ListSQ: f64,
}

impl Width2 {
    pub fn new() -> Self {
        Self {
            THE_SAME_PARTS_LIMIT: 25,
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
            PartsCount: 0,
            PartsCutted: 0,
            PartsSq: 0.0,
            ListSQ: 0.0,
        }
    }

    // -----------------------------------------------------------------------
    // Public helpers
    // -----------------------------------------------------------------------

    pub fn GetCPartsSq(parts: &[CPart]) -> f64 {
        let mut num = 0.0;
        for i in 0..parts.len() {
            num += parts[i].sq() * parts[i].Qty as f64;
        }
        num
    }

    // -----------------------------------------------------------------------
    // Public entry point
    // Returns (CSheet, PPSQ_OUT)
    // -----------------------------------------------------------------------
    pub fn GetCSheet_WIDTH_CUT(
        &mut self,
        parts: &mut Vec<CPart>,
        ListLength: i32,
        ListWidth: i32,
        Blade: i32,
        Padding: i32,
        DoublePadding: bool,
        PARAMS: LW16,
        PSQ: f64,
        PPSQ: f64,
    ) -> (Option<CSheet>, f64) {
        self.L_L = ListLength;
        self.L_W = ListWidth;
        self.P = Padding;
        self.B = Blade;

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

        let mut cSheet = CSheet {
            Alg: 2,
            L: ListLength,
            W: ListWidth,
            ..Default::default()
        };
        let mut lines_index: Vec<i32> = Vec::new();

        let mut num = Padding;
        if DoublePadding {
            num *= 2;
        }
        let num2 = cSheet.L - num;
        let num3 = cSheet.W - num;

        // Build candidate lines
        let mut cLines_WIDTH_CUT = self.GetCLines_WIDTH_CUT(parts, num2, num3, PARAMS);

        // Find minimum line L
        let mut num4 = num2;
        for i in 0..cLines_WIDTH_CUT.len() {
            if num4 > cLines_WIDTH_CUT[i].L {
                num4 = cLines_WIDTH_CUT[i].L;
            }
        }

        // Greedily fill sheet with lines (by L)
        let mut num5 = num2;
        for j in 0..cLines_WIDTH_CUT.len() {
            if num5 >= cLines_WIDTH_CUT[j].L {
                num5 = num5 - cLines_WIDTH_CUT[j].L - self.B;
                cSheet.Lines.push(cLines_WIDTH_CUT[j].clone());
                lines_index.push(j as i32);
                cLines_WIDTH_CUT[j].onSheet = true;
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

            let sheet_line_count = cSheet.Lines.len();
            if sheet_line_count >= 2 {
                for k in 0..sheet_line_count - 1 {
                    for l in k + 1..sheet_line_count {
                        cSheet.Lines[k].onSheet = false;
                        cSheet.Lines[l].onSheet = false;

                        let wo = num5 + self.B + cSheet.Lines[k].L + self.B + cSheet.Lines[l].L;
                        let mut check = false;
                        let array2 = Self::Find_Zamena_Lines_WIDTH_CUT_impl(
                            &cLines_WIDTH_CUT,
                            wo,
                            num4,
                            self.B,
                            &mut check,
                        );

                        if lines_index[k] != array2[0]
                            || lines_index[l] != array2[1]
                            || array2[2] != -1
                        {
                            let num10 = cSheet.Lines[k].L + cSheet.Lines[l].L;
                            let num11 = cSheet.Lines[k].Parts_Sq + cSheet.Lines[l].Parts_Sq;
                            let mut num12 = 0;
                            let mut num13: f64 = 0.0;
                            for m in 0..3 {
                                if array2[m] != -1 {
                                    num13 += cLines_WIDTH_CUT[array2[m] as usize].Parts_Sq;
                                    num12 += cLines_WIDTH_CUT[array2[m] as usize].L;
                                }
                            }
                            if num12 >= num10 && (num13 - num11) as i64 >= 0 {
                                let better = (num13 - num9) as i64 > 0
                                    || ((num13 - num9) as i64 == 0
                                        && num7 >= 0
                                        && num8 >= 0
                                        && num12
                                            - cSheet.Lines[num7 as usize].L
                                            - cSheet.Lines[num8 as usize].L
                                            > 0);
                                if better {
                                    num7 = k as i32;
                                    num8 = l as i32;
                                    best_arr = Some(array2);
                                    num9 = num13;
                                }
                            }
                        }

                        cSheet.Lines[k].onSheet = true;
                        cSheet.Lines[l].onSheet = true;
                    }
                }
            }

            if num7 != -1 && num8 != -1 {
                let array = best_arr.unwrap();
                num5 = num5
                    + self.B
                    + cSheet.Lines[num7 as usize].L
                    + self.B
                    + cSheet.Lines[num8 as usize].L;

                cSheet.Lines[num7 as usize].onSheet = false;
                cSheet.Lines[num8 as usize].onSheet = false;

                cSheet.Lines.remove(num7 as usize);
                lines_index.remove(num7 as usize);
                cSheet.Lines.remove((num8 - 1) as usize);
                lines_index.remove((num8 - 1) as usize);

                for n in 0..3 {
                    if array[n] != -1 {
                        cSheet.Lines.push(cLines_WIDTH_CUT[array[n] as usize].clone());
                        lines_index.push(array[n]);
                        cLines_WIDTH_CUT[array[n] as usize].onSheet = true;
                        num5 = num5 - self.B - cLines_WIDTH_CUT[array[n] as usize].L;
                    }
                }
            } else {
                flag = true;
            }
        }

        // Remain
        cSheet.Remain = CSnip {
            L: num5,
            W: num3,
            ..Default::default()
        };

        // Sort sheet lines by L descending (bubble sort)
        for num14 in 0..cSheet.Lines.len().saturating_sub(1) {
            for num15 in num14 + 1..cSheet.Lines.len() {
                if cSheet.Lines[num15].L > cSheet.Lines[num14].L {
                    let value = lines_index[num14];
                    lines_index[num14] = lines_index[num15];
                    lines_index[num15] = value;
                    cSheet.Lines.swap(num14, num15);
                }
            }
        }

        // SET_OFF parts for lines not on sheet
        for num16 in 0..cLines_WIDTH_CUT.len() {
            if !cLines_WIDTH_CUT[num16].onSheet {
                Self::SET_OFF_Parts_in_Line(parts, &cLines_WIDTH_CUT[num16]);
            }
        }

        // Accumulate Parts_Sq and continue filling each line
        cSheet.Parts_Sq = 0.0;
        for num17 in (0..cSheet.Lines.len()).rev() {
            self.Continue_Line_WIDTH_CUT(&mut cSheet.Lines[num17], parts, PARAMS);
            cSheet.Parts_Sq += cSheet.Lines[num17].Parts_Sq;
        }

        // Fill remaining space with more lines
        flag = false;
        while !flag {
            if Self::FastFindFirstPart(parts, cSheet.Remain.L, cSheet.Remain.W) {
                let mut cLine = CLine::default();
                let mut cSnip = CSnip::default();
                cLine.Snips = Vec::new();
                cLine.PartIDs = Vec::new();
                cLine.Parts_Crds = Vec::new();

                let num18 = self.Find_LENGTH_part(parts, cSheet.Remain.L, cSheet.Remain.W, true);
                let (_id, LD, _WD) = Self::Get_ID_LD_WD(parts, num18);

                cLine.W = cSheet.Remain.W;
                cLine.L = LD;
                cSnip.CRD = Crd { X: 0, Y: 0, ..Default::default() };
                cSnip.L = cLine.L;
                cSnip.W = cLine.W;
                cLine.Snips.push(cSnip);

                let io = self.FindSmallSnip(&cLine.Snips, parts);
                self.Place_Part_to_Line(&mut cLine, parts, num18, io, true);
                self.Continue_Line_WIDTH_CUT(&mut cLine, parts, PARAMS);
                cSheet.Remain.L = cSheet.Remain.L - self.B - cLine.L;
                cSheet.Lines.push(cLine);
                cSheet.Parts_Sq += cSheet.Lines.last().unwrap().Parts_Sq;
            } else {
                flag = true;
            }
        }

        let PPSQ_OUT = PPSQ + cSheet.Parts_Sq;
        (Some(cSheet), PPSQ_OUT)
    }

    // -----------------------------------------------------------------------
    // Private helpers
    // -----------------------------------------------------------------------

    fn Continue_Line_WIDTH_CUT(
        &mut self,
        LINE: &mut CLine,
        parts: &mut Vec<CPart>,
        PARAMS: LW16,
    ) {
        let mut num: i32;
        let mut num2 = self.FindSmallSnip(&LINE.Snips, parts);
        while num2 >= 0 {
            let l = LINE.Snips[num2 as usize].L;
            let w = LINE.Snips[num2 as usize].W;
            num = if PARAMS.MAX_SQ {
                Self::FindMaxSqPart(parts, l, w)
            } else {
                self.Find_LENGTH_part(parts, l, w, true)
            };
            if num != -1 {
                if PARAMS.OPTI_ON {
                    let array = self.Check_part_for_last_in_Line(parts, l, w, num);
                    if num != array[0] && array[0] != -1 {
                        self.Place_2_Parts_to_Line(LINE, parts, &array, num2);
                    } else {
                        self.Place_Part_to_Line(LINE, parts, num, num2, true);
                    }
                } else {
                    self.Place_Part_to_Line(LINE, parts, num, num2, true);
                }
            } else {
                break;
            }
            num2 = self.FindSmallSnip(&LINE.Snips, parts);
        }
    }

    fn GetCLines_WIDTH_CUT(
        &mut self,
        parts: &mut Vec<CPart>,
        LL: i32,
        LW: i32,
        PARAMS: LW16,
    ) -> Vec<CLine> {
        let start_time = Instant::now();
        let mut list: Vec<CLine> = Vec::new();
        let mut num: i32;
        let mut num2 = 0;
        let mut flag = false;
        let mut num3 = LL;
        let num4 = LW;

        while !flag && num2 < self.LINES_LIMIT {
            num2 += 1;
            let mut cLine: Option<CLine> = None;
            let mut cLine2: Option<CLine> = None;

            num = if PARAMS.MAX_SQ {
                Self::FindMaxSqPart(parts, LL, LW)
            } else {
                self.Find_LENGTH_part(parts, num3, num4, false)
            };

            if num != -1 {
                let (line, _pre_cut) = self.MakeLine_WIDTH_CUT(parts, num, num3, num4, PARAMS);
                cLine = line;

                if PARAMS.TURN_ON {
                    let mut index = num;
                    if num < -1 {
                        index = num * -1 - 2;
                    }
                    if parts[index as usize].Turn
                        && ((num < -1
                            && num3 >= parts[index as usize].L
                            && num4 >= parts[index as usize].W)
                            || (num > -1
                                && num3 >= parts[index as usize].W
                                && num4 >= parts[index as usize].L))
                    {
                        let (line2, _pre_cut2) = self.MakeLine_WIDTH_CUT(
                            parts,
                            num * -1 - 2,
                            num3,
                            num4,
                            PARAMS,
                        );
                        cLine2 = line2;
                    }
                    if let (Some(ref cl), Some(ref cl2)) = (&cLine, &cLine2) {
                        if ((cl.filling() - cl2.filling()) * 100.0) as i32 > 0 {
                            // keep cLine
                        } else if (((cl.filling() - cl2.filling()) * 100.0) as i32) < 0 {
                            cLine = cLine2.clone();
                        }
                    }
                }
            }

            if let Some(ref cl) = cLine {
                Self::SET_ON_Parts_in_Line(parts, cl);
                list.push(cl.clone());
                num3 = num3 - self.B - cl.L;
                if !Self::FastFindFirstPart(parts, num3, num4) {
                    num3 = LL;
                    if !Self::FastFindFirstPart(parts, num3, num4) {
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

        list
    }

    fn MakeLine_WIDTH_CUT(
        &mut self,
        parts: &mut Vec<CPart>,
        startPart: i32,
        LineLength: i32,
        LineWidth: i32,
        PARAMS: LW16,
    ) -> (Option<CLine>, Option<CLine>) {
        let mut cLine = CLine {
            Snips: Vec::new(),
            PartIDs: Vec::new(),
            Parts_Crds: Vec::new(),
            ..Default::default()
        };

        let (_id, LD, _WD) = Self::Get_ID_LD_WD(parts, startPart);
        let rez = true;
        cLine.W = LineWidth;
        cLine.L = LD;

        let cSnip = CSnip {
            CRD: Crd { X: 0, Y: 0, ..Default::default() },
            L: cLine.L,
            W: cLine.W,
            ..Default::default()
        };
        cLine.Snips.push(cSnip);

        let mut io: i32;

        if PARAMS.SAME_MAX {
            let (fix_length, Min_W, _Total_Length) =
                self.Get_Parts_with_FixLength(parts, LD, cLine.W, PARAMS.TURN_ON);
            let start_parts =
                self.GetStartParts_for_Line_WIDTH_CUT(parts, fix_length, cLine.W, Min_W);
            io = self.FindSmallSnip(&cLine.Snips, parts);
            for i in 0..start_parts.len() {
                self.Place_Part_to_Line(&mut cLine, parts, start_parts[i], io, rez);
            }
        } else {
            io = 0;
            self.Place_Part_to_Line(&mut cLine, parts, startPart, io, rez);
            let mut flag = false;
            io = self.FindSmallSnip(&cLine.Snips, parts);
            if io != -1 {
                while !flag {
                    let l = cLine.Snips[io as usize].L;
                    let w = cLine.Snips[io as usize].W;
                    let num = self.Find_THE_SAME_LENGTH_part(parts, l, w, PARAMS.TURN_ON);
                    if num != -1 {
                        self.Place_Part_to_Line(&mut cLine, parts, num, io, rez);
                    } else {
                        flag = true;
                    }
                }
            }
        }

        let PreCut = Self::CopyLine_WITHOUT_MARKS(&cLine);
        self.Continue_Line_WIDTH_CUT(&mut cLine, parts, PARAMS);
        Self::SET_OFF_Parts_in_Line(parts, &cLine);

        (Some(cLine), Some(PreCut))
    }

    fn CopyLine_WITHOUT_MARKS(LINE: &CLine) -> CLine {
        let mut cLine = CLine {
            Snips: Vec::new(),
            PartIDs: Vec::new(),
            Parts_Crds: Vec::new(),
            L: LINE.L,
            W: LINE.W,
            Parts_Sq: LINE.Parts_Sq,
            ..Default::default()
        };
        for i in 0..LINE.PartIDs.len() {
            cLine.PartIDs.push(LINE.PartIDs[i]);
            cLine.Parts_Crds.push(LINE.Parts_Crds[i].clone());
        }
        for j in 0..LINE.Snips.len() {
            let s = CSnip {
                L: LINE.Snips[j].L,
                W: LINE.Snips[j].W,
                CRD: Crd {
                    X: LINE.Snips[j].CRD.X,
                    Y: LINE.Snips[j].CRD.Y,
                    ..Default::default()
                },
                ..Default::default()
            };
            cLine.Snips.push(s);
        }
        cLine
    }

    fn Find_2_Lines(
        &self,
        Lines: &[CLine],
        size: i32,
        SQ: f64,
        REZ: bool,
        check: &mut bool,
        SQ_zamena: &mut f64,
    ) -> [i32; 2] {
        let mut array = [-1i32, -1];
        *check = false;
        let mut num: i32 = -1;
        let mut num2: i32 = -1;
        *SQ_zamena = 0.0;
        let mut num3: i32;
        let mut num4: i32;
        let mut num5 = SQ;

        for i in 0..Lines.len() {
            if Lines[i].onSheet {
                continue;
            }
            num3 = if REZ { Lines[i].L } else { Lines[i].W };
            if size < num3 {
                continue;
            }
            for j in 0..Lines.len() {
                if i != j && !Lines[j].onSheet {
                    num4 = if REZ { Lines[j].L } else { Lines[j].W };
                    if size - num3 - self.B - num4 >= 0
                        && (Lines[i].Parts_Sq + Lines[j].Parts_Sq - num5) as i64 > 0
                    {
                        num5 = Lines[i].Parts_Sq + Lines[j].Parts_Sq;
                        num = i as i32;
                        num2 = j as i32;
                        *check = true;
                    }
                }
            }
        }

        array[0] = num;
        array[1] = num2;
        if *check {
            *SQ_zamena = Lines[num as usize].Parts_Sq + Lines[num2 as usize].Parts_Sq;
        }
        array
    }

    fn FastFindFirstPart(parts: &[CPart], LO: i32, WO: i32) -> bool {
        if LO <= 0 || WO <= 0 {
            return false;
        }
        for num in (0..parts.len()).rev() {
            if parts[num].Plased < parts[num].Qty
                && ((LO >= parts[num].L && WO >= parts[num].W)
                    || (parts[num].Turn && LO >= parts[num].W && WO >= parts[num].L))
            {
                return true;
            }
        }
        false
    }

    fn Find_LENGTH_part(
        &self,
        parts: &[CPart],
        LO: i32,
        WO: i32,
        Max_L: bool,
    ) -> i32 {
        let mut result: i32 = -1;
        let mut num: i32 = 0;
        let mut num2: f64 = 0.0;

        for i in 0..parts.len() {
            if parts[i].Plased >= parts[i].Qty {
                continue;
            }
            if !parts[i].Turn {
                if LO >= parts[i].L && WO >= parts[i].W {
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
            } else if parts[i].Turn {
                let mut num3: i32 = 0;
                if LO >= parts[i].L && WO >= parts[i].W && LO >= parts[i].W && WO >= parts[i].L {
                    num3 = if Max_L {
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
                } else if LO >= parts[i].L
                    && WO >= parts[i].W
                    && (LO < parts[i].W || WO < parts[i].L)
                {
                    num3 = parts[i].L;
                } else if (LO < parts[i].L || WO < parts[i].W)
                    && LO >= parts[i].W
                    && WO >= parts[i].L
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

    fn Find_THE_SAME_LENGTH_part(
        &self,
        parts: &[CPart],
        LO: i32,
        WO: i32,
        TURN_ON: bool,
    ) -> i32 {
        let mut result: i32 = -1;
        let mut num: f64 = 0.0;

        for i in 0..parts.len() {
            if parts[i].Plased >= parts[i].Qty {
                continue;
            }
            if !TURN_ON {
                if !parts[i].Turn {
                    if LO == parts[i].L && WO >= parts[i].W && (parts[i].sq() - num) as i64 > 0 {
                        num = parts[i].sq();
                        result = i as i32;
                    }
                } else {
                    if !parts[i].Turn {
                        continue;
                    }
                    if parts[i].L > parts[i].W && LO == parts[i].W && WO >= parts[i].L {
                        if (parts[i].sq() - num) as i64 > 0 {
                            num = parts[i].sq();
                            result = i as i32 * -1 - 2;
                        }
                    } else if parts[i].W > parts[i].L
                        && LO == parts[i].L
                        && WO >= parts[i].W
                        && (parts[i].sq() - num) as i64 > 0
                    {
                        num = parts[i].sq();
                        result = i as i32;
                    }
                }
            } else if LO == parts[i].L && WO >= parts[i].W {
                if (parts[i].sq() - num) as i64 > 0 {
                    num = parts[i].sq();
                    result = i as i32;
                }
            } else if parts[i].Turn
                && LO == parts[i].W
                && WO >= parts[i].L
                && (parts[i].sq() - num) as i64 > 0
            {
                num = parts[i].sq();
                result = i as i32 * -1 - 2;
            }
        }
        result
    }

    fn Place_Part_to_Line(
        &self,
        line: &mut CLine,
        parts: &mut Vec<CPart>,
        part_id: i32,
        io: i32,
        _rez: bool,
    ) {
        let (ID, LD, WD) = Self::Get_ID_LD_WD(parts, part_id);
        let io = io as usize;
        line.Parts_Sq += parts[ID as usize].sq();
        line.PartIDs.push(part_id);

        let crd = Crd {
            X: line.Snips[io].CRD.X,
            Y: line.Snips[io].CRD.Y,
            id_in_order: parts[ID as usize].iD_in_Order,
        };
        line.Parts_Crds.push(crd);
        parts[ID as usize].Plased += 1;

        let l = line.Snips[io].L;
        let w = line.Snips[io].W;

        if l > LD && w > WD {
            if _rez {
                let num = l - LD - self.B;
                let num2 = w;
                let x = line.Snips[io].CRD.X + LD + self.B;
                let y = line.Snips[io].CRD.Y;
                if Self::FastFindFirstPart(parts, num, num2) {
                    let item = Self::Create_CSnip(x, y, num, num2);
                    line.Snips.push(item);
                    let snip_x = line.Snips[io].CRD.X;
                    let snip_y = line.Snips[io].CRD.Y + WD + self.B;
                    Self::Resize_CSnip(&mut line.Snips[io], snip_x, snip_y, LD, w - WD - self.B);
                } else {
                    let item2 = Self::Create_CSnip(x, y, num, WD);
                    line.Snips.push(item2);
                    let snip_x = line.Snips[io].CRD.X;
                    let snip_y = line.Snips[io].CRD.Y + WD + self.B;
                    Self::Resize_CSnip(
                        &mut line.Snips[io],
                        snip_x,
                        snip_y,
                        l,
                        w - WD - self.B,
                    );
                }
            } else {
                let num3 = l;
                let num4 = w - WD - self.B;
                let x2 = line.Snips[io].CRD.X;
                let y2 = line.Snips[io].CRD.Y + WD + self.B;
                if Self::FastFindFirstPart(parts, num3, num4) {
                    let item3 = Self::Create_CSnip(x2, y2, num3, num4);
                    line.Snips.push(item3);
                    let snip_x = line.Snips[io].CRD.X + LD + self.B;
                    let snip_y = line.Snips[io].CRD.Y;
                    Self::Resize_CSnip(
                        &mut line.Snips[io],
                        snip_x,
                        snip_y,
                        l - LD - self.B,
                        WD,
                    );
                } else {
                    let item4 = Self::Create_CSnip(x2, y2, LD, num4);
                    line.Snips.push(item4);
                    let snip_x = line.Snips[io].CRD.X + LD + self.B;
                    let snip_y = line.Snips[io].CRD.Y;
                    Self::Resize_CSnip(
                        &mut line.Snips[io],
                        snip_x,
                        snip_y,
                        l - LD - self.B,
                        w,
                    );
                }
            }
        } else if LD == l && WD < w {
            let snip_x = line.Snips[io].CRD.X;
            let snip_y = line.Snips[io].CRD.Y + WD + self.B;
            Self::Resize_CSnip(&mut line.Snips[io], snip_x, snip_y, l, w - WD - self.B);
        } else if LD < l && WD == w {
            let snip_x = line.Snips[io].CRD.X + LD + self.B;
            let snip_y = line.Snips[io].CRD.Y;
            Self::Resize_CSnip(&mut line.Snips[io], snip_x, snip_y, l - LD - self.B, w);
        } else if LD == l && WD == w {
            let snip_x = line.Snips[io].CRD.X;
            let snip_y = line.Snips[io].CRD.Y;
            Self::Resize_CSnip(&mut line.Snips[io], snip_x, snip_y, 0, 0);
        }
    }

    fn Place_2_Parts_to_Line(
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

        let mut num: i32 = 0;
        let mut num2: i32 = 0;

        let mut num3 = _2parts[0];
        let mut num4_id = _2parts[1];

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
            num4_id = num4_idx;
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
                    let sq1 = self.GetSqPartsForSnips(
                        parts,
                        l, w - num6 - self.B,
                        num, num6 - num2 - self.B,
                        l - num5 - num - 2 * self.B, num6,
                    );
                    let sq2 = self.GetSqPartsForSnips(
                        parts,
                        num5, w - num6 - self.B,
                        num, w - num2 - self.B,
                        l - num5 - num - 2 * self.B, w,
                    );
                    let sq3 = self.GetSqPartsForSnips(
                        parts,
                        num5, w - num6 - self.B,
                        l - num5 - self.B, w - num2 - self.B,
                        l - num5 - num - 2 * self.B, num2,
                    );
                    let sq4 = self.GetSqPartsForSnips(
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
                    } else if (((best_sq_a - best_sq_b) * 100.0) as i64) < 0 {
                        num17 = num18;
                        best_sq_a = best_sq_b;
                    }

                    num13 -= best_sq_a;
                    num11 = num17;
                } else {
                    let sq5 = self.GetSqPartsForSnips(
                        parts,
                        l, w - num6 - self.B,
                        l - num5 - num - 2 * self.B, num6,
                        0, 0,
                    );
                    let sq6 = self.GetSqPartsForSnips(
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
                    let sq1 = self.GetSqPartsForSnips(
                        parts,
                        l - num5 - self.B, w,
                        num5 - num - self.B, num2,
                        num5, w - num6 - num2 - 2 * self.B,
                    );
                    let sq2 = self.GetSqPartsForSnips(
                        parts,
                        l - num5 - self.B, w,
                        num5 - num - self.B, w - num6 - self.B,
                        num, w - num6 - num2 - 2 * self.B,
                    );
                    let sq3 = self.GetSqPartsForSnips(
                        parts,
                        l - num5 - self.B, num6,
                        l - num - self.B, num2,
                        l, w - num6 - num2 - 2 * self.B,
                    );
                    let sq4 = self.GetSqPartsForSnips(
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
                    } else if (((best_sq_a - best_sq_b) * 100.0) as i64) < 0 {
                        num23 = num24;
                        best_sq_a = best_sq_b;
                    }

                    num14 -= best_sq_a;
                    num12 = num23;
                } else {
                    let sq5 = self.GetSqPartsForSnips(
                        parts,
                        l - num5 - self.B, w,
                        num, w - num6 - num2 - 2 * self.B,
                        0, 0,
                    );
                    let sq6 = self.GetSqPartsForSnips(
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
            } else if flag && !flag2 {
                num27 = num11 * -1;
            } else if !flag && flag2 {
                num27 = num12;
            } else {
                num27 = 1;
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
                    Self::Resize_CSnip(&mut line.Snips[io], x, y + num6 + self.B, l, w - num6 - self.B);
                    line.Snips.push(Self::Create_CSnip(x + num5 + self.B, y + num2 + self.B, num, num6 - num2 - self.B));
                    line.Snips.push(Self::Create_CSnip(x + num5 + num + 2 * self.B, y, l - num5 - num - 2 * self.B, num6));
                }
                -2 => {
                    Self::Resize_CSnip(&mut line.Snips[io], x, y + num6 + self.B, num5, w - num6 - self.B);
                    line.Snips.push(Self::Create_CSnip(x + num5 + self.B, y + num2 + self.B, num, w - num2 - self.B));
                    line.Snips.push(Self::Create_CSnip(x + num5 + num + 2 * self.B, y, l - num5 - num - 2 * self.B, w));
                }
                -3 => {
                    Self::Resize_CSnip(&mut line.Snips[io], x, y + num6 + self.B, num5, w - num6 - self.B);
                    line.Snips.push(Self::Create_CSnip(x + num5 + self.B, y + num2 + self.B, l - num5 - self.B, w - num2 - self.B));
                    line.Snips.push(Self::Create_CSnip(x + num5 + num + 2 * self.B, y, l - num5 - num - 2 * self.B, num2));
                }
                -4 => {
                    Self::Resize_CSnip(&mut line.Snips[io], x, y + num6 + self.B, l, w - num6 - self.B);
                    line.Snips.push(Self::Create_CSnip(x + num5 + self.B, y + num2 + self.B, l - num5 - self.B, num6 - num2 - self.B));
                    line.Snips.push(Self::Create_CSnip(x + num5 + num + 2 * self.B, y, l - num5 - num - 2 * self.B, num2));
                }
                -5 => {
                    Self::Resize_CSnip(&mut line.Snips[io], x, y + num6 + self.B, l, w - num6 - self.B);
                    line.Snips.push(Self::Create_CSnip(x + num5 + num + 2 * self.B, y, l - num5 - num - 2 * self.B, num6));
                }
                -6 => {
                    Self::Resize_CSnip(&mut line.Snips[io], x, y + num6 + self.B, num5, w - num6 - self.B);
                    line.Snips.push(Self::Create_CSnip(x + num5 + self.B, y + num2 + self.B, num, w - num2 - self.B));
                    line.Snips.push(Self::Create_CSnip(x + num5 + num + 2 * self.B, y, l - num5 - num - 2 * self.B, w));
                }
                1 => {
                    Self::Resize_CSnip(&mut line.Snips[io], x + num5 + self.B, y, l - num5 - self.B, w);
                    line.Snips.push(Self::Create_CSnip(x + num + self.B, y + num6 + self.B, num5 - num - self.B, num2));
                    line.Snips.push(Self::Create_CSnip(x, y + num6 + num2 + 2 * self.B, num5, w - num6 - num2 - 2 * self.B));
                }
                2 => {
                    Self::Resize_CSnip(&mut line.Snips[io], x + num5 + self.B, y, l - num5 - self.B, w);
                    line.Snips.push(Self::Create_CSnip(x + num + self.B, y + num6 + self.B, num5 - num - self.B, w - num6 - self.B));
                    line.Snips.push(Self::Create_CSnip(x, y + num6 + num2 + 2 * self.B, num, w - num6 - num2 - 2 * self.B));
                }
                3 => {
                    Self::Resize_CSnip(&mut line.Snips[io], x + num5 + self.B, y, l - num5 - self.B, -num6);
                    line.Snips.push(Self::Create_CSnip(x + num + self.B, y + num6 + self.B, l - num - self.B, num2));
                    line.Snips.push(Self::Create_CSnip(x, y + num6 + num2 + 2 * self.B, l, w - num6 - num2 - 2 * self.B));
                }
                4 => {
                    Self::Resize_CSnip(&mut line.Snips[io], x + num5 + self.B, y, l - num5 - self.B, num6);
                    line.Snips.push(Self::Create_CSnip(x + num + self.B, y + num6 + self.B, l - num - self.B, w - num6 - self.B));
                    line.Snips.push(Self::Create_CSnip(x, y + num6 + num2 + 2 * self.B, num, w - num6 - num2 - 2 * self.B));
                }
                5 => {
                    Self::Resize_CSnip(&mut line.Snips[io], x + num5 + self.B, y, l - num5 - self.B, w);
                    line.Snips.push(Self::Create_CSnip(x, y + num6 + num2 + 2 * self.B, num, w - num6 - num2 - 2 * self.B));
                }
                6 => {
                    Self::Resize_CSnip(&mut line.Snips[io], x + num5 + self.B, y, l - num5 - self.B, num6);
                    line.Snips.push(Self::Create_CSnip(x + num + self.B, y + num6 + self.B, l - num - self.B, num2));
                    line.Snips.push(Self::Create_CSnip(x, y + num6 + num2 + 2 * self.B, l, w - num6 - num2 - 2 * self.B));
                }
                _ => { /* 0 => do nothing */ }
            }
        } else {
            // Only one part - choose best layout for remaining snips
            let sq7 = self.GetSqPartsForSnips(
                parts,
                num5, w - num6 - self.B,
                l - num5 - self.B, w,
                0, 0,
            );
            let sq8 = self.GetSqPartsForSnips(
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
                    Self::Resize_CSnip(&mut line.Snips[io], x, y + num6 + self.B, num5, w - num6 - self.B);
                    line.Snips.push(Self::Create_CSnip(x + num5 + self.B, y, l - num5 - self.B, w));
                }
                2 => {
                    Self::Resize_CSnip(&mut line.Snips[io], x, y + num6 + self.B, l, w - num6 - self.B);
                    line.Snips.push(Self::Create_CSnip(x + num5 + self.B, y, l - num5 - self.B, num6));
                }
                _ => {
                    Self::Resize_CSnip(&mut line.Snips[io], x, y + num6 + self.B, num5, w - num6 - self.B);
                    line.Snips.push(Self::Create_CSnip(x + num5 + self.B, y, l - num5 - self.B, w));
                }
            }
        }
    }

    fn Check_part_for_last_in_Line(
        &self,
        parts: &mut Vec<CPart>,
        LO: i32,
        WO: i32,
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
        let num5 = WO - num3 - self.B;
        let num6 = LO - num2 - self.B;
        let num7 = WO;

        let mut flag = false;
        let mut flag2 = false;
        if num4 >= self.minL && num5 >= self.minW {
            flag = Self::FastFindFirstPart(parts, num4, num5);
        }
        if num6 >= self.minL && num7 >= self.minW {
            flag2 = Self::FastFindFirstPart(parts, num6, num7);
        }

        if flag || flag2 {
            array[0] = id;
        } else {
            let num4b = LO - num2 - self.B;
            let num5b = num3;
            let num6b = LO;
            let num7b = WO - num3 - self.B;
            flag = false;
            flag2 = false;
            if num4b >= self.minL && num5b >= self.minW {
                flag = Self::FastFindFirstPart(parts, num4b, num5b);
            }
            if num6b >= self.minL && num7b >= self.minW {
                flag2 = Self::FastFindFirstPart(parts, num6b, num7b);
            }
            if flag || flag2 {
                array[0] = id;
            } else {
                let array2 = self.Find_2_Parts(parts, LO, WO);
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
                if (num8 - sq) as i64 > 0 {
                    array = [array2[0], array2[1]];
                }
            }
        }

        array
    }

    fn Create_CSnip(X: i32, Y: i32, length: i32, width: i32) -> CSnip {
        CSnip {
            L: length,
            W: width,
            CRD: Crd {
                X,
                Y,
                ..Default::default()
            },
            ..Default::default()
        }
    }

    fn Resize_CSnip(snip: &mut CSnip, X: i32, Y: i32, length: i32, width: i32) {
        snip.L = length;
        snip.W = width;
        snip.CRD.X = X;
        snip.CRD.Y = Y;
    }

    fn GetSqPartsForSnips(
        &self,
        parts: &mut Vec<CPart>,
        LO1: i32,
        WO1: i32,
        LO2: i32,
        WO2: i32,
        LO3: i32,
        WO3: i32,
    ) -> f64 {
        let mut total: f64 = 0.0;
        let mut num2: i32 = -1;
        let mut num3: i32 = -1;
        let mut _num4: i32 = -1;

        if LO1 >= self.minL && WO1 >= self.minW {
            num2 = Self::FindMaxSqPart(parts, LO1, WO1);
            if num2 != -1 {
                let idx = if num2 < -1 { num2 * -1 - 2 } else { num2 };
                total += parts[idx as usize].sq();
            }
        }
        if num2 != -1 {
            let idx = if num2 < -1 { num2 * -1 - 2 } else { num2 };
            parts[idx as usize].Plased += 1;
        }

        if LO2 >= self.minL && WO2 >= self.minW {
            num3 = Self::FindMaxSqPart(parts, LO2, WO2);
            if num3 != -1 {
                let idx = if num3 < -1 { num3 * -1 - 2 } else { num3 };
                total += parts[idx as usize].sq();
            }
        }
        if num3 != -1 {
            let idx = if num3 < -1 { num3 * -1 - 2 } else { num3 };
            parts[idx as usize].Plased += 1;
        }

        if LO3 >= self.minL && WO3 >= self.minW {
            _num4 = Self::FindMaxSqPart(parts, LO3, WO3);
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

    fn FindMaxSqPart(parts: &[CPart], LO: i32, WO: i32) -> i32 {
        let mut result: i32 = -1;
        let mut num: f64 = 0.0;

        if LO > 0 && WO > 0 {
            for i in 0..parts.len() {
                if parts[i].Plased >= parts[i].Qty {
                    continue;
                }
                if parts[i].L <= LO && parts[i].W <= WO {
                    if (parts[i].sq() - num) as i64 > 0 {
                        result = i as i32;
                        num = parts[i].sq();
                    }
                } else if parts[i].Turn
                    && parts[i].L <= WO
                    && parts[i].W <= LO
                    && (parts[i].sq() - num) as i64 > 0
                {
                    result = i as i32 * -1 - 2;
                    num = parts[i].sq();
                }
            }
        }
        result
    }

    fn FindMaxSqPart_krome(parts: &[CPart], LO: i32, WO: i32, krome: i32) -> i32 {
        let mut result: i32 = -1;
        let mut num: f64 = 0.0;

        let krome_idx = if krome < -1 { krome * -1 - 2 } else { krome };

        if LO > 0 && WO > 0 {
            for i in 0..parts.len() {
                let mut num2 = parts[i].Qty;
                if i as i32 == krome_idx {
                    num2 -= 1;
                }
                if parts[i].Plased >= num2 {
                    continue;
                }
                if parts[i].L <= LO && parts[i].W <= WO {
                    if (parts[i].sq() - num) as i64 > 0 {
                        result = i as i32;
                        num = parts[i].sq();
                    }
                } else if parts[i].Turn
                    && parts[i].L <= WO
                    && parts[i].W <= LO
                    && (parts[i].sq() - num) as i64 > 0
                {
                    result = i as i32 * -1 - 2;
                    num = parts[i].sq();
                }
            }
        }
        result
    }

    fn FindSmallSnip(&self, snips: &[CSnip], parts: &[CPart]) -> i32 {
        let mut result: i32 = -1;
        let mut num: f64 = 100000000000.0;

        for i in 0..snips.len() {
            if (num - snips[i].sq()) as i64 > 0
                && Self::FastFindFirstPart(parts, snips[i].L, snips[i].W)
            {
                num = snips[i].sq();
                result = i as i32;
            }
        }
        result
    }

    fn Find_2_Parts(&self, parts: &mut Vec<CPart>, LO: i32, WO: i32) -> [i32; 3] {
        let mut array = [-1i32, -1, 0]; // horizontal split
        let mut array2 = [-1i32, -1, 1]; // vertical split
        let mut num5: f64 = 0.0;
        let mut num6: f64 = 0.0;

        for i in 0..parts.len() {
            if parts[i].Plased >= parts[i].Qty {
                continue;
            }

            // Horizontal: split by L
            if LO >= parts[i].L && WO >= parts[i].W {
                let num = parts[i].sq();
                let num8 = LO - self.B - parts[i].L;
                let (num7, num2) = if num8 >= self.minL {
                    let f = Self::FindMaxSqPart_krome(parts, num8, WO, i as i32);
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
            if parts[i].Turn && WO >= parts[i].L && LO >= parts[i].W {
                let num = parts[i].sq();
                let num9 = LO - self.B - parts[i].W;
                let (num7, num2) = if num9 >= self.minL {
                    let f = Self::FindMaxSqPart_krome(parts, num9, WO, i as i32);
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
            if LO >= parts[i].L && WO >= parts[i].W {
                let num3 = parts[i].sq();
                let num10 = WO - self.B - parts[i].W;
                let (num7, num4) = if num10 >= self.minL {
                    let f = Self::FindMaxSqPart_krome(parts, LO, num10, i as i32);
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
            } else if parts[i].Turn && WO >= parts[i].L && LO >= parts[i].W {
                let num3 = parts[i].sq();
                let num11 = WO - self.B - parts[i].L;
                let (num7, num4) = if num11 >= self.minL {
                    let f = Self::FindMaxSqPart_krome(parts, LO, num11, i as i32);
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

    fn Find_Zamena_Lines_WIDTH_CUT_impl(
        Lines: &[CLine],
        WO: i32,
        Minimal_L: i32,
        B: i32,
        check: &mut bool,
    ) -> [i32; 3] {
        let mut array = [-1i32, -1, -1];
        *check = false;
        let mut num: i32 = 0;

        for i in 0..Lines.len() {
            let l = Lines[i].L;
            if Lines[i].onSheet || WO < l {
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
                        num2 += Lines[array[j] as usize].Parts_Sq;
                    }
                }
                if (Lines[i].Parts_Sq - num2) as i64 >= 0 {
                    array = [i as i32, -1, -1];
                    num = l;
                    *check = true;
                }
            }

            if WO - l - B - Minimal_L < 0 {
                continue;
            }
            for k in i + 1..Lines.len() {
                let l2 = Lines[k].L;
                if Lines[k].onSheet || WO < l2 {
                    continue;
                }
                if WO - l - B - l2 >= 0 {
                    if l + l2 - num > 0 {
                        array = [i as i32, k as i32, -1];
                        num = l + l2;
                        *check = true;
                    } else if l + l2 - num == 0 {
                        let mut num3: f64 = 0.0;
                        for m in 0..3 {
                            if array[m] != -1 {
                                num3 += Lines[array[m] as usize].Parts_Sq;
                            }
                        }
                        if (Lines[i].Parts_Sq + Lines[k].Parts_Sq - num3) as i64 >= 0 {
                            array = [i as i32, k as i32, -1];
                            num = l + l2;
                            *check = true;
                        }
                    }
                }
                if WO - l - B - l2 - B - Minimal_L < 0 {
                    continue;
                }
                for n in k + 1..Lines.len() {
                    let l3 = Lines[n].L;
                    if Lines[n].onSheet || WO < l3 || WO - l - B - l2 - B - l3 < 0 {
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
                                num4 += Lines[array[q] as usize].Parts_Sq;
                            }
                        }
                        if (Lines[i].Parts_Sq + Lines[k].Parts_Sq + Lines[n].Parts_Sq - num4) as i64 >= 0 {
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

    fn Find_Zamena_PARTS_WIDTH_CUT(
        Fix: &[i32],
        parts: &[CPart],
        WO: i32,
        max_W: i32,
        Minimal_W: i32,
        B: i32,
        check: &mut bool,
    ) -> [i32; 3] {
        let mut array = [-1i32, -1, -1];
        *check = false;
        let mut max_W = max_W;

        for i in 0..Fix.len() {
            let num = if Fix[i] <= -1 {
                parts[(Fix[i] * -1 - 2) as usize].L
            } else {
                parts[Fix[i] as usize].W
            };
            if WO < num {
                continue;
            }
            if num > max_W {
                array = [i as i32, -1, -1];
                max_W = num;
                *check = true;
            }
            if WO - num - B - Minimal_W < 0 {
                continue;
            }
            for j in i + 1..Fix.len() {
                let num2 = if Fix[j] <= -1 {
                    parts[(Fix[j] * -1 - 2) as usize].L
                } else {
                    parts[Fix[j] as usize].W
                };
                if WO - num - B - num2 >= 0 && num + num2 - max_W > 0 {
                    array = [i as i32, j as i32, -1];
                    max_W = num + num2;
                    *check = true;
                }
                if WO - num - B - num2 - B - Minimal_W < 0 {
                    continue;
                }
                for k in j + 1..Fix.len() {
                    let num3 = if Fix[k] <= -1 {
                        parts[(Fix[k] * -1 - 2) as usize].L
                    } else {
                        parts[Fix[k] as usize].W
                    };
                    if WO - num - B - num2 - B - num3 >= 0 && num + num2 + num3 - max_W > 0 {
                        array = [i as i32, j as i32, k as i32];
                        max_W = num + num2 + num3;
                        *check = true;
                    }
                }
            }
        }
        array
    }

    fn SET_ON_Parts_in_Line(parts: &mut Vec<CPart>, line: &CLine) {
        for i in 0..line.PartIDs.len() {
            if line.PartIDs[i] < -1 {
                parts[(line.PartIDs[i] * -1 - 2) as usize].Plased += 1;
            } else {
                parts[line.PartIDs[i] as usize].Plased += 1;
            }
        }
    }

    fn SET_OFF_Parts_in_Line(parts: &mut Vec<CPart>, line: &CLine) {
        for i in 0..line.PartIDs.len() {
            if line.PartIDs[i] < -1 {
                parts[(line.PartIDs[i] * -1 - 2) as usize].Plased -= 1;
            } else {
                parts[line.PartIDs[i] as usize].Plased -= 1;
            }
        }
    }

    fn Get_ID_LD_WD(parts: &[CPart], id: i32) -> (i32, i32, i32) {
        if id > -1 {
            (id, parts[id as usize].L, parts[id as usize].W)
        } else if id < -1 {
            let idx = id * -1 - 2;
            (idx, parts[idx as usize].W, parts[idx as usize].L)
        } else {
            (-1, -1, -1)
        }
    }

    fn Get_Parts_with_FixLength(
        &self,
        parts: &[CPart],
        L: i32,
        min_in: i32,
        TURN_ON: bool,
    ) -> (Vec<i32>, i32, i32) {
        let mut list: Vec<i32> = Vec::new();
        let mut Min_W = min_in;
        let mut Total_Length = 0;

        for i in 0..parts.len() {
            let cp = &parts[i];
            if cp.Qty <= cp.Plased {
                continue;
            }
            if !TURN_ON {
                if !cp.Turn {
                    if L == cp.L {
                        for _ in 0..(cp.Qty - cp.Plased) {
                            list.push(i as i32);
                            Total_Length += cp.W;
                        }
                        if Min_W > cp.W {
                            Min_W = cp.W;
                        }
                    }
                } else if cp.Turn {
                    let mut l_val: i32;
                    let mut flag = false;
                    if cp.L <= cp.W {
                        l_val = cp.L;
                        flag = false;
                    } else {
                        l_val = cp.W;
                        flag = true;
                    }
                    if l_val == L {
                        for _ in 0..(cp.Qty - cp.Plased) {
                            if flag {
                                list.push(i as i32 * -1 - 2);
                                Total_Length += cp.L;
                            } else {
                                list.push(i as i32);
                                Total_Length += cp.W;
                            }
                        }
                        if flag {
                            if Min_W > cp.L {
                                Min_W = cp.L;
                            }
                        } else if Min_W > cp.W {
                            Min_W = cp.W;
                        }
                    }
                }
            } else if cp.L == L {
                for _ in 0..(cp.Qty - cp.Plased) {
                    list.push(i as i32);
                    Total_Length += cp.W;
                }
                if Min_W > cp.W {
                    Min_W = cp.W;
                }
            } else if cp.Turn && cp.W == L {
                for _ in 0..(cp.Qty - cp.Plased) {
                    list.push(i as i32 * -1 - 2);
                    Total_Length += cp.L;
                }
                if Min_W > cp.L {
                    Min_W = cp.L;
                }
            }
            if list.len() as i32 > self.THE_SAME_PARTS_LIMIT {
                break;
            }
        }

        (list, Min_W, Total_Length)
    }

    fn GetStartParts_for_Line_WIDTH_CUT(
        &self,
        parts: &[CPart],
        mut fix_length: Vec<i32>,
        LineWidth: i32,
        Minimal_W: i32,
    ) -> Vec<i32> {
        let mut list: Vec<i32> = Vec::new();
        let mut num = LineWidth;

        let mut i = 0;
        while i < fix_length.len() {
            let (_ID, _LD, WD) = Self::Get_ID_LD_WD(parts, fix_length[i]);
            if num >= WD {
                num = num - WD - self.B;
                list.push(fix_length[i]);
                fix_length.remove(i);
                if num < Minimal_W {
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

                        let num8 = if list[j] <= -1 {
                            parts[(list[j] * -1 - 2) as usize].L
                        } else {
                            parts[list[j] as usize].W
                        };
                        let num9 = if list[k] <= -1 {
                            parts[(list[k] * -1 - 2) as usize].L
                        } else {
                            parts[list[k] as usize].W
                        };
                        let wo = num + self.B + num8 + self.B + num9;

                        let mut check = false;
                        let array2 = Self::Find_Zamena_PARTS_WIDTH_CUT(
                            &fix_length,
                            parts,
                            wo,
                            num8 + num9,
                            Minimal_W,
                            self.B,
                            &mut check,
                        );

                        if check {
                            let mut num10: i32 = 0;
                            for idx in 0..array2.len() {
                                if array2[idx] != -1 {
                                    let (_, _, wd4) =
                                        Self::Get_ID_LD_WD(parts, fix_length[array2[idx] as usize]);
                                    num10 = num10 + self.B + wd4;
                                }
                            }
                            if num10 > num3 {
                                num4 = j as i32;
                                num5 = k as i32;
                                num6 = num8;
                                num7 = num9;
                                best_arr = Some(array2);
                                num3 = num10;
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
                            Self::Get_ID_LD_WD(parts, fix_length[array[m] as usize]);
                        num = num - self.B - wd5;
                    }
                }

                let mut num11 = 0;
                for n in 0..array.len() {
                    if array[n] != -1 {
                        fix_length.remove((array[n] - num11) as usize);
                        num11 += 1;
                    }
                }
            } else {
                flag = true;
            }
        }

        // Sort by WD descending
        for num12 in 0..list.len().saturating_sub(1) {
            for num13 in num12 + 1..list.len() {
                let (_, _, WD4) = Self::Get_ID_LD_WD(parts, list[num12]);
                let (_, _, WD5) = Self::Get_ID_LD_WD(parts, list[num13]);
                if WD5 > WD4 {
                    list.swap(num12, num13);
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
