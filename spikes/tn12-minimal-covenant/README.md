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

- Status: env-036 resolved the local `protoc` build prerequisite by installing `protobuf-compiler` and verifying `protoc --version` -> `libprotoc 3.21.12`; the bindgen workaround from env-035 remains needed for later `kaspad` build retries.
- SilverScript builds locally.
- `simple_covenant.sil` compiles locally.
- repo-owned local fixtures pass.
- `run_no_broadcast_checks.sh` passes, with output logs captured in `spikes/tn12-minimal-covenant/artifacts/`.
- local `Transaction` construction, `RpcTransaction` conversion, `SubmitTransactionRequest` construction, RPC serializer artifact production, and RPC serializer round-trip verification are all documented in `findings.md`.
- deterministic local Borsh artifact production is documented, but consensus-wire equivalence remains unverified.
- Target network remains TN12/testnet only.
- One read-only local TN12 `getServerInfo` call succeeded in env-037, with output captured in `spikes/tn12-minimal-covenant/artifacts/env-037-get-server-info.txt`.
- No signing was performed.
- No real UTXO was used.
- No faucet funding was used.
- No live submit/broadcast steps were attempted.
- No live TN12 create/spend/inspect lifecycle has been proven.

## Env-037 local TN12 read-only getServerInfo retry

- **Scope:** approved localhost-only TN12 node startup retry, capture startup logs, run exactly one read-only `getServerInfo` call once RPC was reachable, capture the output, and stop before any wallet/key/faucet/signing/submission/broadcast action.

### Exact startup command used

- `cargo run --release --bin kaspad -- --testnet --netsuffix=12 --disable-upnp --listen=127.0.0.1:16311 --rpclisten=127.0.0.1:16210 --rpclisten-borsh=127.0.0.1:17210`

### Localhost-only bind check

- P2P listen: `127.0.0.1:16311`
- gRPC listen: `127.0.0.1:16210`
- wRPC Borsh listen: `127.0.0.1:17210`
- No `0.0.0.0` listen flag was used.

### RPC/read-only result

- Exact read-only RPC call used: one gRPC `getServerInfo` call against `grpc://127.0.0.1:16210`.
- Returned server/network fields:
  - `serverVersion`: `1.1.1-toc.1`
  - `networkId`: `testnet-12`
  - `rpcApiVersion`: `1`
  - `rpcApiRevision`: `0`
  - `hasUtxoIndex`: `false`
  - `isSynced`: `false`
  - `virtualDaaScore`: `0`

### Artifacts

- startup log: `spikes/tn12-minimal-covenant/artifacts/env-037-kaspad-startup.log`
- read-only result: `spikes/tn12-minimal-covenant/artifacts/env-037-get-server-info.txt`

### Result

- Node startup succeeded to the point of exposing the approved localhost-only RPC surfaces.
- Exactly one read-only `getServerInfo` call succeeded.
- The node was then stopped without any wallet/key/faucet/signing/submission/broadcast activity.

### Scope confirmations

- wallet/key created: false
- faucet request made: false
- anything signed: false
- anything submitted/broadcast: false
- stop condition reached: stopped immediately after capturing the single read-only `getServerInfo` result

## Env-036 local build prerequisite resolution

- **Scope:** local build prerequisite resolution only; no node start, no RPC call, no wallet/key/faucet/signing/broadcast work.

### Commands run

- `protoc --version`
- `apt-get update`
- `apt-get install -y protobuf-compiler`
- `protoc --version`

### Result

- `protoc` was not present before installation (`protoc: command not found`).
- Installed package: `protobuf-compiler`.
- Verified after install: `libprotoc 3.21.12`.
- The env-035 bindgen workaround still remains relevant for later local `kaspad` build attempts:
  - `LIBCLANG_PATH=/usr/lib/llvm-18/lib`
  - `BINDGEN_EXTRA_CLANG_ARGS="-isystem /usr/lib/gcc/x86_64-linux-gnu/13/include -isystem /usr/include"`

### Scope confirmations

- `kaspad` was not started.
- No RPC endpoint was called.
- No wallet was created.
- No keys were generated.
- No faucet request was made.
- Nothing was signed.
- Nothing was submitted or broadcast.

### Recommended next action

- Rerun the same approved localhost-only TN12 node startup attempt later, with the bindgen workaround preserved, and stop again before any wallet/faucet/signing/broadcast activity.

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

## Env-030 TN12 read-only RPC connectivity planning

- **Scope:** planning-only documentation update; no live RPC call executed yet.

### Candidate connectivity options

- local TN12 node, if available
- public/community TN12 RPC endpoint, if documented
- Rusty Kaspa CLI/RPC client path, if available
- custom tiny Rust RPC client (later fallback)

### Candidate safe read-only calls

- getServerInfo
- getBlockDagInfo
- getSyncStatus
- getCurrentNetwork
- getInfo
- getConnectedPeerInfo

