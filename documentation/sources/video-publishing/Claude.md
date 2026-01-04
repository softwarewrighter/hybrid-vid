# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Access Restrictions

**DO NOT USE** any of the following:
- `ssh` commands to remote hosts
- `sudo` or any commands requiring password authentication
- Any interactive commands requiring user input
- **Bare `ffmpeg` commands** - ALWAYS use the Rust `vid-*` CLI tools instead (see below)

When remote host verification is needed, ask the user to run the commands manually.

### MANDATORY: Use Rust CLI Tools, NOT ffmpeg

**THIS IS A HARD RULE. DO NOT VIOLATE IT.**

Every video pipeline operation MUST use the corresponding Rust CLI tool:

| Operation | USE THIS | NOT THIS |
|-----------|----------|----------|
| Extract stills | `vid-frames --input video.mp4 --output dir/` | ~~`ffmpeg -vf "fps=1/30"`~~ |
| Generate TTS | `vid-tts --script file.txt --output audio.wav` | ~~`podcast` directly~~ |
| Stretch avatar | `vid-avatar --facing left --duration 10.5` | ~~`ffmpeg -vf setpts`~~ |
| Lip sync | `vid-lipsync --avatar a.mp4 --audio b.wav` | ~~`musetalk-cli` directly~~ |
| Composite | `vid-composite --content c.mp4 --avatar a.mp4` | ~~`ffmpeg -filter_complex overlay`~~ |
| Concatenate | `vid-concat --list concat.txt --output final.mp4` | ~~`ffmpeg -f concat`~~ |
| Speed up | `vid-speedup --input v.mp4 --speed 16.0` | ~~`ffmpeg -vf setpts`~~ |
| Create slide | `vid-slide --title "Title" --duration 4.0` | ~~ImageMagick + ffmpeg~~ |
| Add music | `vid-music --input v.mp4 --music m.mp3 --output o.mp4` | ~~`ffmpeg -filter_complex`~~ |

The Rust tools are located at: `/Users/mike/github/softwarewrighter/video-publishing/tools/target/release/`

## Overview

This project converts OBS screen recordings into narrated YouTube videos with AI-generated voice-over and lip-synced avatar overlays.

## Pipeline Modes

### Draft Mode (Default)
Fast iteration for reviewing AI-generated scripts and checking lip-sync timing.
- Skips background removal (rembg step)
- Avatar has solid background (not transparent)
- Good for validating scripts and timing before final render

### Full/Final Mode (`--mode final`)
Production-quality output with transparent avatar overlays.
- Runs rembg frame-by-frame background removal (SLOW: ~1 min per second of video)
- Avatar overlays are transparent so viewer can read text behind them
- Use only after validating draft mode output

## Commands

### Main Pipeline
```bash
# Draft mode (default) - fast iteration
./obs2yt.sh <source_video.mp4> [project_name]

# Full mode - production quality with transparent avatars
./obs2yt.sh ~/Movies/demo.mp4 my-project --mode final

# With project context (RECOMMENDED - provides accurate AI analysis)
./obs2yt.sh ~/Movies/demo.mp4 my-project --project-path ~/github/user/repo

# With all options
./obs2yt.sh ~/Movies/demo.mp4 my-project \
  --project-path ~/github/softwarewrighter/xmas-rs \
  --avatar front-polo.mp4 \
  --voice ~/Audio/ref.wav \
  --mode draft
```

### Single Clip Testing
Test one segment from dialog through composite video:
```bash
# Test specific segment
./obs2yt.sh ~/Movies/demo.mp4 my-project --test-segment 01-content

# Re-run just lip-sync and composite for one segment
./obs2yt.sh ~/Movies/demo.mp4 my-project --redo-segment 03-content
```

### Project Context for AI Analysis

**IMPORTANT**: Use `--project-path` to provide local repo path. This:
1. Extracts project context (README, languages, WASM indicators)
2. Derives GitHub URL from `git remote get-url origin`
3. Passes context to vision model to prevent hallucinations

Without `--project-path`, the vision model may guess incorrect technologies.

### Testing
```bash
# Run all tests
./tests/run_all_tests.sh

# Individual tests
./tests/test_tts.sh              # Test TTS voice cloning
./tests/test_transparency.sh      # Test background removal
./tests/test_avatar_transparency.sh
./tests/generate_test_video.sh    # Create synthetic test video
```

