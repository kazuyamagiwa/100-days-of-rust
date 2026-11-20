# 100 Days of Rust

Daily tiny Rust programs. **Done beats impressive.** Goal: get started — not master Rust.

Other projects (and press-release weeks) come first. This challenge is designed to survive **5 minutes on a busy day**.

## Rules (minimal)

1. **Ship something that runs** — even 5–15 lines is a valid day.
2. **One folder per day** under `days/day-NNN-slug/`.
3. **Missed a day?** Ship a micro day the next time you can. No guilt reset.
4. **Busy / press week?** Use **Micro mode** below. Streak of “opened the repo” counts.

## Your time budget

| Mode | Your time | What you do |
|------|-----------|-------------|
| **Micro** | ~5 min | Ask the agent for “Day N micro”. Run once. Copy the LinkedIn blurb. Post. |
| **Normal** | ~15–30 min | Same, plus change one name/string or read the 3-line “learned” note. |
| **Curious** | optional | Only when you feel like it — ask “why does this line work?” |

You never owe Normal or Curious mode.

Posting is on **LinkedIn** (not X). Keep blurbs short: what you built + one learning. No need for polish on micro days.

## How to use the cloud agent

Ping with one of:

- `Day N` — default tiny finished app
- `Day N micro` — absolute minimum (5‑min floor)
- `press week` / `busy` — agent picks the smallest possible Day N

Agent delivers: runnable crate + short LinkedIn post text + one “I touched Rust today” bullet.

You: `cargo run -p day-NNN-…` (or the command in that day’s README) → post → stop.

## Layout

```
days/
  day-001-greet/
PROGRESS.md      # checkboxes only
```

## Day 001

```bash
cargo run -p day-001-greet
cargo run -p day-001-greet -- Kazu
cargo test -p day-001-greet
```
