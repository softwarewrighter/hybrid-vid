# Hybrid-VID Architecture

## Overview
- Purpose: Combine capabilities of `video-publishing` and `yt-rs` into a unified platform for planning, generating, and publishing YouTube videos.
- Modes:
  - Fine-grained, human-in-the-loop control for each production step.
  - Autonomous, multi-agent parallel development of multiple video projects using a shared template.
- Template: This repo serves as the template for per-video workspaces that agents clone/instantiate.

## Core Components
- Orchestrator: Coordinates pipelines, step execution, retries, and hand-offs between human and agents.
- Project Template: Standard directory + config used to scaffold a new video project instance.
- Task Graph Engine: DAG of production tasks (ideation → research → script → assets → edit → render → publish), supports conditional branches and parallelism.
- Agent Runner: Spawns and supervises specialized AI agents per task group (researcher, writer, audio, visual, editor, publisher).
- Human Checkpoints: Configurable approval gates with diff/summaries and revert/redo.
- Storage Abstraction: Project artifacts, prompts, outputs, and metadata stored in a structured tree with consistent naming.
- Integrations: Bridges to `video-publishing` tooling and `yt-rs` services/CLI where applicable.

## Data Flow
1) Project created from template → initializes metadata and step graph.
2) Orchestrator schedules runnable steps; Agent Runner or human performs steps.
3) Artifacts written to the project’s workspace; metadata updated.
4) Downstream steps consume upstream artifacts via well-defined interfaces.
5) Final output: rendered video + thumbnails + descriptions → publish via configured channel(s).

## Execution Model
- Deterministic Steps: Prefer idempotent, versioned inputs/outputs; store provenance.
- Parallelism: Independent branches (e.g., B-roll search vs. script polish) can run concurrently.
- Resumability: Checkpoints and cached artifacts allow pause/resume/retry.
- Observability: Each step emits logs, metrics, and structured status for dashboards.

## Configuration
- Global: Default agent roles, model/tooling settings, directory conventions.
- Per-Project: PRD, constraints, schedule, task toggles, prompts, and assets.
- Per-Step: Inputs, outputs, policies (auto/human), timeouts, and fallback strategies.

## Directory Layout (Template)
- `project/metadata/`: `project.json`, `status.json`, `milestones.json`.
- `project/docs/`: PRD, script drafts, storyboard, notes.
- `project/assets/`: raw media, images, audio, stock B-roll.
- `project/output/`: intermediate renders, final exports, thumbnails, captions.
- `project/logs/`: step logs and traces.
- `project/config/`: step graph, agent settings, publish targets.

## Interfaces and Contracts
- Step I/O: Each step declares schemas for inputs/outputs; validated before/after execution.
- Agent API: Standard request/response envelope with context, constraints, and artifact paths.
- Tool Bridges:
  - `video-publishing`: rendering, composition, and publishing utilities.
  - `yt-rs`: YouTube API operations, metadata management, analytics.

## Extensibility
- Add Steps: Register new tasks with I/O contracts and default policies.
- Swap Agents: Replace or augment agent implementations per step.
- Custom Pipelines: Override or extend the DAG per project.

## Security and Compliance
- Secrets management for API keys and tokens (local dev vs. CI).
- Content safety checks and licensing for assets.
- Audit trail of agent prompts and outputs.

## Open Questions
- Optimal split of responsibilities between `video-publishing` and `yt-rs`.
- Standard prompt packs and evaluation benchmarks.
- Review UX for human checkpoints.

