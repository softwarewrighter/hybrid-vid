# Project Plan: Hybrid-VID

## Phases
1) Scaffolding: docs, template layout, basic CLI.
2) Orchestrator: DAG engine, state, policies, status.
3) Agents: researcher, writer, publisher (MVP); adapters to `video-publishing` and `yt-rs`.
4) Human Checkpoints: approvals and diffs in CLI.
5) Parallelization: multiple projects, resource management, caching.
6) Polishing: logs, metrics, error handling, docs.

## Milestones
- M1: Documentation and template skeleton in repo.
- M2: `cli init` creates a new video project.
- M3: `cli run` executes 3–5 core steps end-to-end on sample content.
- M4: `cli approve` gates critical steps; resumable retries.
- M5: `cli publish` uploads via `yt-rs` with metadata and thumbnail.

## Tasks (Initial)
- Define project template directories and metadata schemas.
- Implement orchestrator with simple in-memory state + JSON persistence.
- Implement Agent Runner interface and minimal agents.
- Implement adapters for `video-publishing` and `yt-rs`.
- Implement CLI commands: `init`, `run`, `step`, `status`, `approve`, `publish`.
- Add example project and sample pipeline config.

## Parallelization Strategy
- Independent per-project workspaces; orchestrator instances per project.
- Queue-based scheduling to avoid resource contention (e.g., encoder usage).
- Cache and reuse common assets across projects when allowed.

## Risks/Mitigations
- API changes: wrap in adapters; version interfaces.
- Long-running renders: checkpoint outputs, stream logs.
- Content quality drift: introduce review policies and prompt packs.

