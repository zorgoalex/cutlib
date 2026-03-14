#!/usr/bin/env bash
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
REPO_ROOT="$(cd "$SCRIPT_DIR/.." && pwd)"
WORKSPACE_ROOT="$(cd "$REPO_ROOT/.." && pwd)"

export DOTNET_ROOT="${DOTNET_ROOT:-$WORKSPACE_ROOT/.dotnet}"
export DOTNET_CLI_HOME="${DOTNET_CLI_HOME:-$WORKSPACE_ROOT/.dotnet_cli}"
export HOME="$DOTNET_CLI_HOME"
export NUGET_PACKAGES="${NUGET_PACKAGES:-$WORKSPACE_ROOT/.nuget/packages}"
export DOTNET_SKIP_FIRST_TIME_EXPERIENCE=1
export DOTNET_NOLOGO=1
export MSBuildEnableWorkloadResolver=false

exec "$DOTNET_ROOT/dotnet" "$@"
