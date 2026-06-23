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

- Status: env-008 clone/build probe executed.
- Target network: TN12/testnet.
- Next step: do not start transaction creation/spending in this task; prepare the first covenant command sequence for the next run using the probe results.

## How results are recorded

Update `findings.md` with:

- date/time,
- exact commands,
- outputs,
- success/failure,
- knowns,
- unknowns,
- and assumptions introduced.

## Next-step technical plan

Goal for the next run: based on the successful SilverScript probe, continue with minimal SilverScript command-sequence planning only.

1. **Next approved experiment (preliminary, non-transactional):**
   - Keep the clone in `external/silverscript`.
   - Use verified upstream test/build baseline from `env-008` as the starting point.
   - Identify (from upstream docs/examples) the smallest next command sequence for create/spend inspection, while continuing to avoid tx submission in this phase.

2. **Failure handling for reproducibility:**
   - If a future SilverScript build/test command is blocked, run an environment refresh/retry before changing route.

3. **Fallback plan (after a build failure only):**
   - If SilverScript remains blocked after a targeted retry, fall back to Rusty Kaspa / Rust crates for the first full create/spend path.

No path is treated as valid until a live command sequence is recorded in `findings.md` with outputs.