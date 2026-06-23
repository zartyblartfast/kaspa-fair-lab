# TN12 Spike Handoff

## Current project goal
Continue TN12 minimal covenant spike route discovery with documentation-first evidence, while staying constrained to:

- no roulette build,
- no web app,
- no own covenant implementation,
- no transaction submission,
- no mainnet usage.

## Concise status update

1) env-028 added the local feasibility conclusion and TN12 readiness checklist to the spike docs.

2) Current repo-backed local evidence now covers:
- SilverScript builds locally.
- `simple_covenant.sil` compiles.
- repo-owned local fixtures pass.
- `run_no_broadcast_checks.sh` passes.
- local `Transaction` construction is documented.
- deterministic local Borsh artifact production is documented, with consensus-wire equivalence still unverified.
- local `RpcTransaction` conversion works.
- local `SubmitTransactionRequest` construction works.
- local RPC serializer artifacts were produced.
- local RPC serializer round-trip passes for both `RpcTransaction` and `SubmitTransactionRequest`.

3) Scope limits still in force and still unproven:
- no RPC client was called,
- nothing was signed,
- nothing was broadcast,
- no live TN12 create/spend/inspect was attempted,
- no real UTXO/faucet path was exercised,
- no mainnet behaviour is known.

4) Conservative conclusion:
- Local tooling is credible enough to plan a controlled TN12 experiment.
- Local tooling is not yet sufficient to claim live TN12 create/spend/inspect works.

5) Recommended next task after `/new`:
- stay documentation-first and plan the first live TN12 prerequisite step only,
- use a test-only wallet/key,
- isolate test funds,
- identify the TN12 RPC endpoint or local node,
- confirm faucet process,
- confirm transaction version/covenant expectations,
- decide whether the first approved live step is read-only RPC connectivity or faucet/address setup,
- require explicit manual approval before any broadcast.

## Branch / repo status

- Repo: `/root/kaspa-fair-lab`
- Branch: `main` (`origin/main`)
- Modified files:
  - `docs/current-handoff.md`
  - `spikes/tn12-minimal-covenant/README.md`
  - `spikes/tn12-minimal-covenant/findings.md`

## Suggested first prompt after /new

`Plan the first live TN12 prerequisite step only: identify the TN12 RPC endpoint/local node path, confirm faucet/address setup expectations, keep the plan test-only, and require explicit manual approval before any broadcast.`
