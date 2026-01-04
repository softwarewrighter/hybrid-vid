#!/usr/bin/env bash
set -euo pipefail
ROOT="$(dirname "$0")/.."
"$ROOT/scripts/build-processing.sh"
"$ROOT/scripts/build-cli.sh"
"$ROOT/scripts/build-web.sh"
echo "All components built."