### Preprocessing
```bash
# Create transparent avatar videos (one-time)
./preprocess_avatars.sh

# Batch convert all avatars to transparent
./make-transparent-videos.sh [input_dir] [output_dir]
```

## Architecture

The pipeline runs in 12 sequential steps (ALL using Rust vid-* tools):

1. **Extract Stills** - `vid-frames --input video.mp4 --interval 30`
2. **Generate Scripts** - AI analyzes frames and creates narration segments
3. **Generate TTS Audio** - `vid-tts --script file.txt --output audio.wav` (includes padding)
4. **Pad Audio** - Handled automatically by vid-tts (adds 1s start + 1s end)
5. **Stretch Avatars** - Time-stretch avatar videos to match PADDED audio duration
6. **Lip-Sync** - `musetalk-cli` generates lip-synced avatar videos
7. **Remove Background** - `rembg` ML-based background removal
8. **Create Thumbnails** - Scale avatars to overlay size
9. **Cut Base Clips** - Extract video segments at timecodes
10. **Composite** - Overlay avatar on video clips with ffmpeg overlay filter
11. **Concatenate** - Join all segments into final video
12. **Generate Metadata** - Create thumbnail.jpg, description.txt

### External Dependencies

| Tool | Location | Purpose |
|------|----------|---------|
| `podcast` | `../open-tts-rs/target/release/podcast` | TTS voice cloning |
| `musetalk-cli` | `../musetalk-client-rs/target/release/musetalk-cli` | Lip-sync generation |
| `rembg` | `../musetalk-client-rs/.venv/bin/rembg` | Background removal |
| `ask` | System PATH | AI script generation (optional) |

## CRITICAL: Use Custom Rust Binaries, NOT Bare ffmpeg

**NEVER use bare ffmpeg commands for video pipeline operations.** Instead, ALWAYS use the custom Rust binaries in `tools/target/release/`. These binaries encapsulate correct ffmpeg parameters, handle edge cases, and ensure consistent output.

### Available vid-* Tools

| Binary | Purpose | Key Features |
|--------|---------|--------------|
| `vid-avatar` | Stretch avatar to target duration | Converts to 30fps, forces exact duration |
| `vid-lipsync` | Generate lip-synced avatar | Wraps musetalk-cli with retries |
| `vid-tts` | Generate TTS audio | Wraps podcast CLI |
| `vid-composite` | Overlay avatar on video | Handles positioning, scaling |
| `vid-concat` | Concatenate video segments | Creates concat file, joins clips |
| `vid-slide` | Generate title/content slides | Text rendering with styling |
| `vid-prologue` | Generate prolog slide | Title card with branding |
| `vid-epilog` | Generate epilog slide | Links and CTA |
| `vid-frames` | Extract stills from video | Consistent frame extraction |
| `vid-speedup` | Speed up video segments | For silent B-roll |
| `vid-music` | Add background music to video | Volume control, fade in/out |
| `vid-image` | Convert static image to video | With optional music overlay |
| `vid-quadrants` | Split image into 4 quadrants | For montage effects |
| `vid-montage` | Create rapid-cut video from images | Shuffle, custom duration, music |
| `vid-review` | Web preview of video segments | Live-reloading preview server |

### vid-review: MANDATORY for Previews

**NEVER create custom preview.html files.** Always use `vid-review`:

```bash
VID_REVIEW="/Users/mike/github/softwarewrighter/video-publishing/tools/target/release/vid-review"

# Start preview server for a project WITH MANIFEST (REQUIRED for custom ordering)
$VID_REVIEW /path/to/project/work/composited \
  --manifest /path/to/project/work/manifest.json \
  --port 3030

# Example for comparing-2-ai-agents project:
$VID_REVIEW /Users/mike/github/softwarewrighter/video-publishing/projects/comparing-2-ai-agents/work/composited \
  --manifest /Users/mike/github/softwarewrighter/video-publishing/projects/comparing-2-ai-agents/work/manifest.json \
  --port 3030
```

**CRITICAL: Always pass `--manifest`** to ensure segments appear in the correct order defined in manifest.json.
Without it, vid-review will show "Loaded 0 segments" and segments won't appear.

