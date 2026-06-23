# TN12 Spike Handoff

## Current project goal
Continue TN12 minimal covenant spike route discovery with documentation-first evidence, then run the first constrained live path to demonstrate a minimal create/spend/inspect covenant flow.

## Current status
- `env-006` SilverScript read-only live probe completed and recorded in `spikes/tn12-minimal-covenant/findings.md`.
- No local SilverScript tooling command is available (`silverscript`, `silver`, `ssc`).
- `README.md` and `findings.md` now encode the next-step decision rule and constraints.
- No implementation, roulette work, web app work, or transaction submission has been done.

## What has been committed / ready to commit
- Working tree currently has one new uncommitted file: `docs/current-handoff.md`.
- Last commit on `main`:
  - `a8ccfad` — "Record SilverScript read-only probe"
- Other TN12 documentation is committed; no other local spike docs were modified in this session before adding this handoff file.

## Environment status
- `./scripts/check-env.sh` passes (run at `2026-06-23T13:58:25Z`):
  - `git`, `node`, `npm`, `python3`, `cargo`, `rustc`, `codex` all present.

## Latest finding
- `command -v silverscript` -> not found
- `command -v silver` -> not found
- `command -v ssc` -> not found

## Important correction
- The missing local SilverScript commands only mean no CLI is installed on PATH.
- This does **not** prove SilverScript is unusable; it may require cloning and building the official Rust workspace to become available.

## Current recommended next step
- Do a controlled SilverScript metadata acquisition + clone/build/readme experiment (documentation-guided and constrained) before committing to an alternative route.
- Keep all outputs command-based and mark any unverified assumptions explicitly as **unverified**.

## Constraints for the next session
- No roulette yet.
- No web app.
- No covenant implementation.
- No transaction submission.
- No mainnet assumptions.
- Any unverified item must remain labeled **unverified**.

## Suggested first prompt after /new
- "Please continue TN12 from `docs/current-handoff.md`, then run a constrained SilverScript source discoverability + local clone/build/readme-readiness probe only (no install/tx/mainnet/webapp/roulette), and append live outputs to `spikes/tn12-minimal-covenant/findings.md`."