# Current handoff

Repo: `/root/kaspa-fair-lab`
Purpose: primary handover note for the next ChatGPT session
Scope: documentation only

## Current project goal

TN12 minimal covenant feasibility spike for a future KaspaFair/Toccata showcase.

## Current feasibility verdict

- local SilverScript/Toccata tooling: GREEN
- local Rust transaction/RPC path: GREEN
- local TN12 full sync: GREEN (RAM/swap caveat)
- local TN12 read-only connectivity: GREEN
- local TN12 read-only RPC confirmation: GREEN
- live covenant create/spend/inspect: not yet proven
- roulette PoC: PAUSED

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
- Synced-node read-only RPC evidence is preserved under `spikes/tn12-minimal-covenant/artifacts/env-046-rpc-readonly-suite/`.
- Read-only RPC checks have succeeded for:
  - `getServerInfo`
  - `getBlockDagInfo`
  - `getSyncStatus`
  - `getCurrentNetwork`
- Latest synced-node read-only values include `hasUtxoIndex=false`, `isSynced=true`, `blockCount=1235733`, `headerCount=1235733`, and `virtualDaaScore=46858621`.
- Recent sync observation shows log-level header sync progress.

## What has not been proven / tested

- Signing.
- Real UTXO usage.
- Faucet funding.
- Live TN12 transaction submission.
- Mempool acceptance.
- Covenant-bound create/spend/inspect lifecycle.
- Roulette/game integration.

## Current node / sync status

- Working VPS startup/sync procedure reference: `spikes/tn12-minimal-covenant/tn12-node-runbook.md`.
- GitHub remote remains `https://github.com/zartyblartfast/kaspa-fair-lab.git`.
- Node can start on TN12 with `--testnet --netsuffix=12`.
- Localhost-only ports are used.
- Earlier `kaspad` exits were caused by the Linux OOM killer.
- Adding 8 GB swap allowed sync to complete.
- DNS seeder warning was observed, but synced-node evidence indicates the earlier failures were not primarily DNS.
- `hasUtxoIndex=false` was observed on the synced-node read-only RPC check.

## Safety rules

- No wallet/key/faucet/signing/broadcast without explicit approval.
- No mainnet.
- Keep all live steps read-only unless explicitly approved.
- Roulette remains PAUSED until live covenant lifecycle proof exists.

## Recommended next step

1. Treat ENV-046 as complete and as the current TN12 readiness baseline.
2. Treat ENV-047 as planning-only pending explicit approval.
3. Use `spikes/tn12-minimal-covenant/env-047-gated-plan.md` as the gate-by-gate checklist for any future live work.

## ENV-047 planning status

- Planning doc: `spikes/tn12-minimal-covenant/env-047-gated-plan.md`
- Status: planning-only
- Execution state: no wallet, no faucet, no signing, no broadcast, no mainnet
- Any ENV-047 gate requires explicit approval before execution

## First prompt for new ChatGPT session

Use this as the first prompt:

`Continue from /root/kaspa-fair-lab/docs/current-handoff.md. Treat ENV-046 as complete: local SilverScript/Toccata tooling GREEN, local Rust transaction/RPC path GREEN, local TN12 full sync GREEN with RAM/swap caveat, and local TN12 read-only RPC confirmation GREEN from env-046 under spikes/tn12-minimal-covenant/artifacts/env-046-rpc-readonly-suite/. Treat ENV-047 as planning-only pending explicit approval, and use spikes/tn12-minimal-covenant/env-047-gated-plan.md as the gate-by-gate checklist. Keep all current constraints in force: documentation-first, no roulette build, no web app, no wallet creation unless explicitly approved, no key generation unless explicitly approved, no faucet funding unless explicitly approved, no signing unless explicitly approved, no broadcast unless explicitly approved, no mainnet, no dependency installs, no repo clones, no external SilverScript edits, and no live covenant operations without explicit approval.`

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