Then open http://localhost:3030 in your browser. The preview auto-refreshes when files change.

### CRITICAL: Manifest Management

**The `manifest.json` file defines the video segment order and MUST be updated when adding or reordering clips.**

#### Location
Each project has its own manifest at: `projects/<project-name>/work/manifest.json`

#### Format
```json
{
  "segments": [
    "00-title",
    "01-intro",
    "02-content",
    ...
    "99z-epilog"
  ]
}
```

#### When to Update the Manifest

**ALWAYS update manifest.json when:**
1. Adding new segments (slides, narration clips, etc.)
2. Reordering existing segments
3. Removing segments
4. Adding closing segments (90-99 series)

#### Segment Naming Conventions

| Prefix | Purpose | Example |
|--------|---------|---------|
| 00-* | Title/hook segments | 00-title, 00-hook |
| 01-* | Intro/teaser segments | 01-meme, 01-teaser |
| cc-* | Claude Code segments | cc-01-explore, cc-07-intro |
| oc-* | OpenCode segments | oc-01-prompt, oc-07-intro |
| 90-* | Results/scorecard | 90a-results, 90b-results |
| 91-* | Time analysis | 91a-time, 91b-time |
| 92-* | Mea culpa/confessions | 92a-mea-culpa |
| 93-* | Conclusion | 93a-conclusion |
| 99a-e | Project-specific epilog narration | 99a-epilog, 99b-epilog |
| 99z | Global epilog (pre-made) | 99z-epilog |

#### Global vs Project Epilog

- **99z-epilog**: The pre-made global epilog from `/reference/epilog/99b-epilog.mp4`. Copy to `composited/99z-epilog.mp4` and add to manifest last.
- **99a-e epilog**: Project-specific narration segments with avatar overlay.

#### Creating Draft Video from Manifest

```bash
WORKDIR="/path/to/project/work"
VID_CONCAT="/Users/mike/github/softwarewrighter/video-publishing/tools/target/release/vid-concat"

# Create concat list from manifest (paths only, one per line)
python3 << 'EOF'
import json
workdir = "/path/to/project/work"
with open(f"{workdir}/manifest.json") as f:
    manifest = json.load(f)
with open(f"{workdir}/composited/draft-concat.txt", "w") as f:
    for seg in manifest["segments"]:
        f.write(f"{workdir}/composited/{seg}.mp4\n")
EOF

# Concatenate into draft video
$VID_CONCAT --list "$WORKDIR/composited/draft-concat.txt" --output "$WORKDIR/draft-full.mp4" --reencode --print-duration
```

### Why Custom Binaries Matter

Bare ffmpeg commands often miss critical parameters:
- **FPS conversion**: Reference avatars are 24fps, but MuseTalk needs 30fps input
- **Exact duration**: Must use `-t` flag to force exact output duration
- **Codec settings**: Consistent encoding parameters across all clips

**Example of what goes wrong with bare ffmpeg:**
```bash
# WRONG - Keeps 24fps, causes lip-sync mismatch
ffmpeg -y -i avatar.mp4 -vf "setpts=3.8*PTS" -an stretched.mp4

# CORRECT - Use vid-avatar which adds -r 30 and -t duration
vid-avatar --facing center --duration 19.20 --output stretched.mp4
```

**If you need functionality not covered by existing tools, ASK THE USER to create a new Rust binary.**

### Required Servers

- **TTS (VibeVoice)**: `curiosity:7861`
- **MuseTalk**: `hive:3015`
- **Ollama Vision**: `big72:11434` (llava:13b model)

## Standard Pipeline Using vid-* Tools

**ALWAYS use vid-* tools instead of bare ffmpeg/podcast/musetalk-cli commands.**

### Step 1: Generate TTS Audio (vid-tts)

`vid-tts` handles TTS generation AND padding automatically:

```bash
VID_TTS="/Users/mike/github/softwarewrighter/video-publishing/tools/target/release/vid-tts"

# From script file (includes 1s padding by default)
$VID_TTS --script scripts/00-prolog.txt --output audio/00-prolog.wav --print-duration

# From text directly
$VID_TTS --text "Hello world" --output audio/test.wav

# Custom voice reference (default: ~/Audio/mike-medium-ref-1.wav)
$VID_TTS --script scripts/00-prolog.txt --voice /path/to/voice.wav --output audio/00-prolog.wav

# Skip padding if needed
$VID_TTS --script scripts/00-prolog.txt --output audio/00-prolog.wav --no-pad
```

