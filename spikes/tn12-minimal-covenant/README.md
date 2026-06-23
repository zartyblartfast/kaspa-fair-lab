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
- env-013 fixture creation and local `cli-debugger` run completed with PASS; no live submit/broadcast steps yet.
- env-014 recorded additional no-broadcast local evidence (unit and `cli-debugger` checks) in `findings.md`.
- Next step: create a reproducible repo-owned local no-broadcast workflow before any TN12 wallet/faucet/network prerequisite planning.

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

Goal for the next run: convert the current temporary-only evidence into a repository-owned, reproducible no-broadcast local workflow.

1. **Create repo-owned fixture files first** (no network):
   - Keep fixture sources under `spikes/tn12-minimal-covenant/fixtures/` (for example, move the temporary fixture concepts used in previous `/tmp` runs).
   - Add a minimal fixture bundle for:
     - transition-style local simulation
     - explicit tx-structured simple_covenant check
   - No external dependencies or broadcast steps.

2. **Add a reproducible local command sequence** (README or tiny shell script):
   - Use explicit absolute/relative paths in repo.
   - Sequence should run `cli-debugger --run-all` for each repo-owned fixture and capture outputs in command output/docs.
   - Keep the sequence strictly no-broadcast and TN12-scope.

3. **Only after the above is complete**, plan TN12 wallet/faucet/network prerequisites:
   - document the wallet/tooling/network assumptions separately.
   - avoid execution or claims of live create/spend/inspect until those prerequisites are defined and evidence is collected.

No path is treated as valid until repo-owned no-broadcast evidence is recorded in `findings.md` with outputs.