### Recommended first live step

- one read-only RPC call only: getServerInfo
- no wallet, faucet, signing, submit, or broadcast
- log output to spikes/tn12-minimal-covenant/artifacts/env-030-readonly-rpc-connectivity-log.jsonl

### Required before executing

- endpoint URL or local node command/path
- TN12/testnet network confirmation
- expected TN12/Toccata status
- exact read-only command
- artifact path
- explicit manual approval

### Stop conditions

- stop if endpoint is unknown
- stop if network is not clearly TN12/testnet
- stop if command is not clearly read-only
- stop before wallet/key creation
- stop before faucet request
- stop before signing
- stop before broadcast

## Env-031 read-only TN12 getServerInfo execution plan

### Decision

- Existing local Rusty Kaspa CLI/client: **not detected** in local PATH/repo context.
- Node command/path: **not documented** in repo yet.
- Required approach for now: **tiny Rust probe client** once endpoint is provided.

### Execution plan for env-031

- Endpoint required before run: TN12/testnet RPC URL or local node path (not yet known in docs).
- Candidate safe call: `get_server_info_call(None, GetServerInfoRequest {})`.
- Log destination: `spikes/tn12-minimal-covenant/artifacts/env-031-get-server-info.txt`.
- Manual approval required before running (endpoint + command confirmation).

### Stop conditions

- Stop if endpoint is unknown.
- Stop if response does not confirm TN12/testnet.
- Stop immediately after the single read-only response is captured.

## Env-032 TN12 endpoint identification

### Decision and evidence

- Local node command/path check (this host):
  - `command -v kaspad` -> not found
  - `command -v kaspa-cli` -> not found
  - `command -v kaspa-rpc` -> not found
  - `command -v kaspa-grpc` -> not found
  - `command -v kaspactl` -> not found
- No running local kaspa node process was detected.
- Repo/source/docs scan did **not** find a ready-to-run executable path for TN12 RPC in this environment.

### Official/local node source references

- Kaspa docs (`content/docs/integrate/kaspa-node.mdx`) documents local node startup:
  - `cargo run --release --bin kaspad -- --utxoindex --rpclisten=0.0.0.0 --rpclisten-borsh=0.0.0.0`
  - docker example exposes ports `16110` and `17110`.
- The same docs' JavaScript quickstart shows a test RPC override placeholder: `ws://host:17110`.
- `rusty-kaspa` README documents testnet launch as `cargo run --release --bin kaspad -- --testnet`.

### Endpoint conclusion for env-032

- **Public TN12/testnet endpoint URL:** not documented in the repo/docs source checked for this spike.
- **Safe path for this stage:** local node startup command above (once built/available), then use the node-local wRPC candidate `ws://127.0.0.1:17110`.
- Exact first-read call once endpoint is approved:
  - `rpc_client.get_server_info_call(None, GetServerInfoRequest {})`

### Output/logging target

- Planned artifact path for eventual env-032 execution: `spikes/tn12-minimal-covenant/artifacts/env-032-get-server-info.txt`

### Manual approval still required

- Before env-032 execution:
  1. endpoint confirmation (must be TN12/testnet),
  2. explicit permission for one read-only call,
  3. confirmation of artifact path availability,
  4. stop gate for no wallet/faucet/signing/broadcast.

## Env-033 local TN12 node startup plan

- **Scope:** planning-only documentation update; no local node was started and no live RPC call was made.

### Current blocker

- no public TN12 endpoint found in checked docs/source
- no local node installed/running
- read-only `getServerInfo` cannot be run yet

### Candidate local-node path

- use existing `external/silverscript` / `rusty-kaspa` source if suitable
- or use pinned `rusty-kaspa` source already in Cargo cache if suitable
- command candidate from docs:
  - `cargo run --release --bin kaspad -- --testnet --utxoindex`
- `rpclisten` / `borsh` / wRPC flags and exact ports still need confirmation before execution

### Risks / costs

- full node sync time
- disk usage
- CPU/RAM use
- long-running process management
- port exposure
- avoid `0.0.0.0` unless explicitly needed

### Safer recommended first execution (later, only after approval)

- start a local testnet node bound to localhost only
- capture startup logs to `spikes/tn12-minimal-covenant/artifacts/env-033-node-startup.log`
- wait only until server info is available
- run exactly one read-only `getServerInfo` call
- capture output to `spikes/tn12-minimal-covenant/artifacts/env-033-get-server-info.txt`
- stop before wallet/faucet/signing/broadcast

### Required manual approvals

- approval to start a local testnet node
- approval to expose/listen on any port
- approval to run read-only `getServerInfo`
- separate approval for any wallet/faucet/signing/broadcast later

### Conservative conclusion

- The next safe technical action is to prepare a local TN12 node startup command and log plan, but not run it until explicitly approved.

## Env-034 local TN12 node startup command refinement