#### CRITICAL: Keep TTS Scripts SHORT (1-2 Sentences Max)

**Long scripts cause problems:**
1. **Garbled audio** - VibeVoice produces artifacts on long text
2. **Poor GPU pipelining** - Long TTS blocks prevent overlap with lip sync
3. **Harder to fix** - If one sentence is bad, must regenerate everything

**ALWAYS split narration into multiple short segments:**

```
WRONG (too long - will cause garbled audio):
90-results.txt:
"So here's the scorecard. Both agents created working apps on the first try.
Well, working in the loosest sense of the word. Claude built the prettier app
with more controls, but then spent twenty minutes trying to click a button."

CORRECT (short segments - clean audio, better pipelining):
90a-results.txt: "So here's the scorecard. Both agents created working apps on the first try."
90b-results.txt: "Well, working in the loosest sense of the word."
90c-results.txt: "Claude built the prettier app with more controls, but then spent twenty minutes trying to click a button."
```

**Rule of thumb:** If a script has more than 2 sentences or takes longer than ~8-10 seconds of audio, split it into multiple segments.

**ENFORCED BY TOOLING:** `vid-tts` will REJECT scripts exceeding 200 characters or 2 sentences. There is NO override flag - you MUST split long text into shorter segments.

### Step 2: Stretch Avatar (vid-avatar)

```bash
VID_AVATAR="/Users/mike/github/softwarewrighter/video-publishing/tools/target/release/vid-avatar"
REFDIR="/Users/mike/github/softwarewrighter/video-publishing/reference"

# Stretch to match audio duration (outputs 30fps)
$VID_AVATAR --facing center --duration 19.20 --reference-dir "$REFDIR" --output avatar/stretched/00-prolog.mp4

# --facing options: center, left (selects appropriate reference avatar)
```

### Step 3: Lip-Sync (vid-lipsync)

```bash
VID_LIPSYNC="/Users/mike/github/softwarewrighter/video-publishing/tools/target/release/vid-lipsync"

# Default server: hive:3015, default fps: 30
$VID_LIPSYNC --avatar avatar/stretched/00-prolog.mp4 --audio audio/00-prolog.wav --output avatar/lipsynced/00-prolog.mp4

# Use alternate server
$VID_LIPSYNC --avatar avatar/stretched/00-prolog.mp4 --audio audio/00-prolog.wav --server hive:3016 --output avatar/lipsynced/00-prolog.mp4
```

### Step 4: Speed Up Video (vid-speedup)

```bash
VID_SPEEDUP="/Users/mike/github/softwarewrighter/video-publishing/tools/target/release/vid-speedup"

# 16x speed (muted by default)
$VID_SPEEDUP --input source.mp4 --speed 16.0 --output speedup.mp4

# With start/duration for clipping
$VID_SPEEDUP --input source.mp4 --speed 16.0 --start 0 --duration 420 --output speedup.mp4
```

### Step 5: Composite (vid-composite)

```bash
VID_COMPOSITE="/Users/mike/github/softwarewrighter/video-publishing/tools/target/release/vid-composite"

$VID_COMPOSITE --content clips/00-prolog.mp4 --avatar avatar/lipsynced/00-prolog.mp4 --output composited/00-prolog.mp4 --size 200 --position bottom-right
```

### Step 6: Concatenate (vid-concat)

```bash
VID_CONCAT="/Users/mike/github/softwarewrighter/video-publishing/tools/target/release/vid-concat"

# Create concat list file, then:
$VID_CONCAT --list concat.txt --output final.mp4 --reencode
```

### Step 7: Add Background Music (vid-music)

For slides and silent fast-forward segments, add background music:

