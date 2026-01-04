# Repository Guidelines

## Project Structure & Module Organization
- Root: this repository (docs in `documentation/`).
- Workspaces: `workspaces/processing/`, `workspaces/cli-tools/`, `workspaces/web-ui/`, `workspaces/adapters/`.
- Scripts: `scripts/` build helpers.
- Rule: do not modify symlinked repos (`video-publishing/`, `yt-rs/`); copy logic into adapters/libs.
- Follow CLAUDE.md rules from both symlinked projects (see `documentation/sources/*/Claude.md`).

## Build, Test, and Development Commands
- Build all: `scripts/build-all.sh`
- Processing libs: `scripts/build-processing.sh`
- CLI tools: `scripts/build-cli.sh`
- Web (WASM): `scripts/build-web.sh` (requires `wasm32-unknown-unknown`; optional `trunk serve` in `hv-web-app/`).
- Tests (any workspace): `cargo test --workspace --all-features`
- Lint/format: `cargo fmt --all`, `cargo clippy --workspace -W warnings`

## Coding Style & Naming Conventions
- Language: Rust (edition 2024) across all crates.
- Formatting: rustfmt defaults (4-space indent, 100–120 cols).
- Naming: crates kebab-case (e.g., `processing-core`), modules/functions snake_case, types CamelCase, binaries `hv-*`.
- Public APIs: prefer small, composable traits; keep step I/O explicit via types.

## Testing Guidelines
- Place tests alongside code (`src/lib.rs` unit tests) and integration tests in `tests/` per crate.
- Name tests after behavior (e.g., `normalizes_peak_levels`).
- Aim for coverage on core engine, block I/O contracts, and adapters’ surfaces (mock external calls).
- Run: `cargo test --workspace` before PRs.

## Commit & Pull Request Guidelines
- Messages: imperative, concise, scoped (e.g., `processing-core: add DAG cycle check`).
- PRs: include summary, rationale, screenshots for UI, and links to issues.
- Keep changes small and focused; avoid unrelated refactors.
- Do not commit secrets. Do not alter symlinked repos.

## Security & Configuration Tips
- Store API keys outside VCS (env or OS keychain). Adapters should read from env and fail closed.
- Review generated artifacts before publishing; respect licenses for third‑party assets.

## Agent‑Specific Instructions
- Treat `documentation/sources/video-publishing/Claude.md` and `documentation/sources/yt-rs/Claude.md` as authoritative: follow the stated process, use the prescribed tools, observe checkpoints, and document learnings.
- Align with `documentation/process.md`, `documentation/tools.md`, and `documentation/ai_agent_instructions.md` (Proact). Prefer repo scripts over ad‑hoc commands.
- Prefer library‑first changes reused by both CLIs and the Yew UI; stage risky changes behind flags.
- Use human checkpoints for publish‑critical steps; attach diffs/artifacts in PRs.
