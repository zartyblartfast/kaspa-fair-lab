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

1. **Proposed next experiment (planned, no run yet):** controlled SilverScript metadata + clone/build readiness probe in an isolated location.
   - Confirmed local command probes still show no local `silverscript` binary (`command -v silverscript`, `silver`, `ssc` all missing from PATH).
   - Execute only read-only metadata checks now.
   - Proposed isolated location for clone/build: `external/` (or `spikes/tn12-minimal-covenant/vendor/`), but **do not clone yet**.

2. **Candidate first live experiment (when approved):**
   - Clone (or reuse) official upstream metadata source into the isolated location.
   - Run `cargo test -p silverscript-lang` as the first build/test check.
   - Run the documented debugger invocation as the first execution check:
     - `cargo run -p cli-debugger -- silverscript-lang/tests/examples/if_statement.sil --function hello --ctor-arg 3 --ctor-arg 10 --arg 1 --arg 2`
   - Mark both commands as **UNVERIFIED** until actually executed in a follow-up task.

3. **Decision rule for first live experiment:**
   - If official SilverScript route discovery remains intact and clone/build succeeds, prioritize the SilverScript probe first.
   - If SilverScript is not actionable in this environment, pivot to Rusty Kaspa / Rust crates for the first live attempt.

4. **Secondary read-only checks (before any build/tx):**
   - Check feasibility of WASM SDK and Python SDK routes for payload/build/inspection support.

No path is treated as valid until a live command sequence is recorded in `findings.md` with outputs.