```bash
VID_MUSIC="/Users/mike/github/softwarewrighter/video-publishing/tools/target/release/vid-music"

# Add music to a slide (30% volume is default, with fade in/out)
$VID_MUSIC --input clips/02-challenge.mp4 --music music/track.mp3 --output composited/02-challenge.mp4

# Sample from 15 seconds into the music track
$VID_MUSIC --input clips/03-database.mp4 --music music/track.mp3 --music-offset 15.0 --output composited/03-database.mp4

# Custom volume (50%)
$VID_MUSIC --input clips/slide.mp4 --music music/track.mp3 --volume 0.5 --output composited/slide.mp4

# Custom fades (1s in, 2s out)
$VID_MUSIC --input clips/slide.mp4 --music music/track.mp3 --fade-in 1.0 --fade-out 2.0 --output composited/slide.mp4
```

**When to use vid-music:**
- Title slides
- Transition slides (e.g., "But First...")
- Fast-forward (FF) segments
- Epilog slides

**When NOT to use vid-music:**
- Segments with avatar narration (the avatar audio IS the audio track)

## CRITICAL: Random Avatar Selection

Each segment MUST use a DIFFERENT randomly-selected avatar from the pool for visual variety.

```bash
# Reference avatars location (5 approved avatars)
AVATAR_VIDEOS_DIR="$SCRIPT_DIR/reference"

# CORRECT: Random selection per segment using shuf
avatar=$(ls "$AVATAR_VIDEOS_DIR"/*.mp4 | shuf -n 1)
echo "Selected: $(basename $avatar)"

# WRONG: Using same avatar for all segments
avatar="$AVATAR_VIDEOS_DIR/front-polo.mp4"  # DO NOT DO THIS
```

Available reference avatars:
- `front-polo.mp4`
- `head-left-front.mp4`
- `veo-head.mp4`
- `veo-loop.mp4`
- `xmas-polo.mp4`

## CRITICAL: Lip-Sync FPS and Sequential Execution

### FPS 30 Requirement (NOT 15!)

**PROBLEM**: MuseTalk server generates ~30 frames per second of audio internally. Using `--fps 15` causes frame assembly bugs where:
- Output video duration doesn't match audio duration
- Lips continue moving after audio stops
- Lip sync is out of phase with audio

**ROOT CAUSE**: The musetalk-cli assembler has issues when downsampling from 30fps to 15fps - frames get dropped or timing is miscalculated.

**SOLUTION**: Always use `--fps 30` with musetalk-cli:

```bash
# CORRECT: Use --fps 30 (matches MuseTalk server's internal generation rate)
musetalk-cli --server http://hive:3015 \
  --reference avatar/stretched/00-prolog.mp4 \
  --audio audio/00-prolog-padded.wav \
  --output avatar/lipsynced/00-prolog.mp4 \
  --fps 30

# WRONG: --fps 15 causes duration mismatch and lip sync issues
musetalk-cli ... --fps 15  # DO NOT USE --fps 15
```

**VERIFICATION**: After lip-sync, verify durations match:
```bash
# Audio and lip-synced video should have same duration (within 0.1s)
ffprobe -v error -show_entries format=duration -of csv=p=0 audio/00-prolog-padded.wav
ffprobe -v error -show_entries format=duration -of csv=p=0 avatar/lipsynced/00-prolog.mp4
```

### CRITICAL: GPU Scheduling Rules

**Each GPU can only run ONE task at a time.** Overloading causes CUDA OOM, job failures, reduced throughput, and wasted electricity.

#### Available GPU Resources (3 Total)

| Server | Port | GPU | Service | Task Type |
|--------|------|-----|---------|-----------|
| curiosity | 7861 | GPU #1 | VibeVoice | TTS audio generation |
| hive | 3015 | GPU #2 | MuseTalk | Lip sync |
| hive | 3016 | GPU #3 | MuseTalk | Lip sync |

#### Goal: Maximize Throughput

**Use ALL 3 GPUs simultaneously, but EXACTLY 1 task per GPU:**
- While generating TTS on curiosity:7861, you can run 2 lip-sync jobs on hive:3015 and hive:3016
- Never queue multiple tasks on the same GPU
- Never leave GPUs idle when there's work to do

#### Optimal: Parallel on Both GPUs (2x speed)

Run exactly ONE job per GPU simultaneously using `run_in_background: true`:

```bash
# CORRECT: One job per GPU, both running in parallel
# Job 1 on GPU 3015 (background)
$VID_LIPSYNC --avatar seg1.mp4 --audio seg1.wav --server hive:3015 --output out1.mp4
# Job 2 on GPU 3016 (background)
$VID_LIPSYNC --avatar seg2.mp4 --audio seg2.wav --server hive:3016 --output out2.mp4
# Wait for both to complete before starting next pair
```

