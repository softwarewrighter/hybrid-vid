# Design: Hybrid-VID

## High-Level Design
- Orchestrator manages a DAG of production steps with policies per step.
- Agent Runner executes tasks using role-specific agents or shells out to tools.
- Human checkpoints provide approval UI via CLI prompts and diffs.
- Integrations wrap existing capabilities from `video-publishing` and `yt-rs` behind stable interfaces.

## Modules
- core/orchestrator: DAG engine, scheduling, retries, persistence.
- core/state: project + step state, metadata, serialization.
- core/io: artifact paths, schemas, validation.
- agents/: role implementations (researcher, writer, editor, publisher).
- adapters/video_publishing/: bridge to `video-publishing` actions.
- adapters/yt_rs/: bridge to `yt-rs` for YouTube operations.
- cli/: user commands (init, run, step, status, approve, publish).

## Data Model (Sketch)
- Project: id, name, created_at, config, status, milestones.
- Step: id, name, inputs, outputs, status, logs, attempts, policy.
- Artifact: path, type, checksum, provenance.

## Step Policies
- auto: run agent/tool immediately when inputs are ready.
- human: require approval before running.
- review: run automatically, require approval to accept outputs.

## Workflows
- Init: `cli init --name <video>` → scaffold template, create graph.
- Run: `cli run` → orchestrator schedules tasks with ready inputs.
- Step: `cli step <name>` → run a specific step.
- Approve: `cli approve <step>` → accept outputs and unblock dependents.
- Status: `cli status` → show per-step progress and blocking items.
- Publish: `cli publish` → finalize metadata + call `yt-rs`.

## Observability
- Logs per step in `project/logs/<step>/<attempt>.log`.
- Status JSON with timestamps per step; summarized in CLI.
- Optional event hooks for dashboards/notifications.

## Error Handling
- Retries with exponential backoff for transient failures.
- Clear failure reasons and next-action hints.
- Partial artifact cleanup on abort with confirmation.

## Security
- Centralized secret management (env/manager); least privilege to adapters.
- Scrub PII from logs where possible.

## Future Enhancements
- Web dashboard with live graph.
- More agent specializations (thumbnail artist, voiceover, b-roll curator).
- Prompt packs and evaluation for quality control.

