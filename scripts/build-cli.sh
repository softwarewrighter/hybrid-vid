#!/usr/bin/env bash
set -euo pipefail
cd "$(dirname "$0")/.."/workspaces/cli-tools
cargo build --workspace --release

