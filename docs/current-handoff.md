# TN12 Spike Handoff

## Current project goal
Continue TN12 minimal covenant spike route discovery with documentation-first evidence, while staying constrained to:

- no roulette build,
- no web app,
- no own covenant implementation,
- no transaction submission,
- no mainnet usage.

## Concise status update

1) env-030 added TN12 read-only RPC connectivity planning to the spike docs (after env-029).

2) Current repo-backed local evidence still covers:
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
- The next safe move is read-only TN12 connectivity/discovery, not transaction creation.
- Local tooling is not yet sufficient to claim live TN12 create/spend/inspect works.

5) Recommended first live step after `/new`:
- read-only TN12 RPC connectivity check only (candidate first call: `getServerInfo` / `get_server_info_call(None, GetServerInfoRequest {})`),
- no wallet,
- no faucet,
- no signing,
- no transaction submission.

6) Information required before any live step:
- TN12 RPC endpoint or local node path,
- network selector/name,
- expected node version/Toccata/TN12 status,
- safe read-only RPC command to call,
- exact read-only method/API path,
- logging/artifact path,
- explicit stop condition before any state-changing action.

7) Manual approval gates:
- approval before wallet/key creation,
- approval before faucet request,
- approval before signing,
- approval before broadcast.

## Branch / repo status

- Repo: `/root/kaspa-fair-lab`
- Branch: `main` (`origin/main`)
- Modified files:
  - `docs/current-handoff.md`
  - `spikes/tn12-minimal-covenant/README.md`
  - `spikes/tn12-minimal-covenant/findings.md`

## Suggested first prompt after /new

`Plan the first live TN12 prerequisite step only: identify the TN12 RPC endpoint/local node path, confirm faucet/address setup expectations, keep the plan test-only, and require explicit manual approval before any broadcast.`
