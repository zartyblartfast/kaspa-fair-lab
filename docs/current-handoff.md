# Current handoff

Repo: `/root/kaspa-fair-lab`
Purpose: primary handover note for the next ChatGPT session
Scope: documentation only

## Current project goal

TN12 minimal covenant feasibility spike for a future KaspaFair/Toccata showcase.

## Current feasibility verdict

- local SilverScript/Toccata tooling: GREEN
- local Rust transaction/RPC path: GREEN
- local TN12 read-only connectivity: GREEN
- live covenant create/spend/inspect: not yet proven
- roulette PoC: remain paused

## What has been proven

- SilverScript builds.
- Simple covenant compiles.
- Repo-owned local fixtures pass.
- `run_no_broadcast_checks.sh` passes.
- Local `Transaction` construction works.
- `RpcTransaction` conversion works.
- `SubmitTransactionRequest` construction works.
- RPC serializer artifacts and round-trip verification pass.
- Local TN12 node starts with localhost-only listeners.
- Read-only RPC checks have succeeded for:
  - `getServerInfo`
  - `getBlockDagInfo`
  - `getSyncStatus`
  - `getCurrentNetwork`
- Recent sync observation shows log-level header sync progress.

## What has not been proven

- Node fully synced.
- Signing.
- Real UTXO usage.
- Faucet funding.
- Live TN12 transaction submission.
- Mempool acceptance.
- Covenant-bound create/spend/inspect lifecycle.
- Roulette/game integration.

## Current node / sync status

- Working VPS startup/sync procedure reference: `spikes/tn12-minimal-covenant/tn12-node-runbook.md`.
- Node can start on TN12 with `--testnet --netsuffix=12`.
- Localhost-only ports are used.
- DNS seeder warning was observed, but sync still appeared to progress.
- Latest observation showed header progress in logs, while RPC counters may still show zero / false.
- Manual tmux sync may currently be running or may need to be restarted or checkpointed.

## Safety rules

- No wallet/key/faucet/signing/broadcast without explicit approval.
- No mainnet.
- Keep all live steps read-only unless explicitly approved.
- Roulette remains paused until live covenant lifecycle proof exists.

## Recommended next step

1. Finish or check the current TN12 sync observation.
2. Capture fresh `getServerInfo`, `getBlockDagInfo`, and `getSyncStatus`.
3. Decide whether node sync/readiness is sufficient before planning any test-only wallet/faucet work.

## First prompt for new ChatGPT session

Use this as the first prompt:

`Continue from /root/kaspa-fair-lab/docs/current-handoff.md. Treat these as already proven: local SilverScript/Toccata tooling GREEN, local Rust transaction/RPC path GREEN, and local TN12 read-only connectivity GREEN. Keep all current constraints in force: documentation-first, no roulette build, no web app, no wallet creation, no key generation, no faucet funding, no signing, no broadcast, no mainnet, no dependency installs, no repo clones, no external SilverScript edits, no kaspad startup unless explicitly approved, and no RPC unless explicitly approved. Focus on checking or finishing the current TN12 sync observation and determining whether sync/readiness is sufficient before planning any test-only wallet/faucet work.`

## Constraints recap

- Do not build roulette.
- Do not create a web app.
- Do not submit or broadcast any Kaspa transaction.
- Do not create a wallet.
- Do not generate keys.
- Do not request faucet funds.
- Do not sign anything.
- Do not use mainnet.
- Do not install dependencies.
- Do not clone repositories.
- Do not modify external SilverScript source.
- Do not start kaspad.
- Do not call RPC.
