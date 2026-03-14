# LibCut CLI — Reference

## Overview

Command-line utility for 2D guillotine cutting optimization of sheet materials.
Takes a list of rectangular parts and arranges them on sheets of given size
with minimal waste, using guillotine (straight through) cuts.

## Synopsis

```bash
LibCutCLI -i <input_file> [options]
LibCutCLI --help
```

## Options

| Flag | Long form | Type | Default | Description |
|------|-----------|------|---------|-------------|
| `-i` | `--input` | string | *required* | Path to input file (CSV or JSON) |
| `-o` | `--output` | string | stdout | Path to output file. If omitted, results go to stdout |
| `-s` | `--sheet` | string | — | Sheet dimensions in mm: `LxW`, `L*W`, or `LXW` (e.g. `2440x1220`) |
| `-b` | `--blade` | int | `4` | Blade/kerf width in mm |
| `-p` | `--padding` | int | `0` | Edge padding (margin from sheet edges) in mm |
| `-a` | `--algorithm` | string | `optimal` | Algorithm to use (see below) |
| `-f` | `--format` | string | `text` | Output format: `text` or `json` |
| `-h` | `--help` | — | — | Show usage info and exit |

## Algorithms

| Value | Aliases | Description |
|-------|---------|-------------|
| `length` | `l`, `1` | Primary cuts along sheet length. Best for long narrow parts |
| `width` | `w`, `2` | Primary cuts along sheet width. Best for wide short parts |
| `optimal` | `opt`, `3` | Tries both Length and Width strategies plus a combined approach, picks the layout with highest material utilization. **Recommended for most cases** |

Each algorithm internally runs 9 parallel variants with different optimization
parameters (same-size grouping, max square packing, rotation, optimization passes)
and selects the best result. The `optimal` algorithm additionally runs a combined
Length+Width strategy for a total of up to 27 variants per sheet.

## Input Formats

### CSV

Separator: semicolon (`;`), comma (`,`) or tab. Lines starting with `#` are comments.

```
# length;width;qty;rotate;name
800;400;5;1;Panel A
600;300;8;1;Panel B
500;250;4;0;Shelf
1200;600;2;1;Door
```

| Column | Required | Description |
|--------|----------|-------------|
| 1 — length | yes | Part length in mm |
| 2 — width | yes | Part width in mm |
| 3 — qty | yes | Number of pieces |
| 4 — rotate | no | `1` = allow rotation 90°, `0` = fixed orientation. Default: `0` |
| 5 — name | no | Part label for output |

When using CSV, sheet dimensions **must** be provided via `-s` flag.

### JSON

```json
{
  "sheet": { "length": 2440, "width": 1220 },
  "blade": 4,
  "padding": 10,
  "algorithm": "optimal",
  "parts": [
    { "length": 800, "width": 400, "qty": 5, "rotate": true, "name": "Panel A" },
    { "length": 600, "width": 300, "qty": 8, "rotate": true, "name": "Panel B" },
    { "length": 500, "width": 250, "qty": 4, "rotate": false, "name": "Shelf" },
    { "length": 1200, "width": 600, "qty": 2, "rotate": true, "name": "Door" }
  ]
}
```

| Field | Required | Description |
|-------|----------|-------------|
| `sheet.length` | yes* | Sheet length in mm |
| `sheet.width` | yes* | Sheet width in mm |
| `blade` | no | Blade width in mm (overrides `-b`) |
| `padding` | no | Edge padding in mm (overrides `-p`) |
| `algorithm` | no | Algorithm name (overrides `-a`) |
| `parts[].length` | yes | Part length in mm |
| `parts[].width` | yes | Part width in mm |
| `parts[].qty` | no | Quantity, default `1` |
| `parts[].rotate` | no | Allow rotation, default `true` |
| `parts[].name` | no | Part label |

\* Sheet size from JSON can be overridden by the `-s` flag. If neither is provided, the program exits with an error.

**Priority**: CLI flags override JSON values for `blade`, `padding`, `algorithm`. For sheet size, the `-s` flag takes precedence over JSON `sheet` only if `-s` is provided.

## Output Formats

### Text (default)

