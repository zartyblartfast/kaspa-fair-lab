# Current handoff

Repo: `/root/kaspa-fair-lab`
Purpose: primary handover note for the next ChatGPT session
Scope: documentation only

## Current project goal

TN12 minimal covenant feasibility spike for a future KaspaFair/Toccata showcase.

## Current feasibility verdict

- ENV-046 is complete and pushed
- local SilverScript/Toccata tooling: GREEN
- local Rust transaction/RPC path: GREEN
- local TN12 synced node: GREEN (RAM/swap caveat)
- local TN12 read-only RPC confirmation: GREEN
- ENV-047 is planning-only background context
- ENV-049 Gate 1 key/address generation: COMPLETE
- one TN12 test-only address has been generated
- ENV-050 Gate 2 faucet funding: PENDING / BLOCKED FOR SAFETY REVIEW
- ENV-050A funding-route discovery: COMPLETE (needs human follow-up)
- wallet/faucet/signing/broadcast/covenant lifecycle beyond Gate 1 remains NOT TESTED
- roulette remains PAUSED

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
- ENV-049 Gate 1 generated one TN12 test-only address with a non-interactive local helper.
- Public ENV-049 evidence is preserved under `spikes/tn12-minimal-covenant/artifacts/env-049-key-address/`.
- Existing ENV-049 address reused for ENV-050 Gate 2: `kaspatest:qqaq5f4ju52g9r869c50n55lmtgku9nsf2pc56y76neaj7rksmewg2ytrxccg`.
- Public ENV-050 evidence is preserved under `spikes/tn12-minimal-covenant/artifacts/env-050-faucet-funding/`.
- Public ENV-050A discovery evidence is preserved under `spikes/tn12-minimal-covenant/artifacts/env-050a-funding-route-discovery/`.
- Private material for ENV-049, if needed, is stored only under ignored `spikes/tn12-minimal-covenant/local-secrets/`.

## What has not been proven / tested

- Signing.
- Real UTXO usage.
- ENV-050 Gate 2 funding completion.
- ENV-050A still needs a human-verified TN12 funding route before any submission.
- Gate 3 read-only UTXO inspection (not started).
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

1. Treat ENV-046 as complete and as the TN12 readiness baseline.
2. Treat ENV-047 and ENV-048 as planning/preflight background context.
3. Treat ENV-049 Gate 1 as complete: one TN12 test-only address generated, with public evidence under `spikes/tn12-minimal-covenant/artifacts/env-049-key-address/`.
4. Treat ENV-050 Gate 2 as still blocked pending a verified TN12/testnet-12 funding route.
5. Treat ENV-050A as complete discovery-only: official Discord/community escalation was identified, but no automated funding route was verified strongly enough to use.
6. Do not proceed to Gate 3 read-only UTXO inspection, signing, broadcast, or covenant lifecycle work without explicit future approval.

## ENV-047 planning status

- Planning doc: `spikes/tn12-minimal-covenant/env-047-gated-plan.md`
- Status: planning-only
- Execution state: no wallet, no faucet, no signing, no broadcast, no mainnet
- Any ENV-047 gate requires explicit approval before execution

## ENV-048 preflight status

- Active step: completed preflight reference
- Discovery note: codex-spark completed the prior read-only discovery pass
- Formal preflight doc: `spikes/tn12-minimal-covenant/env-048-key-address-preflight.md`
- Execution state during ENV-048: no key/address/wallet/faucet/signing/broadcast action was executed

## ENV-049 Gate 1 status

- Status: complete
- One TN12 test-only address generated
- Public evidence path: `spikes/tn12-minimal-covenant/artifacts/env-049-key-address/env-049-summary.txt`
- Helper path used: `spikes/tn12-minimal-covenant/env-049-key-address-helper/src/main.rs`
- Private material, if any, stored only under ignored `spikes/tn12-minimal-covenant/local-secrets/`
- Faucet/signing/broadcast/covenant lifecycle still NOT TESTED after Gate 1
- Roulette remains PAUSED

## ENV-050 Gate 2 status

- Status: pending / blocked for safety review
- Existing ENV-049 address reused: `kaspatest:qqaq5f4ju52g9r869c50n55lmtgku9nsf2pc56y76neaj7rksmewg2ytrxccg`
- Candidate faucet route checked: `https://faucet.kaspanet.io/`
- Accessible session state: blocked at Cloudflare security verification; no faucet form or network selector became available for verification
- Public evidence path: `spikes/tn12-minimal-covenant/artifacts/env-050-faucet-funding/env-050-summary.txt`
- Faucet funding requested/completed/pending: pending
- Gate 3 read-only UTXO inspection remains NOT STARTED
- Signing/broadcast/covenant lifecycle remains NOT TESTED
- Roulette remains PAUSED

## ENV-050A funding-route discovery status

- Status: complete (result = NEEDS HUMAN)
- Funding remains blocked unless a verified TN12 route is found
- Existing ENV-049 address remains the only approved address: `kaspatest:qqaq5f4ju52g9r869c50n55lmtgku9nsf2pc56y76neaj7rksmewg2ytrxccg`
- Rejected routes:
  - public faucet routes limited to TN10/TN11
  - `kaspa-ng.org` because Phantom flagged it as malicious/unsafe
  - `https://faucet.kaspanet.io/` because the accessible session stayed blocked at Cloudflare without TN12-verifiable funding controls
- Discovery evidence path: `spikes/tn12-minimal-covenant/artifacts/env-050a-funding-route-discovery/env-050a-summary.txt`
- Best current human-assisted route found: official Kaspa Discord `https://discord.gg/kaspa`
- Gate 3 read-only UTXO inspection remains NOT STARTED
- Signing/broadcast/covenant lifecycle remains NOT TESTED
- Roulette remains PAUSED

## Suggested model/session guidance

- use `gpt-5.4` for ENV-048 documentation and normal repo work
- use `gpt-5.5` before any high-risk signing, broadcast, or covenant execution
- use `gpt-5.3-codex-spark` only for small bounded tasks and refresh with `/new` regularly

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
