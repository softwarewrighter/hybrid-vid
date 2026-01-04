#!/usr/bin/env bash
set -euo pipefail
cd "$(dirname "$0")/.."/workspaces/web-ui
rustup target add wasm32-unknown-unknown >/dev/null 2>&1 || true
cargo build -p hv-web-app --target wasm32-unknown-unknown --release
echo "Built hv-web-app (WASM). For local serve, install trunk: 'cargo install trunk' and run 'trunk serve' inside hv-web-app/."