#### Acceptable: Sequential on One GPU (1x speed)

If only one GPU available, run jobs one at a time:

```bash
# ACCEPTABLE: Sequential on single GPU
for seg in 01 02 03 04; do
  $VID_LIPSYNC --avatar ${seg}.mp4 --audio ${seg}.wav --server hive:3015 --output out-${seg}.mp4
done
```

#### FORBIDDEN: Multiple Jobs Per GPU

**NEVER schedule more than one job per GPU port. This causes:**
- CUDA out of memory errors
- All jobs fail
- Wasted GPU time

```bash
# WRONG: Multiple jobs on same GPU (CUDA OOM)
$VID_LIPSYNC ... --server hive:3015 &
$VID_LIPSYNC ... --server hive:3015 &  # SECOND JOB ON SAME GPU - WILL FAIL
```

#### FORBIDDEN: Killing and Restarting Jobs

**NEVER kill running MuseTalk jobs to restart them.** This is worse than waiting:
- The GPU has already done partial work
- Killing wastes that work
- Restarting doubles the total time

If jobs are running, WAIT for them to complete. Check running jobs:
```bash
ps aux | grep musetalk-cli | grep -v grep
```

#### Claude Code: Proper Parallel Execution

When using Claude Code to run parallel lip-sync jobs, you MUST:

1. Use `run_in_background: true` for BOTH Bash commands
2. Send BOTH commands in a SINGLE message (not sequential messages)
3. Wait for BOTH to complete with TaskOutput before starting next pair

```
# CORRECT Claude Code pattern:
# In a single response, call BOTH:
Bash(command for hive:3015, run_in_background=true)
Bash(command for hive:3016, run_in_background=true)
# Then wait for both:
TaskOutput(task_id_1)
TaskOutput(task_id_2)
# Only then start next pair
```

**Common mistake:** Running Bash commands without `run_in_background` causes the second command to wait or get interrupted, leading to retries that overload the GPU.

#### CRITICAL: Pipeline TTS and Lip Sync (DO NOT BATCH)

**NEVER batch operations.** Do NOT generate all TTS audio first, then stretch all avatars, then lip sync all segments. This leaves GPUs idle.

**ALWAYS pipeline operations** to keep all 3 GPUs busy:

```
WRONG (Batched - GPUs idle most of the time):
  1. TTS seg1, TTS seg2, TTS seg3, TTS seg4, TTS seg5, TTS seg6  (curiosity busy, hive idle)
  2. Stretch all avatars (CPU only, all GPUs idle)
  3. Lipsync seg1+seg2, seg3+seg4, seg5+seg6 (hive busy, curiosity idle)

CORRECT (Pipelined - all GPUs busy):
  Time 0: TTS seg1 on curiosity
  Time 1: TTS seg2 on curiosity | Stretch seg1 (CPU)
  Time 2: TTS seg3 on curiosity | Stretch seg2 | Lipsync seg1 on hive:3015
  Time 3: TTS seg4 on curiosity | Stretch seg3 | Lipsync seg2 on hive:3016 | seg1 continues
  Time 4: TTS seg5 on curiosity | Stretch seg4 | Lipsync seg3 on hive:3015 | seg2 continues
  ... (all 3 GPUs working simultaneously)
```

**Implementation pattern:**
1. Start TTS for segment N on curiosity:7861 (background)
2. While TTS runs, stretch avatar for segment N-1 (CPU, fast)
3. When segment N-2's avatar is ready, start lip sync on hive:3015 or hive:3016
4. Repeat - always have work queued for each GPU

**The goal**: At any moment, curiosity should be generating TTS while both hive GPUs are lip syncing. If any GPU is idle while there's pending work, the pipeline is wrong.

## Segment and Still Relationship

Stills are extracted every 30 seconds. Dialog segments relate to stills as follows:

