# TN12 Minimal Covenant Spike

## Intent

Validate the absolute minimum covenant flow needed to support a future provably fair demo.

## Why this spike exists

Before implementing roulette, we need confidence that base primitives actually work:

- create a tiny covenant artefact,
- spend it,
- inspect it,
- and explain the observed behavior.

## Required local tooling

- Planned local tools: `git`, `node`, `npm`, `python3`, `cargo`, `rustc`, `codex`
- `scripts/check-env.sh` now reports `git`, `node`, `npm`, `python3`, `cargo`, `rustc`, and `codex` as present in this session (verify per session).
- Rust PATH visibility should be re-checked at the start of each new shell session.

## Non-goals

- No roulette mechanics.
- No web app.
- No gameplay economics or bankroll logic.
- No production deployment.

## Acceptance criteria

1. Documented command or API sequence for create/spend.
2. Captured raw outputs (hashes, txids, payloads where available).
3. Verification notes showing what was observed, what was assumed, and what remains unverified.

## Current status

- Status: planned, not yet executed.
- Target network: TN12/testnet.
- Next step: run the first full command sequence and record results in `findings.md`.

## How results are recorded

Update `findings.md` with:

- date/time,
- exact commands,
- outputs,
- success/failure,
- knowns,
- unknowns,
- and assumptions introduced.

## Next-step technical plan (minimum-route discovery only)

Goal for the next run: identify the smallest reproducible path to create/inspect a tiny artefact, without assuming any path works yet.

1. **Proposed next experiment (planned, no run yet):** read-only route-discovery only.
   - Since no local `silverscript`/`silver`/`ssc` command was found, do a metadata-only discovery for official SilverScript source/repo/package references (no clone/install).
   - Keep Rust, WASM, and Python checks read-only as secondary follow-up.
   - No build, compile, dependency install, repository clone, transaction submit, or covenant implementation in this step.

2. **Decision rule for first live experiment:**
   - If SilverScript metadata/actionability is confirmed, proceed with a constrained live SilverScript help/version/create-probe sequence.
   - If SilverScript metadata/actionability is not confirmed, pivot to Rusty Kaspa / Rust crates for the first live attempt.

3. **Secondary read-only checks (before any live step):**
   - Check feasibility of WASM SDK and Python SDK routes for payload/build/inspection support.

No path is treated as valid until a live command sequence is recorded in `findings.md` with outputs.