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

- Status: env-028 local feasibility conclusion completed and documented in `findings.md`.
- SilverScript builds locally.
- `simple_covenant.sil` compiles locally.
- repo-owned local fixtures pass.
- `run_no_broadcast_checks.sh` passes, with output logs captured in `spikes/tn12-minimal-covenant/artifacts/`.
- local `Transaction` construction, `RpcTransaction` conversion, `SubmitTransactionRequest` construction, RPC serializer artifact production, and RPC serializer round-trip verification are all documented in `findings.md`.
- deterministic local Borsh artifact production is documented, but consensus-wire equivalence remains unverified.
- Target network remains TN12/testnet only.
- No RPC client was called.
- No signing was performed.
- No live submit/broadcast steps were attempted.
- No live TN12 create/spend/inspect lifecycle has been proven.

## Env-028 local feasibility conclusion

Local tooling is now credible enough to plan a controlled TN12 experiment, but not enough to claim live TN12 create/spend/inspect works.

What is proven locally:
- SilverScript builds.
- simple covenant compiles.
- repo-owned local fixtures pass.
- `run_no_broadcast_checks.sh` passes.
- local `Transaction` / `RpcTransaction` / `SubmitTransactionRequest` object paths work.
- RPC serializer artifacts and local round-trip checks pass.

What is still unproven:
- signing,
- real UTXO use,
- faucet funding,
- live TN12 RPC submission,
- mempool acceptance,
- spend/inspect lifecycle,
- mainnet behaviour.

Readiness checklist before any live TN12 step:
- use test-only wallet/key,
- isolate test funds,
- identify TN12 RPC endpoint or local node,
- confirm faucet process,
- confirm transaction version/covenant expectations,
- decide whether the first live step is read-only RPC connectivity or faucet/address setup,
- require explicit manual approval before any broadcast.

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

### Env-019 next-step plan

Recommended next-step now:

- keep the spike constrained to local no-broadcast assembly only,
- use the new scaffold at `spikes/tn12-minimal-covenant/rust-tx-assembly/`,
- keep `run_no_broadcast_checks.sh` as the canonical verifier baseline,
- after this scaffold confirms minimal construction, proceed to a follow-up that emits deterministic transaction plan fields (and later signed payloads) without wallets, faucets, network submit, or broadcast.

### Env-019 short next-step checklist

- Keep `run_no_broadcast_checks.sh` as the canonical evidence baseline.
- Add a Rust-only, local tx-assembly planning pass (no broadcast, no wallet/faucet/mainnet).
- Capture in `findings.md`:
  - tx assembly API route chosen,
  - mock key strategy,
  - transaction context fields emitted (version, inputs, outputs, covenant ids),
  - serialization/signature output form.
- After env-019 evidence is complete, update `README.md` + `findings.md` and request explicit go-ahead for env-020 (no-broadcast signed payload proof).