| Segment | Still(s) | Video Offset | Description |
|---------|----------|--------------|-------------|
| 00-prolog | frame_0001 (0:00) | Title card | Introduction overlay on title |
| 01-content | frame_0001 (0:00) | 0:00-0:30 | First 30s of source video |
| 02-content | frame_0002 (0:30) | 0:30-1:00 | Second 30s |
| 03-content | frame_0003 (1:00) | 1:00-1:30 | Third 30s |
| 04-content | frame_0004 (1:30) | 1:30-2:00 | Fourth 30s |
| ... | ... | ... | ... |
| 99-epilog | Last frame | End of video | Conclusion/CTA |

**Note**: Number of stills = ceil(video_duration / 30). Number of content segments = stills - 1 (prolog/epilog are separate).

## Preview HTML Requirements

The preview (`work/preview.html`) is for validating artifacts during pipeline execution.

### REQUIRED Annotations

Every artifact MUST display:

1. **Audio files**: Duration in seconds (e.g., "5.6s")
2. **Video files**: Duration in seconds (e.g., "8.0s")
3. **Stills**: Source video timestamp (e.g., "0:30", "1:00")
4. **Script segments**: Which still(s) they correspond to
5. **Stretched avatars**: Duration showing stretch worked
6. **Lip-synced avatars**: Duration matching audio duration
7. **Composited segments**: Video offset range (e.g., "0:30-1:00")

### REQUIRED Modal/Expand Controls

When clicking "Expand" on any video, the modal MUST have:

1. **Play/Pause button** - Click or spacebar to toggle
2. **Seek bar** - Drag to position in video
3. **Elapsed/Total time** - Display like "2:15 / 8:00"
4. **Fullscreen button** - Expand to full screen
5. **Close button** - X or Escape key to close

### Preview Sections

1. **Header**: Project name, source video path/duration, output directory, mode, refresh controls
2. **Status Bar**: Checkmarks (completed), spinners (in-progress), circles (pending)
3. **Step 1: Stills**: Grid with timestamp labels, click to expand
4. **Step 2: Scripts**: Grid with segment labels + YouTube Description box
5. **Step 3: Audio**: Players with duration display
6. **Step 4: Stretched**: Videos with duration annotations
7. **Step 5: Lip-Synced**: Videos WITH SOUND for validation
8. **Step 6: Composited**: Videos with offset range labels
9. **Step 7: Combined**: Final video player + YouTube Thumbnail

### Cache Busting

All resource URLs must have `?ts=<epoch>` suffix:
```javascript
const ts = () => `?ts=${Date.now()}`;
const url = `../audio/00-prolog.wav${ts()}`;
```

## TTS Commands

```bash
# Voice-cloned TTS with podcast CLI
podcast --host curiosity -p 7861 -v <voice_ref.wav> -d "<text>" -o <output.wav>

# Alternative with open-tts-rs
open-tts-rs --host curiosity -m vv -r "ref.wav;description" -g "text" -o output.wav
```

## Video Processing Commands

### Transparent Video Encoding
```bash
# PNG with alpha -> WebM VP9 with alpha
ffmpeg -y -framerate 30 -i "alpha/frame_%04d.png" \
    -c:v libvpx-vp9 -pix_fmt yuva420p -auto-alt-ref 0 -b:v 2M \
    -c:a libopus \
    output.webm
```

### Avatar Overlay Composite
```bash
# Overlay transparent avatar on clip
ffmpeg -y -i clip.mp4 -i avatar.webm \
    -filter_complex "[0:v]scale=1920:1080[bg];[1:v]scale=160:160[avatar];[bg][avatar]overlay=1740:900:shortest=1" \
    -map "[outv]" -map 1:a \
    composited.mp4
```

### Audio Padding
```bash
# Add 1s silence at start and end
ffmpeg -y -i input.wav -af "adelay=1000|1000,apad=pad_dur=1" output-padded.wav
```

## CRITICAL: Background Music Guidelines

**ALWAYS use `vid-music` for adding background music.** Never use bare ffmpeg.

### vid-music Defaults (Best Practices Built-In)

The `vid-music` tool has sensible defaults that enforce best practices:
- **Volume**: 30% (0.3) - music doesn't overpower
- **Fade in**: 0.5 seconds - smooth start
- **Fade out**: 1.5 seconds - smooth end

