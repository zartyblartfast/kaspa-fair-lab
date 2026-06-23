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

- Status: env-010 compile-to-artifact and env-011 local covenant-workflow discovery completed.
- env-012 local `cli-debugger --run-all` check discovered no upstream `simple_covenant.test.json` fixture; run failed with missing test file.
- Target network: TN12/testnet.
- Status: env-013 fixture creation and local `cli-debugger` run completed with PASS; no live submit/broadcast steps yet.
- Next step: proceed to no-broadcast TN12 transaction-construction planning (create/spend/inspect evidence flow).

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

Goal for the next run: use the documented local verifier path to validate `simple_covenant` semantics without network submission.

1. **Next approved step completed (local simulation only):**
   - Created `spikes/tn12-minimal-covenant/fixtures/simple_covenant.test.json`.
   - Ran:
     - `cargo run -p cli-debugger -- silverscript-lang/tests/examples/simple_covenant.sil --run-all --test-file /root/kaspa-fair-lab/spikes/tn12-minimal-covenant/fixtures/simple_covenant.test.json`
   - Result captured in `spikes/tn12-minimal-covenant/findings.md`:
     - `1 tests: 1 passed, 0 failed`

2. **Failure handling for reproducibility (if needed):**
   - If the command fails in a future attempt, re-run once in a clean shell context before changing route.

3. **Planned next step (post-local fixture pass):**
   - Move to no-broadcast TN12 transaction-construction planning for create/spend/inspect semantics, using simulator/fixtures as the evidence source.
   - Keep this strictly non-submitting and TN12-only.

No path is treated as valid until no-broadcast transaction construction evidence is recorded in `findings.md` with outputs.
