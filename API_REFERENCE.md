# LibCut API Reference

## Base URL

```
http://localhost:8080
```

## Authentication

The API is protected by a **Bearer token** at the nginx reverse proxy level.

Include the `Authorization` header in all requests to `/api/*`:

```
Authorization: Bearer <your-token>
```

The `/health` endpoint does not require authentication.

### Responses

| Status | When |
|---|---|
| **401 Unauthorized** | `Authorization` header missing |
| **403 Forbidden** | Token invalid |

```json
{
  "type": "about:blank",
  "title": "Unauthorized",
  "status": 401,
  "detail": "Missing Authorization header."
}
```

See [DEPLOYMENT.md](DEPLOYMENT.md) for token generation and nginx configuration.

---

## Endpoints

| Method | Path | Description |
|---|---|---|
| `GET` | `/health` | Health check |
| `POST` | `/api/cut/optimize` | Run cutting optimization |

---

## GET /health

Returns the service health status.

### Request

No parameters.

### Response

**200 OK**

```json
{
  "status": "ok",
  "service": "LibCut.Api"
}
```

---

## POST /api/cut/optimize

Accepts a cutting order and returns the optimal placement layout.

### Request

**Content-Type:** `application/json`

```json
{
  "sheet": {
    "length": 2440,
    "width": 1220
  },
  "blade": 4,
  "padding": 10,
  "algorithm": "optimal",
  "parts": [
    {
      "name": "Panel A",
      "length": 800,
      "width": 400,
      "qty": 5,
      "rotate": true
    }
  ]
}
```

### Request Fields

#### Top Level

| Field | Type | Required | Default | Description |
|---|---|---|---|---|
| `sheet` | object | **yes** | — | Sheet (stock material) dimensions |
| `parts` | array | **yes** | — | List of parts to cut (min 1) |
| `blade` | integer | no | `4` | Blade (kerf) width in mm |
| `padding` | integer | no | `0` | Padding around each part in mm |
| `algorithm` | string | no | `"optimal"` | Optimization algorithm |
| `options` | object | no | — | Alternative location for blade/padding/algorithm |

> **Option resolution:** Top-level fields take priority over `options.*` fields. For example, if both `blade` and `options.blade` are provided, the top-level `blade` is used.

#### `sheet`

| Field | Type | Required | Description |
|---|---|---|---|
| `length` | integer | **yes** | Sheet length in mm (must be > 0) |
| `width` | integer | **yes** | Sheet width in mm (must be > 0) |

Aliases: `lengthMm`, `widthMm` (case-insensitive).

#### `parts[]`

| Field | Type | Required | Default | Description |
|---|---|---|---|---|
| `name` | string | no | `""` | Part label for identification |
| `length` | integer | **yes** | — | Part length in mm (must be > 0) |
| `width` | integer | **yes** | — | Part width in mm (must be > 0) |
| `qty` | integer | no | `1` | Quantity needed (must be > 0) |
| `rotate` | boolean | no | `true` | Whether the part may be rotated 90° |

Aliases: `qty`/`quantity`, `rotate`/`canRotate`/`can_rotate` (case-insensitive).

#### `algorithm`

| Value | Aliases | Description |
|---|---|---|
| `"optimal"` | `"opt"`, `"3"` | **(default)** Runs both Length and Width strategies, picks the best result. Up to 27 internal variants. |
| `"length"` | `"l"`, `"1"` | Primary cuts along sheet length. Best for long narrow parts. |
| `"width"` | `"w"`, `"2"` | Primary cuts along sheet width. Best for wide short parts. |

Algorithm values are case-insensitive.

---

### Response

**200 OK** — `Content-Type: application/json`

```json
{
  "sheetSize": {
    "length": 2440,
    "width": 1220
  },
  "sheetsUsed": 2,
  "partsPlaced": 19,
  "partsTotal": 19,
  "efficiencyPercent": 83.6,
  "sheets": [
    {
      "sheet": 1,
      "parts": [
        {
          "name": "Panel A",
          "length": 800,
          "width": 400,
          "x": 0,
          "y": 0,
          "rotated": false
        }
      ],
      "offcuts": [
        {
          "length": 200,
          "width": 300,
          "x": 1200,
          "y": 0
        }
      ]
    }
  ]
}
```

### Response Fields

#### Top Level

| Field | Type | Description |
|---|---|---|
| `sheetSize` | object | Sheet dimensions used |
| `sheetSize.length` | integer | Sheet length in mm |
| `sheetSize.width` | integer | Sheet width in mm |
| `sheetsUsed` | integer | Number of sheets consumed |
| `partsPlaced` | integer | Number of parts successfully placed |
| `partsTotal` | integer | Total parts requested |
| `efficiencyPercent` | float | Material utilization (0–100%) |
| `sheets` | array | Per-sheet placement details |

#### `sheets[]`

| Field | Type | Description |
|---|---|---|
| `sheet` | integer | Sheet number (1-indexed) |
| `parts` | array | Parts placed on this sheet |
| `offcuts` | array | Waste/remainder areas on this sheet |

#### `sheets[].parts[]`