```bash
VID_MUSIC="/Users/mike/github/softwarewrighter/video-publishing/tools/target/release/vid-music"

# Basic usage - defaults handle volume and fades
$VID_MUSIC --input slide.mp4 --music track.mp3 --output slide-with-music.mp4

# Sample from specific point in music
$VID_MUSIC --input slide.mp4 --music track.mp3 --music-offset 15.0 --output slide-with-music.mp4

# Override defaults if needed
$VID_MUSIC --input slide.mp4 --music track.mp3 --volume 0.5 --fade-in 1.0 --fade-out 2.0 --output slide-with-music.mp4
```

### When to Use vid-music

| Segment Type | Use vid-music? | Notes |
|-------------|----------------|-------|
| Title slides | YES | Default 30% volume, fades |
| Transition slides | YES | e.g., "But First..." |
| Fast-forward (FF) | YES | Silent B-roll segments |
| Epilog slides | YES | CTA/links slides |
| OBS with avatar narration | NO | Avatar audio is the audio track |

**Key Rule**: Music is for silent segments only. Never layer music under narration.

## Project Directory Structure

Each pipeline run creates: `pipeline/YYYYMMDD/<project-name>/`

```
├── stills/              # Extracted frames (frame_NNNN.jpg)
├── scripts/             # Narration text (00-prolog.txt, 01-content.txt, 99-epilog.txt)
├── audio/               # TTS output (.wav, -padded.wav)
├── avatar/
│   ├── stretched/       # Time-stretched avatar videos
│   ├── lipsynced/       # After MuseTalk processing
│   ├── transparent/     # After rembg background removal (optional)
│   └── thumbs/          # Scaled for overlay (160x160)
├── clips/               # Cut segments from source video
├── composited/          # Final segments with avatar overlay
├── work/                # preview.html
├── thumbnail.jpg        # YouTube thumbnail
└── description.txt      # YouTube description with project URL
```

## Segment Naming Convention

- `00-prolog` - Introduction (plays over title card)
- `01-content` through `NN-content` - Main content at 30s intervals
- `99-epilog` - Conclusion/CTA (plays over source video ending)

## Known Narration Quality Issues

The current vision model approach generates descriptions with these problems:

### Bad Phrases to Avoid

| Bad Phrase | Problem | Should Be |
|------------|---------|-----------|
| "the user..." | Third-person for creator | "I" or "we" |
| "the developers..." | Third-person for creator | "Claude and I" or "we" |
| "likely", "possibly" | Uncertain language | Confident statements |
| "appears to be" | Hedging | Direct assertions |
| "IDE" | Wrong guess | Specific tool name (e.g., "Claude Code") |

### Style Problems

1. **Descriptive not explanatory** - Says "I see a terminal window" instead of "I'm asking Claude to implement..."
2. **Observer perspective** - Narrates like watching someone else's video, not the creator explaining their work
3. **Guessing technologies** - Without `--project-path`, the vision model guesses and is often wrong

### Proposed Solutions (Not Yet Implemented)

1. **Structured questions** - Ask vision model yes/no questions instead of open-ended "what do you see"
2. **Git context integration** - Match video timestamps to commit timestamps for accurate descriptions
3. **Two-stage LLM** - Vision model answers structured questions, text LLM synthesizes into first-person narration
4. **Project path requirement** - Always use `--project-path` to provide context about what's being shown

## Common Issues

| Issue | Solution |
|-------|----------|
| Lip-sync ends before audio | Add 1s audio padding before stretching |
| Lips move too fast | Avatar not stretched enough - use PADDED duration |
| MuseTalk CUDA OOM | Run lip-sync jobs sequentially (~4GB GPU per job) |
| Same avatar for all segments | Use `shuf -n 1` for random selection |
| Audio duration not showing | Use `ffprobe -show_entries format=duration` |
| Cached preview content | Add `?ts=<epoch>` to all resource URLs |
| Transparency not working | Verify `pix_fmt=yuva420p` in WebM output |
| Preview scripts not loading | Scripts must be in `../scripts/` relative to work/ |

## Configuration

Copy `config.template.sh` to your project and source it:
```bash
cp config.template.sh pipeline/20251221/my-project/config.sh
source pipeline/20251221/my-project/config.sh
```

Key settings: `TTS_HOST`, `TTS_MODEL`, `MUSETALK_SERVER`, `VOICE_REF`, `AVATAR_VIDEOS_DIR`
