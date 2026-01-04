# Hybrid-VID

Rust-first, multi-component toolkit for YouTube video production:
- Reusable processing libraries (audio/video blocks + pipeline engine)
- Multiple CLI binaries wrapping the same core logic
- Yew-based web UI to compose and run processing blocks interactively

No Python/JavaScript/TypeScript. Web UI is Rust (WASM) with Yew.

## Layout
- `workspaces/processing/` — processing engine and block crates
- `workspaces/cli-tools/` — command-line binaries using processing libs
- `workspaces/web-ui/` — Yew web app and shared web-facing types
- `workspaces/adapters/` — adapters (yt-rs, video-publishing) stubs
- `scripts/` — build scripts per component
- `documentation/` — architecture, PRD, design, plan, status

## Build
- Processing libs: `scripts/build-processing.sh`
- CLI tools: `scripts/build-cli.sh`
- Web UI (WASM): `scripts/build-web.sh` (uses `wasm32-unknown-unknown` target; Trunk optional)
- Everything: `scripts/build-all.sh`

## Notes
- Do not modify symlinked repos. Copy code selectively into adapters or blocks when needed.
- Reuse instructions from `video-publishing` and `yt-rs` documents (e.g., `process.md`, `Claude.md`, `tools.md`).
- Adapters are placeholders; we’ll populate them after confirming integration surfaces.

## Toolchain
- Rust edition: `2024` across all crates. Ensure your toolchain is up to date, e.g. `rustup update`.