- **Scope:** documentation-only refinement; no local node was started and no live RPC call was made.

### Source-backed TN12 conclusion

- `--testnet` alone is not sufficient for TN12 in this codebase.
- Local `rusty-kaspa` TN12 doc states the TN12 node command is:
  - `cargo run --release --bin kaspad -- --testnet --netsuffix=12 --utxoindex`
- The same TN12 doc warns that omitting `--netsuffix=12` connects to mainnet or the default testnet.
- `kaspad` argument parsing confirms:
  - `--testnet` selects testnet mode,
  - default `--netsuffix` is `10`,
  - `--rpclisten` is gRPC,
  - `--rpclisten-borsh` is wRPC Borsh,
  - `--rpclisten-json` is wRPC JSON,
  - wRPC listeners are disabled unless explicitly enabled.

### Refined localhost-only command

Recommended minimal command for a later read-only `getServerInfo` check:

- `cargo run --release --bin kaspad -- --testnet --netsuffix=12 --disable-upnp --listen=127.0.0.1:16311 --rpclisten=127.0.0.1:16210 --rpclisten-borsh=127.0.0.1:17210`

Optional additions only if explicitly wanted later:

- add `--utxoindex` to match the TN12 participation doc and make `has_utxo_index=true`, but it is not required for the first read-only `getServerInfo` check.
- add `--rpclisten-json=127.0.0.1:18210` only if a JSON wRPC client is intentionally used.

### Port expectations

- TN12/testnet-12 P2P: `16311`
- testnet gRPC / `--rpclisten`: `16210`
- testnet wRPC Borsh / `--rpclisten-borsh`: `17210`
- testnet wRPC JSON / `--rpclisten-json`: `18210`

Notes:

- P2P port varies by testnet suffix; TN12 uses `16311`.
- RPC/wRPC ports are keyed by network type `testnet`, not by suffix, so TN12 still uses the standard testnet RPC ports above.
- gRPC already defaults to loopback if `--rpclisten` is omitted, but the explicit loopback form is safer for documentation and review.
- `--rpclisten-borsh=default` would also resolve to `127.0.0.1:17210`, but the explicit address is clearer.

### Log path for later execution

- startup log artifact: `spikes/tn12-minimal-covenant/artifacts/env-034-kaspad-startup.log`
- later read-only response artifact: `spikes/tn12-minimal-covenant/artifacts/env-034-get-server-info.txt`

### Stop conditions for the later execution

- stop if the command to be run differs from the approved TN12 form (`--testnet --netsuffix=12`)
- stop if any listen address is not localhost-only
- stop if startup fails before the RPC surface is reachable
- stop after exactly one read-only `getServerInfo` capture, regardless of success/failure
- stop before wallet creation, key generation, faucet use, signing, submission, or broadcast

### Conservative conclusion

- The later first live run should be a localhost-only TN12 node start plus one read-only `getServerInfo` check, with no `0.0.0.0`, no wallet flow, and no transaction activity.

## Env-035 local TN12 read-only getServerInfo

- **Scope:** approved live attempt to start a localhost-only TN12 node, capture startup logs, and stop after one read-only `getServerInfo` check if RPC became available.

### Exact startup command attempted

- `cargo run --release --bin kaspad -- --testnet --netsuffix=12 --disable-upnp --listen=127.0.0.1:16311 --rpclisten=127.0.0.1:16210 --rpclisten-borsh=127.0.0.1:17210`

### Localhost-only binding check

- P2P listen stayed configured as `127.0.0.1:16311`.
- gRPC listen stayed configured as `127.0.0.1:16210`.
- wRPC Borsh listen stayed configured as `127.0.0.1:17210`.
- No `0.0.0.0` listen flag was used.

### Artifacts

- startup log: `spikes/tn12-minimal-covenant/artifacts/env-035-kaspad-startup.log`
- read-only result record: `spikes/tn12-minimal-covenant/artifacts/env-035-get-server-info.txt`

### Result

- Node startup did not succeed.
- The build first failed in `librocksdb-sys` because bindgen could not find `stdbool.h`.
- After retrying with `LIBCLANG_PATH=/usr/lib/llvm-18/lib` and `BINDGEN_EXTRA_CLANG_ARGS="-isystem /usr/lib/gcc/x86_64-linux-gnu/13/include -isystem /usr/include"`, the build advanced further but failed in `kaspa-p2p-lib` because `protoc` is not available in this environment.
- Because the node never exposed the approved local RPC surface, the single read-only `getServerInfo` call was not executed.

### Scope confirmations

- wallet/key created: false
- faucet request made: false
- anything signed: false
- anything submitted/broadcast: false
- stop condition reached: startup failed before RPC readiness

### Conservative conclusion

- The next blocking prerequisite is environment-level build support for `protoc` (and the bindgen header path if not exported), not TN12 flag selection.

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
