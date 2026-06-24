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
- One additional read-only local TN12 `getBlockDagInfo` call succeeded in env-038, with output captured in `spikes/tn12-minimal-covenant/artifacts/env-038-get-blockdag-info.txt`.
- One additional read-only local TN12 `getSyncStatus` call succeeded in env-039, with output captured in `spikes/tn12-minimal-covenant/artifacts/env-039-get-sync-status.txt`.
- One final additional read-only local TN12 `getCurrentNetwork` call succeeded in env-040, with output captured in `spikes/tn12-minimal-covenant/artifacts/env-040-get-current-network.txt`.
- env-041 added a documentation-only feasibility summary and go/no-go assessment for the KaspaFair/Toccata showcase idea.
- env-042 ran a longer localhost-only TN12 sync observation under the same no-wallet/no-faucet/no-signing/no-broadcast constraints. The cumulative observation window exceeded 30 minutes across two approved long-run attempts, sync progress was visible in the node logs, the final approved read-only artifacts were captured, and the node was stopped with post-stop listener verification.
- env-043 hardened `run_env_042_observation.sh` so it validates the intended `rusty-kaspa` Cargo workspace before any `cargo run`, uses `--manifest-path`, and refuses to fall back to the repo root.
- env-044 reran the hardened observation script from repo root. The hardening worked: startup succeeded via `--manifest-path` and no caller-cwd Cargo failure recurred. The observed runtime after readiness was `28m36s`, localhost-only listeners were confirmed on `127.0.0.1:16311`, `127.0.0.1:16210`, and `127.0.0.1:17210`, sync progress was visible in the node log, and the node stopped with no remaining listeners. However, `kaspad` exited during the observation loop before the script reached its final read-only RPC suite, so env-044 did not regenerate fresh end-state `getServerInfo` / `getBlockDagInfo` / `getSyncStatus` artifacts.
- No signing was performed.
- No real UTXO was used.
- No faucet funding was used.
- No live submit/broadcast steps were attempted.
- No live TN12 create/spend/inspect lifecycle has been proven.

## Env-042 local TN12 30-minute sync observation

- Scope: use the approved localhost-only TN12 startup command, observe sync behavior for approximately 30 minutes without any state-changing RPC/wallet/faucet/signing/broadcast step, capture startup/sync logs, capture final read-only RPC outputs, stop the node, and verify the localhost listeners are gone.

### Exact startup command used

- `cargo run --release --bin kaspad -- --testnet --netsuffix=12 --disable-upnp --listen=127.0.0.1:16311 --rpclisten=127.0.0.1:16210 --rpclisten-borsh=127.0.0.1:17210`

### Localhost-only bind check

- P2P listen: `127.0.0.1:16311`
- gRPC listen: `127.0.0.1:16210`
- wRPC Borsh listen: `127.0.0.1:17210`
- All approved listen addresses were localhost-only: true
- No `0.0.0.0` listen flag was used.

### Observation duration and control notes

- Initial env-042 launch attempt failed immediately at `2026-06-24T09:01:31Z` before readiness because `run_env_042_observation.sh` was started from `/root/kaspa-fair-lab`, where Cargo could not find a `Cargo.toml`; the recorded error was: `error: could not find Cargo.toml in /root/kaspa-fair-lab or any parent directory`.
- After that failure, the observation flow was corrected to start from the `rusty-kaspa` Cargo checkout rather than the repo root.
- env-043 hardened the wrapper so it now validates the resolved `rusty-kaspa` workspace directory and `Cargo.toml` up front, checks that the manifest still looks like the expected checkout by matching `kaspad`, and runs `cargo` via `--manifest-path` instead of relying on the caller's current working directory.
- Long-run attempt A reached readiness at `2026-06-24T09:02:13Z` and then exited unexpectedly during observation at `2026-06-24T09:23:54Z` (`21m41s` runtime after readiness).
- Long-run attempt B reused the same approved command and remained running for at least `27` more minutes under process monitoring before the managed background process exited unexpectedly before the planned 30-minute endpoint.
- Because the long-run attempts did not remain up continuously to the exact planned endpoint, the final approved read-only capture was taken after a short localhost-only restart using the same approved command. This kept the scope read-only and localhost-only, but the observation should be described as cumulative rather than one uninterrupted 30-minute run.
- Cumulative observed runtime before the final read-only capture exceeded 30 minutes.

### Node log and artifacts

- primary node log: `spikes/tn12-minimal-covenant/artifacts/env-042-kaspad-30min-sync.log`
- supporting runner/monitor log: `spikes/tn12-minimal-covenant/artifacts/env-042-runner.log`
- post-stop listener check: `spikes/tn12-minimal-covenant/artifacts/env-042-post-stop-listeners.txt`
- read-only outputs:
  - `spikes/tn12-minimal-covenant/artifacts/env-042-get-server-info-after-30min.txt`
  - `spikes/tn12-minimal-covenant/artifacts/env-042-get-blockdag-info-after-30min.txt`
  - `spikes/tn12-minimal-covenant/artifacts/env-042-get-sync-status-after-30min.txt`

