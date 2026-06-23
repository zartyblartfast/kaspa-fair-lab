# TN12 Spike Handoff

## Current project goal
Continue TN12 minimal covenant spike route discovery with documentation-first evidence, while staying constrained to:

- no roulette build,
- no web app,
- no own covenant implementation,
- no transaction submission,
- no mainnet usage.

## Concise status update

1) env-037 succeeded for the approved localhost-only TN12 retry: the local node reached RPC readiness, exactly one read-only `getServerInfo` call succeeded against `grpc://127.0.0.1:16210`, output was captured, and the node was stopped immediately afterward.

1b) env-038 also succeeded for the approved localhost-only follow-up: the local node again reached RPC readiness, exactly one read-only `getBlockDagInfo` call succeeded against `grpc://127.0.0.1:16210`, output was captured, and the node was stopped immediately afterward.

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
- one live read-only TN12 `getServerInfo` call now succeeded with captured output.
- one additional live read-only TN12 `getBlockDagInfo` call now succeeded with captured output.

3) Scope limits still in force and still unproven for anything beyond read-only connectivity:
- nothing was signed,
- nothing was broadcast,
- no live TN12 create/spend/inspect was attempted,
- no real UTXO/faucet path was exercised,
- no mainnet behaviour is known.

4) Conservative conclusion:
- Local tooling is now sufficient to prove localhost-only TN12 node startup plus one read-only `getServerInfo` call and one read-only `getBlockDagInfo` call.
- Local tooling is still not sufficient to claim live TN12 create/spend/inspect works.
- Any next step beyond read-only connectivity still needs fresh explicit approval.

5) env-038 localhost-only node startup + read-only RPC result:
- exact startup command used:
  - `cargo run --release --bin kaspad -- --testnet --netsuffix=12 --disable-upnp --listen=127.0.0.1:16311 --rpclisten=127.0.0.1:16210 --rpclisten-borsh=127.0.0.1:17210`
- localhost-only bind policy held:
  - no `0.0.0.0` listen flags were used
  - configured addresses stayed `127.0.0.1:16311`, `127.0.0.1:16210`, and `127.0.0.1:17210`
- startup/readiness evidence:
  - startup log shows `GRPC Server starting on: 127.0.0.1:16210`
  - startup log shows `P2P Server starting on: 127.0.0.1:16311`
  - startup log shows `WRPC Server starting on: 127.0.0.1:17210`
  - `ss -ltnp` confirmed listeners on the same three localhost ports before the call
  - `ss -ltnp` showed no remaining listeners on those ports after the node was stopped
- exact read-only RPC call used:
  - one gRPC `getBlockDagInfo` call against `grpc://127.0.0.1:16210`
- returned fields:
  - `network=testnet-12`
  - `blockCount=0`
  - `headerCount=0`
  - `difficulty=655360.625000596`
  - `pastMedianTime=1633687894966`
  - `virtualDaaScore=0`
  - `tipHashes[0]=300fe02031119f6132f39ec03c5cf7ddf10cc23d6f5c3e5fe42d6391dc3d5c2a`
  - `virtualParentHashes[0]=300fe02031119f6132f39ec03c5cf7ddf10cc23d6f5c3e5fe42d6391dc3d5c2a`
  - `pruningPointHash=300fe02031119f6132f39ec03c5cf7ddf10cc23d6f5c3e5fe42d6391dc3d5c2a`
  - `sink=300fe02031119f6132f39ec03c5cf7ddf10cc23d6f5c3e5fe42d6391dc3d5c2a`
- result:
  - node startup succeeded to RPC readiness
  - exactly one read-only `getBlockDagInfo` call succeeded
  - artifacts: `spikes/tn12-minimal-covenant/artifacts/env-038-kaspad-startup.log` and `spikes/tn12-minimal-covenant/artifacts/env-038-get-blockdag-info.txt`
  - the node was stopped immediately after capture

6) Constraint confirmations for env-038:
- no wallet/key was created
- no faucet request was made
- nothing was signed
- nothing was submitted or broadcast
- no mainnet usage

7) Remaining approval gates:
- approval before any wallet/key creation,
- approval before any faucet request,
- approval before any signing,
- approval before any transaction submission/broadcast,
- approval before any non-localhost exposure.

## Branch / repo status

- Repo: `/root/kaspa-fair-lab`
- Branch: `main` (`origin/main`)
- Modified files:
  - `docs/current-handoff.md`
  - `spikes/tn12-minimal-covenant/README.md`
  - `spikes/tn12-minimal-covenant/findings.md`

## Suggested first prompt after /new

`Continue from env-037: treat localhost-only TN12 startup plus one read-only getServerInfo call as proven, keep the same constraints (no wallet/faucet/signing/broadcast), and propose the next smallest explicitly approvable read-only TN12 verification step.`
