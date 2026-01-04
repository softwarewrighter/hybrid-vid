# Product Requirements (PRD): Hybrid-VID

## Summary
- Goal: Provide a robust, repeatable pipeline to ideate, produce, and publish YouTube videos with both human-in-the-loop control and autonomous multi-agent execution.
- Users: Solo creators, small media teams, and programmatic content operations.

## Problem
- Manual video production is time-consuming and inconsistent.
- Existing tools are fragmented; orchestration and handoffs are brittle.
- Parallel development across multiple videos is error-prone without templates and automation.

## Objectives
- Repeatable project template for each video.
- End-to-end pipeline orchestration with observable status.
- Fine-grained controls with configurable human checkpoints.
- Scalable multi-agent parallelization for multiple projects.
- First-class integration with `video-publishing` and `yt-rs`.

## Scope (MVP)
- Project template creation CLI/command.
- Step graph definition for core pipeline (ideation → publish).
- Basic Agent Runner with at least researcher + scriptwriter + publisher roles.
- Human checkpoint mechanism for script approval and final publish.
- Storage layout and artifact conventions.
- Status dashboard (CLI textual to start) showing per-step state.

## Non-Goals (MVP)
- Full-fledged GUI dashboard.
- Advanced editing/AE timeline manipulation.
- Complex collaboration permissions.

## Functional Requirements
- Create new project from template with unique ID and metadata.
- Configure auto/human policy per step and per project.
- Run steps individually or as part of an orchestrated DAG.
- Resume from checkpoints; retry failed steps.
- Publish to YouTube via `yt-rs` with title, desc, tags, thumbnail.
- Log and persist all agent prompts and outputs.

## Performance/Scale
- Handle 10–50 concurrent projects on a modest workstation/runner.
- Steps should be idempotent and cache intermediate outputs to reduce recompute.

## Success Metrics
- Time-to-first-video from template < 2 hours.
- 90% of steps are resumable without manual cleanup.
- < 5% publish errors attributed to orchestration.

## Dependencies
- `video-publishing` (rendering/publishing pipeline tools).
- `yt-rs` (YouTube API client/services).
- Local encoders (ffmpeg, etc.) when applicable.

## Risks
- API limits and auth complexity.
- Asset licensing and content policy compliance.
- Drift between template and per-project customizations.

## Open Questions
- Model/tool selection defaults and configurability.
- Best UX for diff-based approvals.