| Field | Type | Description |
|---|---|---|
| `name` | string | Part label |
| `length` | integer | Part length in mm (as placed) |
| `width` | integer | Part width in mm (as placed) |
| `x` | integer | X position from left edge in mm |
| `y` | integer | Y position from top edge in mm |
| `rotated` | boolean | Whether the part was rotated 90° |

#### `sheets[].offcuts[]`

| Field | Type | Description |
|---|---|---|
| `length` | integer | Offcut length in mm |
| `width` | integer | Offcut width in mm |
| `x` | integer | X position from left edge in mm |
| `y` | integer | Y position from top edge in mm |

**Coordinate system:** Origin (0, 0) is the top-left corner of the sheet. X increases rightward, Y increases downward. All values are in millimeters.

---

### Error Responses

All errors use the **Problem Details** format ([RFC 9457](https://www.rfc-editor.org/rfc/rfc9457)).

**Content-Type:** `application/problem+json`

#### 400 Bad Request — Validation Error

```json
{
  "type": "https://datatracker.ietf.org/doc/html/rfc9110#section-15.5.1",
  "title": "Invalid cut optimization request.",
  "status": 400,
  "detail": "Correct the fields listed in the errors section and retry the request.",
  "errors": {
    "sheet": ["Sheet is required."],
    "parts[0].length": ["Part length must be greater than zero."]
  },
  "traceId": "a1b2c3d4-e5f6-7890-abcd-ef1234567890"
}
```

#### 400 Bad Request — Malformed JSON

```json
{
  "type": "https://datatracker.ietf.org/doc/html/rfc9110#section-15.5.1",
  "title": "Invalid cut optimization request.",
  "status": 400,
  "detail": "Correct the fields listed in the errors section and retry the request.",
  "errors": {
    "body": ["Malformed JSON: expected value at line 1 column 1"]
  },
  "traceId": "a1b2c3d4-e5f6-7890-abcd-ef1234567890"
}
```

#### 400 Bad Request — Empty Body

```json
{
  "type": "https://datatracker.ietf.org/doc/html/rfc9110#section-15.5.1",
  "title": "Invalid cut optimization request.",
  "status": 400,
  "detail": "Correct the fields listed in the errors section and retry the request.",
  "errors": {
    "body": ["Request body is empty."]
  },
  "traceId": "a1b2c3d4-e5f6-7890-abcd-ef1234567890"
}
```

#### 500 Internal Server Error

```json
{
  "type": "https://datatracker.ietf.org/doc/html/rfc9110#section-15.6.1",
  "title": "Cut optimization failed.",
  "status": 500,
  "detail": "Unexpected server error while processing the optimization request.",
  "traceId": "a1b2c3d4-e5f6-7890-abcd-ef1234567890"
}
```

### Validation Rules

| Field | Rule | Error message |
|---|---|---|
| `sheet` | Required | Sheet is required. |
| `sheet.length` | > 0 | Sheet length must be greater than zero. |
| `sheet.width` | > 0 | Sheet width must be greater than zero. |
| `parts` | At least 1 element | At least one part is required. |
| `parts[].length` | > 0 | Part length must be greater than zero. |
| `parts[].width` | > 0 | Part width must be greater than zero. |
| `parts[].qty` | > 0 | Part quantity must be greater than zero. |
| `blade` | >= 0 | Blade width must not be negative. |
| `padding` | >= 0 | Padding must not be negative. |
| `algorithm` | Valid value | Unknown algorithm '{value}'. Allowed values: length, width, optimal. |

All validation errors are collected (not fail-fast) and returned together.

---

## Examples

### Minimal Request

```bash
curl -X POST http://localhost:8080/api/cut/optimize \
  -H "Content-Type: application/json" \
  -d '{
    "sheet": { "length": 2440, "width": 1220 },
    "parts": [
      { "length": 800, "width": 400, "qty": 3 }
    ]
  }'
```

### Full Request with All Options

```bash
curl -X POST http://localhost:8080/api/cut/optimize \
  -H "Content-Type: application/json" \
  -d '{
    "sheet": { "length": 2440, "width": 1220 },
    "blade": 4,
    "padding": 10,
    "algorithm": "optimal",
    "parts": [
      { "name": "Panel A",  "length": 800,  "width": 400, "qty": 5, "rotate": true },
      { "name": "Panel B",  "length": 600,  "width": 300, "qty": 8, "rotate": true },
      { "name": "Shelf",    "length": 500,  "width": 250, "qty": 4, "rotate": false },
      { "name": "Door",     "length": 1200, "width": 600, "qty": 2, "rotate": true }
    ]
  }'
```

### Non-rotatable Parts

```bash
curl -X POST http://localhost:8080/api/cut/optimize \
  -H "Content-Type: application/json" \
  -d '{
    "sheet": { "length": 2440, "width": 1220 },
    "blade": 3,
    "parts": [
      { "name": "Top",  "length": 1800, "width": 600, "qty": 1, "rotate": false },
      { "name": "Side", "length": 700,  "width": 400, "qty": 2, "rotate": false }
    ]
  }'
```