```
=== CUTTING RESULTS ===
Sheet: 2440 x 1220 mm
Sheets used: 2
Parts placed: 19 / 19
Material efficiency: 83.6%

--- Parts placement ---
Panel A: 800x400 mm, placed 5/5
    Sheet 1: (10, 10)
    Sheet 1: (10, 414)
    Sheet 1: (10, 818)
    Sheet 1: (814, 10)
    Sheet 1: (814, 414)
Panel B: 600x300 mm, placed 8/8
    Sheet 1: (814, 818)
    Sheet 1: (1418, 10)
    ...
Shelf: 500x250 mm, placed 4/4
    Sheet 2: (10, 10)
    ...
Door: 1200x600 mm, placed 2/2
    Sheet 2: (10, 264) [rotated]
    Sheet 2: (614, 264)

--- Waste/offcuts ---
  Sheet 1: 200x400 mm at (1618, 818)
  Sheet 2: 1020x600 mm at (1420, 10)
```

### JSON (`-f json`)

```json
{
  "sheetSize": { "length": 2440, "width": 1220 },
  "sheetsUsed": 2,
  "partsPlaced": 19,
  "partsTotal": 19,
  "efficiencyPercent": 83.6,
  "sheets": [
    {
      "sheet": 1,
      "parts": [
        { "name": "Panel A", "length": 800, "width": 400, "x": 10, "y": 10, "rotated": false },
        ...
      ],
      "offcuts": [
        { "length": 200, "width": 400, "x": 1618, "y": 818 }
      ]
    },
    {
      "sheet": 2,
      "parts": [ ... ],
      "offcuts": [ ... ]
    }
  ]
}
```

| Field | Type | Description |
|-------|------|-------------|
| `sheetSize` | object | Sheet dimensions used |
| `sheetsUsed` | int | Total number of sheets |
| `partsPlaced` | int | Number of parts successfully placed |
| `partsTotal` | int | Total parts requested |
| `efficiencyPercent` | float | Material utilization as percentage |
| `sheets[]` | array | Per-sheet breakdown |
| `sheets[].parts[]` | array | Parts placed on this sheet |
| `sheets[].parts[].x`, `.y` | int | Placement coordinates (mm, top-left corner of part) |
| `sheets[].parts[].rotated` | bool | Whether the part was rotated 90° |
| `sheets[].offcuts[]` | array | Waste rectangles on this sheet |

## Error Handling

All informational and error messages go to **stderr**. Results go to **stdout** (or to `-o` file).

| Condition | stderr message | Exit behavior |
|-----------|----------------|---------------|
| No arguments | — | Prints usage and exits with code `0` |
| Missing `-i` | `Error: Input file is required (-i).` | Exits with code `1` |
| No sheet size (no `-s` and no JSON `sheet`) | Validation errors for `sheet.length` and `sheet.width` | Exits with code `1` |
| Input file not found | `Error: input file not found: ...` | Exits with code `1` |
| Invalid CSV/JSON format | `Error: Invalid CSV row...` or `Error: Input JSON is invalid.` | Exits with code `1` |

On successful run, stderr shows a summary:
```
Sheet: 2440x1220 mm
Blade: 4 mm, Padding: 0 mm
Algorithm: Optimal
Parts: 4 types, 19 total pieces
```

## Usage Examples

**Basic — CSV input, text output to terminal:**
```bash
LibCutCLI -i parts.csv -s 2440x1220
```

**JSON input with all parameters in file:**
```bash
LibCutCLI -i order.json
```

**Save JSON output to file:**
```bash
LibCutCLI -i parts.csv -s 2440x1220 -f json -o result.json
```

**Custom blade and padding:**
```bash
LibCutCLI -i parts.csv -s 2440x1220 -b 3 -p 15
```

**Force length-first algorithm:**
```bash
LibCutCLI -i parts.csv -s 2440x1220 -a length
```

**Pipe JSON results for further processing:**
```bash
LibCutCLI -i parts.csv -s 2440x1220 -f json 2>/dev/null | jq '.efficiencyPercent'
```

**Override JSON file parameters from CLI:**
```bash
LibCutCLI -i order.json -s 3000x1500 -b 5 -a width
```

## Coordinate System

- Origin `(0, 0)` is the top-left corner of the sheet
- `x` increases to the right (along sheet length)
- `y` increases downward (along sheet width)
- Coordinates represent the top-left corner of the placed part
- When `rotated = true`, the part's length and width are swapped during placement
- All input/output values are in **millimeters**
- Internally the engine operates in tenths of mm (×10) for precision

## Build

```bash
./scripts/dotnet.sh publish src/LibCut.Cli/LibCut.Cli.csproj -c Release -r linux-x64 --self-contained false -o artifacts/linux-cli
```

Output: `artifacts/linux-cli/LibCutCLI` (framework-dependent Linux build).