### Read-only RPC calls used

- `getServerInfo`
- `getBlockDagInfo`
- `getSyncStatus`

### Before/after comparison

- Before (earlier env-037/env-038/env-039 evidence):
  - `blockCount=0`
  - `headerCount=0`
  - `virtualDaaScore=0`
  - `isSynced=false`
- After (env-042 final approved read-only capture):
  - `blockCount=0`
  - `headerCount=0`
  - `virtualDaaScore=0`
  - `isSynced=false`

### Sync progress outcome

- Sync progress did occur in the startup/sync logs.
- The strongest concrete evidence is in the long-run observation log excerpts recorded in `env-042-runner.log`, including:
  - `IBD: Processed 121972 block headers (8%)`
  - `IBD: Processed 289042 block headers (19%)`
- Even with that observed header-proof/IBD progress, the final approved read-only `getBlockDagInfo`/`getServerInfo` capture still showed `blockCount=0`, `headerCount=0`, `virtualDaaScore=0`, and `isSynced=false`.
- Therefore: sync progress occurred in logs, but it was not reflected as an improved final read-only DAG/server state in the captured endpoint outputs.

### Result

- Node startup succeeded.
- Localhost-only bind policy held.
- Final approved read-only captures succeeded.
- Final post-stop `ss -ltnp` verification showed no remaining listeners on `127.0.0.1:16210`, `127.0.0.1:16311`, or `127.0.0.1:17210`.
- Final capture stop was clean; however, the two longer observation attempts ended unexpectedly before the exact planned endpoint.

### Scope confirmations

- wallet/key created: false
- faucet request made: false
- anything signed: false
- anything submitted/broadcast: false
- stop condition reached: true

## Env-044 hardened TN12 30-minute sync observation rerun

- Scope: rerun `./spikes/tn12-minimal-covenant/run_env_042_observation.sh` exactly as approved, keep the node localhost-only, observe sync for approximately 30 minutes, capture startup/sync logs, stop after the read-only observation window, and confirm no wallet/faucet/signing/broadcast action occurred.

### Exact script command run

- `./spikes/tn12-minimal-covenant/run_env_042_observation.sh`

### Hardening and localhost-only checks

- The hardened directory/manifest guard passed: the script launched `kaspad` via `cargo run --release --manifest-path /root/.cargo/git/checkouts/rusty-kaspa-410e06d1fde91a92/42b734f/Cargo.toml --bin kaspad ...` and did not reproduce the earlier repo-root `could not find Cargo.toml` failure.
- Runner log readiness evidence showed the approved localhost-only listeners:
  - `127.0.0.1:16210`
  - `127.0.0.1:16311`
  - `127.0.0.1:17210`
- No `0.0.0.0` listener was used.

### Observation duration and outcome

- Readiness was logged at `2026-06-24T13:09:21Z`.
- The script logged `kaspad exited during observation` at `2026-06-24T13:37:57Z`.
- Observed runtime after readiness: `1716` seconds (`28m36s`).
- Because the node exited before the script reached `observation window complete; running read-only checks`, env-044 did not produce fresh final read-only RPC artifacts.

### Artifact/log paths touched by env-044

- updated node log: `spikes/tn12-minimal-covenant/artifacts/env-042-kaspad-30min-sync.log`
- updated runner log: `spikes/tn12-minimal-covenant/artifacts/env-042-runner.log`
- stale prior end-state artifacts still on disk but not regenerated by env-044:
  - `spikes/tn12-minimal-covenant/artifacts/env-042-get-server-info-after-30min.txt`
  - `spikes/tn12-minimal-covenant/artifacts/env-042-get-blockdag-info-after-30min.txt`
  - `spikes/tn12-minimal-covenant/artifacts/env-042-get-sync-status-after-30min.txt`
  - `spikes/tn12-minimal-covenant/artifacts/env-042-post-stop-listeners.txt`

### Sync-progress evidence

- Sync progress occurred in the env-044 log rerun.
- Example log-level evidence captured in the updated runner log:
  - `IBD: Processed 363640 block headers (33%)`
  - `IBD: Processed 417842 block headers (38%)`
- Later in the same run, header processing slowed to repeated low-count `Processed 0 blocks and N headers` lines while DNS seeder warnings appeared, but the logs still show real sync progress relative to the earlier zero/false baseline.

### Final read-only RPC results

