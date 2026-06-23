# TN12 Spike Handoff

## Current project goal
Continue TN12 minimal covenant spike route discovery with documentation-first evidence, while staying constrained to:

- no roulette build,
- no web app,
- no own covenant implementation,
- no transaction submission,
- no mainnet usage.

## Concise status update

1) env-033 added local TN12 node startup planning to the spike docs; all live actions remain pending.

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

5) env-031 next live-planning step identified (read-only only):
- execution mode: one-call read-only TN12 endpoint probe via `getServerInfo` (`get_server_info_call(None, GetServerInfoRequest {})`)
- safest path: tiny Rust probe client once endpoint is approved (no wallet/state/signing/broadcast),
- no local `kaspad`/CLI executable is currently in PATH,
- repository/docs scan confirms local node startup command exists but has no explicit public TN12 endpoint URL.

5b) env-033 local node startup planning status:
- current blocker:
  - no public TN12 endpoint found in checked docs/source
  - no local kaspad/kaspa CLI tooling installed in PATH
  - no local kaspad process running
  - read-only `getServerInfo` cannot be run yet
- candidate local-node path:
  - use existing `external/silverscript` / `rusty-kaspa` source if suitable
  - or use pinned `rusty-kaspa` source already present in Cargo cache if suitable
  - command candidate from docs: `cargo run --release --bin kaspad -- --testnet --utxoindex`
  - `rpclisten` / `borsh` / wRPC flags and exact ports still require confirmation before execution
- conservative next action:
  - prepare localhost-only startup + log capture plan, but do not start `kaspad` yet
  - after explicit approval, run exactly one read-only `getServerInfo`, capture output, then stop before wallet/faucet/signing/broadcast

6) Information required before any live step:
- approval to start a local testnet node,
- confirmation of localhost-only bind vs any exposed listen address,
- confirmed `kaspad` flags/ports for wRPC / Borsh / RPC surfaces,
- safe read-only RPC command/API path,
- logging/artifact paths (`spikes/tn12-minimal-covenant/artifacts/env-033-node-startup.log` and `spikes/tn12-minimal-covenant/artifacts/env-033-get-server-info.txt`),
- explicit stop condition before any state-changing action.

7) Manual approval gates:
- approval to start a local testnet node,
- approval to expose/listen on any port beyond localhost,
- approval to run one read-only `getServerInfo`,
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

`Continue env-033 planning only: confirm the safest localhost-only TN12 local node startup command candidate, identify unresolved wRPC/Borsh/listen flags and ports, keep execution disabled, and require explicit manual approval before any node start or read-only RPC call.`
