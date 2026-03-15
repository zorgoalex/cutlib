#![allow(non_snake_case)]

/// Algorithm computation types matching .NET LibCut.Core.Internal
/// All dimensions in these types are x10 (tenths of mm) for subpixel precision.

#[derive(Debug, Clone)]
pub struct CPart {
    pub L: i32,
    pub W: i32,
    pub Qty: i32,
    pub Plased: i32,
    pub Turn: bool,
    pub iD_in_Order: i32,
}

impl CPart {
    pub fn sq(&self) -> f64 {
        self.L as f64 * self.W as f64
    }
}

impl Default for CPart {
    fn default() -> Self {
        Self {
            L: 0,
            W: 0,
            Qty: 0,
            Plased: 0,
            Turn: false,
            iD_in_Order: -1,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Crd {
    pub X: i32,
    pub Y: i32,
    pub id_in_order: i32,
}

impl Default for Crd {
    fn default() -> Self {
        Self {
            X: 0,
            Y: 0,
            id_in_order: -8,
        }
    }
}

#[derive(Debug, Clone)]
pub struct CSnip {
    pub L: i32,
    pub W: i32,
    pub Filled: bool,
    pub CRD: Crd,
}

impl CSnip {
    pub fn sq(&self) -> f64 {
        self.L as f64 * self.W as f64
    }
}

impl Default for CSnip {
    fn default() -> Self {
        Self {
            L: 0,
            W: 0,
            Filled: false,
            CRD: Crd::default(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct CLine {
    pub L: i32,
    pub W: i32,
    pub PartIDs: Vec<i32>,
    pub Parts_Crds: Vec<Crd>,
    pub Snips: Vec<CSnip>,
    pub Parts_Sq: f64,
    pub crd: Crd,
    pub onSheet: bool,
}

impl CLine {
    pub fn sq(&self) -> f64 {
        self.L as f64 * self.W as f64
    }

    #[allow(dead_code)]
    pub fn filling(&self) -> f32 {
        let s = self.sq();
        if s == 0.0 {
            0.0
        } else {
            (self.Parts_Sq / s) as f32
        }
    }
}

impl Default for CLine {
    fn default() -> Self {
        Self {
            L: 0,
            W: 0,
            PartIDs: Vec::new(),
            Parts_Crds: Vec::new(),
            Snips: Vec::new(),
            Parts_Sq: 0.0,
            crd: Crd::default(),
            onSheet: false,
        }
    }
}

#[derive(Debug, Clone)]
pub struct CSheet {
    pub L: i32,
    pub W: i32,
    pub Lines: Vec<CLine>,
    pub Remain: CSnip,
    pub Parts_Sq: f64,
    pub Alg: i32,
    pub Filled: bool,
}

impl CSheet {
    pub fn sq(&self) -> f64 {
        self.L as f64 * self.W as f64
    }

    pub fn filling(&self) -> f32 {
        let s = self.sq();
        if s == 0.0 {
            0.0
        } else {
            (self.Parts_Sq / s) as f32
        }
    }
}

impl Default for CSheet {
    fn default() -> Self {
        Self {
            L: 0,
            W: 0,
            Lines: Vec::new(),
            Remain: CSnip::default(),
            Parts_Sq: 0.0,
            Alg: 0,
            Filled: false,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct LW16 {
    pub SAME_MAX: bool,
    pub MAX_SQ: bool,
    pub OPTI_ON: bool,
    pub TURN_ON: bool,
}

impl LW16 {
    pub fn all_variants() -> [LW16; 8] {
        [
            LW16 { SAME_MAX: true,  MAX_SQ: true,  OPTI_ON: true,  TURN_ON: true },
            LW16 { SAME_MAX: true,  MAX_SQ: true,  OPTI_ON: true,  TURN_ON: false },
            LW16 { SAME_MAX: true,  MAX_SQ: true,  OPTI_ON: false, TURN_ON: true },
            LW16 { SAME_MAX: true,  MAX_SQ: true,  OPTI_ON: false, TURN_ON: false },
            LW16 { SAME_MAX: true,  MAX_SQ: false, OPTI_ON: true,  TURN_ON: true },
            LW16 { SAME_MAX: true,  MAX_SQ: false, OPTI_ON: true,  TURN_ON: false },
            LW16 { SAME_MAX: true,  MAX_SQ: false, OPTI_ON: false, TURN_ON: true },
            LW16 { SAME_MAX: true,  MAX_SQ: false, OPTI_ON: false, TURN_ON: false },
        ]
    }
}

#[derive(Debug, Clone, Default)]
pub struct Remain {
    pub X: i32,
    pub Y: i32,
    pub L: i32,
    pub W: i32,
    pub sheet_index: i32,
    pub nlist: i32,
    pub Sq: i64,
}

/// Cut parameters for algorithm variant selection (CutPar in .NET)
#[derive(Debug, Clone, Default)]
pub struct CutPar {
    pub AlgType: i32,
    pub DoublePadding: bool,
}
