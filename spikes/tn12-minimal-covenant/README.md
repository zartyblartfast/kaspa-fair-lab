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

- Status: env-016/017/018 no-broadcast workflow + API discovery completed and documented in `findings.md`.
- env-013 fixture creation and local `cli-debugger --run-all` checks passed.
- env-014/015 added deeper local verifier passes and moved temporary fixture concepts into repo-owned fixtures.
- env-016 introduced `run_no_broadcast_checks.sh`.
- env-017 executed `run_no_broadcast_checks.sh` and confirmed all three checks pass, with output logs captured in `spikes/tn12-minimal-covenant/artifacts/`.
- env-018 validated the Rust tx-construction API route (no-broadcast): `Transaction::new`, `PopulatedTransaction::new`, `sign_with_multiple_v2`, and version-aware serde paths are present in repo-owned sources.
- Target network: TN12/testnet.
- No live submit/broadcast steps yet.

## How results are recorded

Update `findings.md` with:

- date/time,
- exact commands,
- outputs,
- success/failure,
- knowns,
- unknowns,
- and assumptions introduced.

## How to rerun

Run `./spikes/tn12-minimal-covenant/run_no_broadcast_checks.sh` from the repo root (or anywhere in this repo).

## Next-step technical plan

Goal for the next run: move from verification-only tests to explicit no-broadcast transaction-construction planning (still no wallet or broadcast).

1. **Done (this phase):** repo-owned fixture files are in place under
   `spikes/tn12-minimal-covenant/fixtures/` and were executed via `run_no_broadcast_checks.sh`.
   - Add a minimal fixture bundle for:
     - transition-style local simulation
     - explicit tx-structured simple_covenant check
   - No external dependencies or broadcast steps.

2. **Run the reproducible repo-owned workflow** (repo-only, no broadcast):

   From the external clone:

   ```bash
   cd /root/kaspa-fair-lab/external/silverscript

   /root/.cargo/bin/cargo run -p cli-debugger -- \
     /root/kaspa-fair-lab/spikes/tn12-minimal-covenant/fixtures/simple_covenant.sil \
     --run-all \
     --test-file /root/kaspa-fair-lab/spikes/tn12-minimal-covenant/fixtures/simple_covenant.test.json

   /root/.cargo/bin/cargo run -p cli-debugger -- \
     /root/kaspa-fair-lab/spikes/tn12-minimal-covenant/fixtures/tn12_demo_transition.sil \
     --run-all \
     --test-file /root/kaspa-fair-lab/spikes/tn12-minimal-covenant/fixtures/tn12_demo_transition.test.json
   ```

   Optional explicit tx-version check (same contract):

   ```bash
   /root/.cargo/bin/cargo run -p cli-debugger -- \
     /root/kaspa-fair-lab/spikes/tn12-minimal-covenant/fixtures/simple_covenant.sil \
     --run-all \
     --test-file /root/kaspa-fair-lab/spikes/tn12-minimal-covenant/fixtures/simple_covenant_tx_structured.test.json
   ```

   Expected result: all tests in each fixture report PASS locally with output entries like `RUN ...` and `PASS ...`.

3. **Next recommendation:** proceed with explicit route planning for no-broadcast transaction construction (no live submit):
   - keep this script path as the canonical verifier baseline,
   - confirm whether a repo-owned Rust `Transaction`/`PopulatedTransaction` assembly path is needed for signed payload output.

Recommended now: keep using this script-based local verifier path as the canonical baseline, and open a follow-up task that wires a Rust/tx-assembly path (e.g., `Transaction::new` + `PopulatedTransaction`) to generate and log concrete signed transaction blobs for future create/spend/inspect testing.

No path is considered valid until repo-owned no-broadcast evidence is recorded in `findings.md` with outputs.

### Env-019 short next-step checklist

- Keep `run_no_broadcast_checks.sh` as the canonical evidence baseline.
- Add a Rust-only, local tx-assembly planning pass (no broadcast, no wallet/faucet/mainnet).
- Capture in `findings.md`:
  - tx assembly API route chosen,
  - mock key strategy,
  - transaction context fields emitted (version, inputs, outputs, covenant ids),
  - serialization/signature output form.
- After env-019 evidence is complete, update `README.md` + `findings.md` and request explicit go-ahead for env-020 (no-broadcast signed payload proof).
