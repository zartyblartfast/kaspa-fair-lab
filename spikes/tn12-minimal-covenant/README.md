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
- Next step: continue simulation-first by inspecting upstream covenant `.test.json` examples and then planning a local fixture for `simple_covenant.sil` in a follow-up step.

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

1. **Next approved step (local simulation only):**
   - Continue to inspect upstream covenant test examples for `--run-all` schema and `test` payloads (especially in `debugger/cli/tests/cli_tests.rs`) and then decide the `simple_covenant` fixture shape.
   - Do not create `simple_covenant.test.json` in this turn.
   - Once a valid fixture is available, run:
     - `cargo run -p cli-debugger -- silverscript-lang/tests/examples/simple_covenant.sil --run-all --test-file <path-to-test-json>`
   - Capture:
     - full command line,
     - PASS/FAIL summary,
     - any failure context and the fixture inputs that drove it.
   - Keep this strictly non-submitting and TN12-only.

2. **Failure handling for reproducibility:**
   - If the local command still fails once a fixture exists, re-run once in a clean shell context before changing route.

3. **Fallback plan (after local sim failure only):**
   - If SilverScript simulator tooling remains blocked, pivot to Rust-level constructs (`covenant_decl_sigscript`, `execute_input_with_covenants`) only for pure local spending validation before any live workflow.

No path is treated as valid until a live command sequence is recorded in `findings.md` with outputs.
