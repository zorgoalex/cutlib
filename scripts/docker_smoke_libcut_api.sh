#!/usr/bin/env bash
set -euo pipefail

IMAGE_NAME="${IMAGE_NAME:-libcut-api:local}"
CONTAINER_NAME="${CONTAINER_NAME:-libcut_api_smoke}"
PORT="${PORT:-18080}"
REQUEST_FILE="${REQUEST_FILE:-/home/dmina/apps/cutcli/repo_cutcli/artifacts/sample_order.json}"
CURL_IMAGE="${CURL_IMAGE:-curlimages/curl:8.6.0}"
REQUEST_DIR="$(dirname "$REQUEST_FILE")"
REQUEST_BASENAME="$(basename "$REQUEST_FILE")"

cleanup() {
  docker rm -f "$CONTAINER_NAME" >/dev/null 2>&1 || true
}

trap cleanup EXIT

cleanup

docker build -f /home/dmina/apps/cutcli/repo_cutcli/Dockerfile.libcut-api -t "$IMAGE_NAME" /home/dmina/apps/cutcli/repo_cutcli
docker run -d --name "$CONTAINER_NAME" -p "${PORT}:8080" "$IMAGE_NAME" >/dev/null

for _ in $(seq 1 30); do
  if docker run --rm --network "container:${CONTAINER_NAME}" "$CURL_IMAGE" \
    -fsS "http://127.0.0.1:8080/health" >/dev/null 2>&1; then
    break
  fi
  sleep 1
done

echo "== health =="
docker run --rm --network "container:${CONTAINER_NAME}" "$CURL_IMAGE" \
  -fsS "http://127.0.0.1:8080/health"
echo
echo "== openapi =="
OPENAPI_DOC="$(
  docker run --rm --network "container:${CONTAINER_NAME}" "$CURL_IMAGE" \
    -fsS "http://127.0.0.1:8080/openapi/v1.json"
)"
printf '%s' "$OPENAPI_DOC" | grep -q '"/api/cut/optimize"'
echo "OpenAPI document is available"
echo
echo "== optimize =="
docker run --rm --network "container:${CONTAINER_NAME}" \
  -v "${REQUEST_DIR}:/data:ro" \
  "$CURL_IMAGE" \
  -fsS "http://127.0.0.1:8080/api/cut/optimize" \
  -H 'Content-Type: application/json' \
  --data-binary "@/data/${REQUEST_BASENAME}"
echo
