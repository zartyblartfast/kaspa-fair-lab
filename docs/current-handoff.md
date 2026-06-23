# TN12 Spike Handoff

## Current project goal
Continue TN12 minimal covenant spike route discovery with documentation-first evidence, while staying constrained to:

- no roulette build,
- no web app,
- no own covenant implementation,
- no transaction submission,
- no mainnet usage.

## Concise status update

1) env-034 refined the localhost-only TN12 startup command and expected ports; all live actions remain pending.

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

5b) env-034 localhost-only node command refinement status:
- current blocker:
  - no public TN12 endpoint found in checked docs/source
  - no local kaspad/kaspa CLI tooling installed in PATH
  - no local kaspad process running
  - read-only `getServerInfo` cannot be run yet
- refined command conclusion:
  - TN12 requires `--testnet --netsuffix=12`; `--testnet` alone is insufficient
  - recommended minimal localhost-only command: `cargo run --release --bin kaspad -- --testnet --netsuffix=12 --disable-upnp --listen=127.0.0.1:16311 --rpclisten=127.0.0.1:16210 --rpclisten-borsh=127.0.0.1:17210`
  - `--utxoindex` is not required for the first read-only `getServerInfo` check
  - optional only if needed later: `--utxoindex` and/or `--rpclisten-json=127.0.0.1:18210`
- conservative next action:
  - prepare localhost-only startup + log capture plan, but do not start `kaspad` yet
  - after explicit approval, run exactly one read-only `getServerInfo`, capture output, then stop before wallet/faucet/signing/broadcast

6) Information required before any live step:
- approval to start a local testnet node,
- confirmation of localhost-only bind vs any exposed listen address,
- confirmed TN12 selector and ports (`--testnet --netsuffix=12`, `16311`, `16210`, `17210`, optional `18210`),
- safe read-only RPC command/API path,
- logging/artifact paths (`spikes/tn12-minimal-covenant/artifacts/env-034-kaspad-startup.log` and `spikes/tn12-minimal-covenant/artifacts/env-034-get-server-info.txt`),
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

`Continue env-034 planning only: confirm the refined localhost-only TN12 local node startup command (--testnet --netsuffix=12), keep execution disabled, and require explicit manual approval before any node start or read-only RPC call.`
