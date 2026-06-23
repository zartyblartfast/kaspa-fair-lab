# TN12 Spike Handoff

## Current project goal
Continue TN12 minimal covenant spike route discovery with documentation-first evidence, while staying constrained to:

- no roulette build,
- no web app,
- no own covenant implementation,
- no transaction submission,
- no mainnet usage.

## Concise status update

1) env-035 attempted the approved localhost-only TN12 node startup, but the node never reached RPC readiness because the local release build is blocked by missing `protoc` (after an initial bindgen header-path issue was worked around); no live `getServerInfo` call was executed.

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

5b) env-035 localhost-only node startup attempt status:
- attempted command:
  - `cargo run --release --bin kaspad -- --testnet --netsuffix=12 --disable-upnp --listen=127.0.0.1:16311 --rpclisten=127.0.0.1:16210 --rpclisten-borsh=127.0.0.1:17210`
- localhost-only bind policy held:
  - no `0.0.0.0` listen flags were used
  - configured addresses stayed `127.0.0.1:16311`, `127.0.0.1:16210`, and `127.0.0.1:17210`
- startup blockers observed:
  - first failure: `librocksdb-sys` bindgen could not find `stdbool.h`
  - retry with `LIBCLANG_PATH=/usr/lib/llvm-18/lib` and `BINDGEN_EXTRA_CLANG_ARGS="-isystem /usr/lib/gcc/x86_64-linux-gnu/13/include -isystem /usr/include"` got further
  - final blocker before env-036: `kaspa-p2p-lib` protobuf compilation failed because `protoc` was not available in this environment
- result:
  - no local kaspad process reached RPC readiness
  - no live `getServerInfo` call was executed
  - artifacts: `spikes/tn12-minimal-covenant/artifacts/env-035-kaspad-startup.log` and `spikes/tn12-minimal-covenant/artifacts/env-035-get-server-info.txt`

5c) env-036 local build prerequisite resolution:
- approved scope executed:
  - checked `protoc --version`
  - ran `apt-get update`
  - installed minimum package `protobuf-compiler`
  - rechecked `protoc --version`
- observed result:
  - `protoc` was missing before install (`command not found`)
  - `protobuf-compiler` installed successfully
  - `protoc --version` now returns `libprotoc 3.21.12`
  - bindgen workaround should still be preserved for the next local build retry:
    - `LIBCLANG_PATH=/usr/lib/llvm-18/lib`
    - `BINDGEN_EXTRA_CLANG_ARGS="-isystem /usr/lib/gcc/x86_64-linux-gnu/13/include -isystem /usr/include"`
- scope confirmations:
  - `kaspad` was not started
  - no RPC endpoint was called
  - no wallet/key/faucet/signing/broadcast occurred
- conservative next action:
  - rerun the same approved localhost-only TN12 startup command later with the bindgen workaround preserved
  - stop again before any wallet/faucet/signing/broadcast work

6) Information required before any live step:
- approval to start a local testnet node,
- confirmation of localhost-only bind vs any exposed listen address,
- confirmed TN12 selector and ports (`--testnet --netsuffix=12`, `16311`, `16210`, `17210`, optional `18210`),
- safe read-only RPC command/API path,
- logging/artifact paths (`spikes/tn12-minimal-covenant/artifacts/env-035-kaspad-startup.log` and `spikes/tn12-minimal-covenant/artifacts/env-035-get-server-info.txt`),
- environment build prerequisites for local startup (`protoc`, plus bindgen header-path settings if needed),
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

`Continue env-035 from the recorded failed localhost-only TN12 startup attempt: keep the TN12 selector/ports unchanged, resolve the local build blockers (protoc and any required bindgen header-path environment), then rerun the same one-call read-only startup plan without adding wallet/faucet/signing/broadcast work.`
