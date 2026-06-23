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

- Status: env-029 TN12 prerequisite planning completed and documented in `findings.md`.
- SilverScript builds locally.
- `simple_covenant.sil` compiles locally.
- repo-owned local fixtures pass.
- `run_no_broadcast_checks.sh` passes, with output logs captured in `spikes/tn12-minimal-covenant/artifacts/`.
- local `Transaction` construction, `RpcTransaction` conversion, `SubmitTransactionRequest` construction, RPC serializer artifact production, and RPC serializer round-trip verification are all documented in `findings.md`.
- deterministic local Borsh artifact production is documented, but consensus-wire equivalence remains unverified.
- Target network remains TN12/testnet only.
- No RPC client was called.
- No signing was performed.
- No real UTXO was used.
- No faucet funding was used.
- No live submit/broadcast steps were attempted.
- No live TN12 create/spend/inspect lifecycle has been proven.

## Env-029 prerequisite planning conclusion

Recommended first live step:
- read-only TN12 RPC connectivity/discovery only.
- no wallet.
- no faucet.
- no signing.
- no transaction submission.

Candidate first live step options:
- read-only TN12 RPC connectivity check.
- test-only address/key generation.
- faucet/address setup.
- no-broadcast signed local transaction construction.
- live submission/broadcast later only with explicit manual approval.

Information needed before any live step:
- TN12 RPC endpoint or local node path.
- network selector/name.
- expected node version/Toccata/TN12 status.
- safe read-only RPC command to call.
- logging/artifact path.
- explicit stop condition before any state-changing action.

Manual approval gates:
- approval before wallet/key creation.
- approval before faucet request.
- approval before signing.
- approval before broadcast.

Conservative conclusion:
- The next safe move is read-only TN12 connectivity/discovery, not transaction creation.

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

Goal for the next run: perform TN12 prerequisite discovery only, starting with a read-only connectivity/discovery check if and only if the required live inputs are provided and explicitly approved.

1. Confirm the minimum live prerequisites before any command is run:
   - TN12 RPC endpoint or local node path.
   - network selector/name.
   - expected node version/Toccata/TN12 status.
   - one safe read-only RPC command.
   - artifact/log path.
   - explicit stop condition before any state-changing action.

2. First approved live action should stay read-only:
   - call only the agreed read-only TN12 RPC connectivity/discovery command,
   - collect raw output to the chosen artifact/log path,
   - stop immediately after confirming connectivity/version/status information.

3. Do not proceed further in the same run without fresh manual approval for each escalation:
   - wallet/key creation,
   - faucet request,
   - signing,
   - broadcast.

Recommended now: keep `run_no_broadcast_checks.sh` and the local Rust object/serializer evidence as the canonical baseline, and treat read-only TN12 connectivity/discovery as the only safe first live step.

No path is considered valid until repo-owned evidence is recorded in `findings.md` with exact commands, outputs, and explicit stop conditions.
