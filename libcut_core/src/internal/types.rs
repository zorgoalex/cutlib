#![allow(non_snake_case)]

/// Internal domain types matching .NET LibCut.Core.Internal

#[derive(Debug, Clone, Default)]
pub struct Sheet {
    pub Length: i32,
    pub Width: i32,
}

#[derive(Debug, Clone, Default)]
pub struct Parameters {
    pub Algoritm: i32,         // 1=Length, 2=Width, 3=Optimal
    pub ListLength_mm: i32,
    pub ListWidth_mm: i32,
    pub Padding: i32,
    pub Blade: i32,
    pub Units: i32,            // 1=mm
    pub StartPoint: i32,       // 1=top-left
}

#[derive(Debug, Clone)]
pub struct Coord {
    pub X: i32,
    pub Y: i32,
    pub list: i32,      // sheet number (1-based, -1 if unplaced)
    pub nlist: i32,
    pub Cutted: bool,
    pub isTurn: bool,
    pub onList: bool,
}

impl Default for Coord {
    fn default() -> Self {
        Self {
            X: 0,
            Y: 0,
            list: -1,
            nlist: -1,
            Cutted: false,
            isTurn: false,
            onList: false,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Part {
    pub Npart: i32,
    pub Name: String,
    pub Length_mm: i32,
    pub Width_mm: i32,
    pub Amount: i32,
    pub Sq: i64,
    pub Turn: bool,
    pub nPlaced: i32,
    pub Coords: Vec<Coord>,
    // Edge/slot fields (unused in core algorithm, kept for mapping compatibility)
    pub ELength: i32,
    pub EWidth: i32,
}

impl Default for Part {
    fn default() -> Self {
        Self {
            Npart: 0,
            Name: String::new(),
            Length_mm: 0,
            Width_mm: 0,
            Amount: 0,
            Sq: 0,
            Turn: true,
            nPlaced: 0,
            Coords: Vec::new(),
            ELength: 0,
            EWidth: 0,
        }
    }
}

#[derive(Debug, Clone)]
pub struct SCoord {
    pub X: i32,
    pub Y: i32,
    pub list: i32,
    pub nlist: i32,
    pub onList: bool,
}

impl Default for SCoord {
    fn default() -> Self {
        Self {
            X: 0,
            Y: 0,
            list: -1,
            nlist: -1,
            onList: false,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Snip {
    pub Length_mm: i32,
    pub Width_mm: i32,
    pub Amount: i32,
    pub Sq: i64,
    pub X: i32,
    pub Y: i32,
    pub list: i32,
    pub nlist: i32,
    pub onList: bool,
    pub nCutted: i32,
    pub SCoords: Vec<SCoord>,
}

impl Default for Snip {
    fn default() -> Self {
        Self {
            Length_mm: 0,
            Width_mm: 0,
            Amount: 0,
            Sq: 0,
            X: 0,
            Y: 0,
            list: -1,
            nlist: -1,
            onList: false,
            nCutted: 0,
            SCoords: Vec::new(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Order {
    pub sheet: Sheet,
    pub Parts: Vec<Part>,
    pub Snips: Vec<Snip>,
    pub NSnips: Vec<Snip>,
    pub parameters: Parameters,
    pub SheetCount: i32,
    pub PartsPlaced: i32,
    pub PartsSq: i64,
}

impl Default for Order {
    fn default() -> Self {
        Self {
            sheet: Sheet::default(),
            Parts: Vec::new(),
            Snips: Vec::new(),
            NSnips: Vec::new(),
            parameters: Parameters::default(),
            SheetCount: 0,
            PartsPlaced: 0,
            PartsSq: 0,
        }
    }
}
