#!/usr/bin/env bash
set -euo pipefail
cd "$(dirname "$0")/.."/workspaces/processing
cargo build --workspace --release