- Fresh env-044 end-state `getServerInfo` result: not captured
- Fresh env-044 end-state `getBlockDagInfo` result: not captured
- Fresh env-044 end-state `getSyncStatus` result: not captured
- Reason: the script exited on the `kaspad exited during observation` path before invoking `spikes/tn12-minimal-covenant/rpc-readonly-suite`.
- Latest previously captured values still available from env-042 artifacts (not regenerated by env-044):
  - `blockCount=0`
  - `headerCount=0`
  - `virtualDaaScore=0`
  - `isSynced=false`
  - `networkId=testnet-12`
  - `serverVersion=1.1.1-toc.1`

### Teardown and scope confirmations

- Post-run `ss -ltnp` verification after env-044 showed no remaining listeners on:
  - `127.0.0.1:16210`
  - `127.0.0.1:16311`
  - `127.0.0.1:17210`
- wallet/key created: false
- faucet request made: false
- anything signed: false
- anything submitted/broadcast: false
- node startup succeeded: true
- node stopped cleanly: partially; the process exited before the planned endpoint, but no listeners remained afterward
- listeners cleared: true

## Env-041 feasibility summary and go/no-go assessment

### Executive feasibility verdict

- local Toccata/SilverScript feasibility: GREEN
- local Rust transaction/RPC feasibility: GREEN
- live TN12 readiness: AMBER
- suitability for a future KaspaFair roulette PoC: AMBER

### What has been proven

- SilverScript build/compile/local simulation works locally.
- repo-owned no-broadcast fixtures pass.
- local `Transaction` construction works.
- local `RpcTransaction` conversion works.
- local `SubmitTransactionRequest` construction works.
- RPC serializer artifacts are produced.
- RPC serializer round-trip verification passes.
- local TN12 node startup works.
- read-only TN12 RPC connectivity works for `getServerInfo`, `getBlockDagInfo`, `getSyncStatus`, and `getCurrentNetwork`.

### What has not been proven

- signing
- real UTXO usage
- faucet funding
- live transaction submission
- mempool acceptance
- covenant-bound create/spend/inspect lifecycle
- wallet UX
- roulette/game integration

### Feasibility risks

- node sync status and peer connectivity remain only partially characterized; reachability is proven more strongly than full sync/readiness.
- TN12 tooling maturity beyond the current local simulation/object/RPC-shape work remains uncertain.
- covenant transaction construction complexity may increase substantially once real UTXOs, fees, addresses, and network-valid transaction details are required.
- signing/spend path complexity remains unproven.
- there is still a gap between local simulation/object serialization success and live network acceptance.
- a future roulette PoC still depends on first proving the covenant lifecycle itself.

### Stop/continue criteria

- Continue only if the next TN12 steps remain controlled, reproducible, and evidence-backed.
- Stop or pause if signing/covenant spend work requires large undocumented Rusty Kaspa internals.
- Stop or pause if the TN12 node cannot sync/connect reliably.
- Stop or pause if a covenant-bound live transaction would require fragile custom code.

### Recommended next technical milestone

- Do not build roulette yet.
- Safer next option: allow the local TN12 node to sync further and confirm stronger readiness.
- Alternative later option: plan a test-only key/address/faucet workflow, still without broadcast.
- Recommendation: choose the sync/readiness option first because it preserves the current no-wallet/no-key/no-faucet/no-signing posture while reducing network-readiness uncertainty.

### Feasibility conclusion for the KaspaFair roulette PoC

- The project is worth continuing as a constrained technical spike.
- Roulette UI/app development should remain paused.
- Before roulette work resumes, the project must still prove a controlled test-only key/address/faucet path, a real signing path, and a covenant-bound create/spend/inspect lifecycle that reaches live TN12 acceptance.
- The real “wow” proof would be a reproducible TN12 test-only end-to-end create/spend/inspect demonstration with captured evidence.

## Env-040 local TN12 read-only getCurrentNetwork

- **Scope:** approved localhost-only TN12 node startup retry, capture startup logs, run exactly one read-only `getCurrentNetwork` call once RPC was reachable, capture the output, and stop before any wallet/key/faucet/signing/submission/broadcast action.

### Exact startup command used

- `cargo run --release --bin kaspad -- --testnet --netsuffix=12 --disable-upnp --listen=127.0.0.1:16311 --rpclisten=127.0.0.1:16210 --rpclisten-borsh=127.0.0.1:17210`

### Localhost-only bind check

- P2P listen: `127.0.0.1:16311`
- gRPC listen: `127.0.0.1:16210`
- wRPC Borsh listen: `127.0.0.1:17210`
- No `0.0.0.0` listen flag was used.

### RPC/read-only result

- Exact read-only RPC call used: one gRPC `getCurrentNetwork` call against `grpc://127.0.0.1:16210`.
- Returned network field:
  - `network`: `testnet`

### Artifacts

