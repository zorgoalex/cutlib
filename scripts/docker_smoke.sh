#!/bin/bash
set -e

IMAGE="libcut-api"
CONTAINER="libcut-api-smoke"
NETWORK="libcut-smoke-net"
PORT=8080

cleanup() {
  docker rm -f "$CONTAINER" 2>/dev/null || true
  docker network rm "$NETWORK" 2>/dev/null || true
}
trap cleanup EXIT

echo "Building Docker image..."
docker build -t "$IMAGE" .

echo "Creating network..."
docker network create "$NETWORK"

echo "Starting container..."
docker run -d --name "$CONTAINER" --network "$NETWORK" "$IMAGE"
sleep 2

echo "Testing /health..."
HEALTH=$(docker run --rm --network "$NETWORK" curlimages/curl \
  -s "http://${CONTAINER}:${PORT}/health")
echo "$HEALTH"

echo "Testing /api/cut/optimize..."
RESULT=$(docker run --rm --network "$NETWORK" curlimages/curl \
  -s -X POST "http://${CONTAINER}:${PORT}/api/cut/optimize" \
  -H "Content-Type: application/json" \
  -d '{
    "sheet": { "length": 2440, "width": 1220 },
    "blade": 4, "padding": 10, "algorithm": "optimal",
    "parts": [
      { "length": 800, "width": 400, "qty": 5, "rotate": true, "name": "Panel A" },
      { "length": 600, "width": 300, "qty": 8, "rotate": true, "name": "Panel B" },
      { "length": 500, "width": 250, "qty": 4, "rotate": false, "name": "Shelf" },
      { "length": 1200, "width": 600, "qty": 2, "rotate": true, "name": "Door" }
    ]
  }')

SHEETS=$(echo "$RESULT" | python3 -c "import json,sys; print(json.load(sys.stdin)['sheetsUsed'])" 2>/dev/null || echo "FAIL")
PARTS=$(echo "$RESULT" | python3 -c "import json,sys; print(json.load(sys.stdin)['partsPlaced'])" 2>/dev/null || echo "FAIL")

echo "Sheets: $SHEETS, Parts: $PARTS"

if [ "$SHEETS" = "2" ] && [ "$PARTS" = "19" ]; then
  echo "SMOKE TEST PASSED"
  exit 0
else
  echo "SMOKE TEST FAILED"
  exit 1
fi