- startup log: `spikes/tn12-minimal-covenant/artifacts/env-040-kaspad-startup.log`
- read-only result: `spikes/tn12-minimal-covenant/artifacts/env-040-get-current-network.txt`

### Result

- Node startup succeeded to the point of exposing the approved localhost-only RPC surfaces.
- Exactly one read-only `getCurrentNetwork` call succeeded.
- The node was then stopped without any wallet/key/faucet/signing/submission/broadcast activity.

### Scope confirmations

- wallet/key created: false
- faucet request made: false
- anything signed: false
- anything submitted/broadcast: false
- stop condition reached: stopped immediately after capturing the single read-only `getCurrentNetwork` result

## Env-039 local TN12 read-only getSyncStatus

- **Scope:** approved localhost-only TN12 node startup retry, capture startup logs, run exactly one read-only `getSyncStatus` call once RPC was reachable, capture the output, and stop before any wallet/key/faucet/signing/submission/broadcast action.

### Exact startup command used

- `cargo run --release --bin kaspad -- --testnet --netsuffix=12 --disable-upnp --listen=127.0.0.1:16311 --rpclisten=127.0.0.1:16210 --rpclisten-borsh=127.0.0.1:17210`

### Localhost-only bind check

- P2P listen: `127.0.0.1:16311`
- gRPC listen: `127.0.0.1:16210`
- wRPC Borsh listen: `127.0.0.1:17210`
- No `0.0.0.0` listen flag was used.

### RPC/read-only result

- Exact read-only RPC call used: one gRPC `getSyncStatus` call against `grpc://127.0.0.1:16210`.
- Returned sync fields:
  - `isSynced`: `false`

### Artifacts

- startup log: `spikes/tn12-minimal-covenant/artifacts/env-039-kaspad-startup.log`
- read-only result: `spikes/tn12-minimal-covenant/artifacts/env-039-get-sync-status.txt`

### Result

- Node startup succeeded to the point of exposing the approved localhost-only RPC surfaces.
- Exactly one read-only `getSyncStatus` call succeeded.
- The node was then stopped without any wallet/key/faucet/signing/submission/broadcast activity.

### Scope confirmations

- wallet/key created: false
- faucet request made: false
- anything signed: false
- anything submitted/broadcast: false
- stop condition reached: stopped immediately after capturing the single read-only `getSyncStatus` result

## Env-038 local TN12 read-only getBlockDagInfo

- **Scope:** approved localhost-only TN12 node startup retry, capture startup logs, run exactly one read-only `getBlockDagInfo` call once RPC was reachable, capture the output, and stop before any wallet/key/faucet/signing/submission/broadcast action.

### Exact startup command used

- `cargo run --release --bin kaspad -- --testnet --netsuffix=12 --disable-upnp --listen=127.0.0.1:16311 --rpclisten=127.0.0.1:16210 --rpclisten-borsh=127.0.0.1:17210`

### Localhost-only bind check

- P2P listen: `127.0.0.1:16311`
- gRPC listen: `127.0.0.1:16210`
- wRPC Borsh listen: `127.0.0.1:17210`
- No `0.0.0.0` listen flag was used.

### RPC/read-only result

- Exact read-only RPC call used: one gRPC `getBlockDagInfo` call against `grpc://127.0.0.1:16210`.
- Returned DAG/network fields:
  - `network`: `testnet-12`
  - `blockCount`: `0`
  - `headerCount`: `0`
  - `tipHashes[0]`: `300fe02031119f6132f39ec03c5cf7ddf10cc23d6f5c3e5fe42d6391dc3d5c2a`
  - `difficulty`: `655360.625000596`
  - `pastMedianTime`: `1633687894966`
  - `virtualParentHashes[0]`: `300fe02031119f6132f39ec03c5cf7ddf10cc23d6f5c3e5fe42d6391dc3d5c2a`
  - `pruningPointHash`: `300fe02031119f6132f39ec03c5cf7ddf10cc23d6f5c3e5fe42d6391dc3d5c2a`
  - `virtualDaaScore`: `0`
  - `sink`: `300fe02031119f6132f39ec03c5cf7ddf10cc23d6f5c3e5fe42d6391dc3d5c2a`

### Artifacts

- startup log: `spikes/tn12-minimal-covenant/artifacts/env-038-kaspad-startup.log`
- read-only result: `spikes/tn12-minimal-covenant/artifacts/env-038-get-blockdag-info.txt`

### Result

- Node startup succeeded to the point of exposing the approved localhost-only RPC surfaces.
- Exactly one read-only `getBlockDagInfo` call succeeded.
- The node was then stopped without any wallet/key/faucet/signing/submission/broadcast activity.

### Scope confirmations

- wallet/key created: false
- faucet request made: false
- anything signed: false
- anything submitted/broadcast: false
- stop condition reached: stopped immediately after capturing the single read-only `getBlockDagInfo` result

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
