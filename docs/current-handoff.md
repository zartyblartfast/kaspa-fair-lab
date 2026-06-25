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
- ENV-050B TN12 mining preflight: COMPLETE
- ENV-050C TN12 mining attempt: BLOCKED (miner binary unavailable)
- ENV-051 TN10 public wRPC proof: COMPLETE
- ENV-052 TN10 test-only wallet/address generation: COMPLETE
- ENV-053 TN10 faucet funding only: COMPLETE
- ENV-054 TN10 read-only balance and UTXO inspection: COMPLETE
- ENV-055 TN10 ordinary spend preflight / dry-run only: READY
- ENV-056 TN10 live ordinary send of 1 TKAS: COMPLETE
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
- ENV-051 proved official `tn10-toc3` `kaspa-wallet` can connect to public resolver-backed TN10 wRPC.
- ENV-052 generated one TN10 test-only wallet/address with the official `tn10-toc3` `kaspa-wallet`.
- Public ENV-052 evidence is preserved under `spikes/tn12-minimal-covenant/artifacts/env-052-tn10-wallet-address/`.
- ENV-053 submitted one TN10 faucet request to the existing ENV-052 address via the public TN10 faucet.
- Public ENV-053 evidence is preserved under `spikes/tn12-minimal-covenant/artifacts/env-053-tn10-faucet-funding/`.
- ENV-054 completed read-only TN10 balance and UTXO inspection for the funded ENV-052 address.
- Public ENV-054 evidence is preserved under `spikes/tn12-minimal-covenant/artifacts/env-054-tn10-readonly-utxo/`.
- ENV-055 completed ordinary TN10 spend preflight only and identified the future estimate/send command path without signing or broadcasting.
- Public ENV-055 evidence is preserved under `spikes/tn12-minimal-covenant/artifacts/env-055-tn10-ordinary-spend-preflight/`.
- ENV-056 completed one live ordinary TN10 send of 1 TKAS to a fresh TN10 recipient address and confirmed the resulting recipient/change UTXOs.
- Public ENV-056 evidence is preserved under `spikes/tn12-minimal-covenant/artifacts/env-056-tn10-ordinary-send/`.
- ENV-049 Gate 1 generated one TN12 test-only address with a non-interactive local helper.
- Public ENV-049 evidence is preserved under `spikes/tn12-minimal-covenant/artifacts/env-049-key-address/`.
- Existing ENV-049 address reused for ENV-050 Gate 2: `kaspatest:qqaq5f4ju52g9r869c50n55lmtgku9nsf2pc56y76neaj7rksmewg2ytrxccg`.
- Public ENV-050 evidence is preserved under `spikes/tn12-minimal-covenant/artifacts/env-050-faucet-funding/`.
- Public ENV-050A discovery evidence is preserved under `spikes/tn12-minimal-covenant/artifacts/env-050a-funding-route-discovery/`.
- Public ENV-050B mining preflight evidence is preserved under `spikes/tn12-minimal-covenant/artifacts/env-050b-tn12-mining-preflight/`.
- Public ENV-050C mining-attempt evidence is preserved under `spikes/tn12-minimal-covenant/artifacts/env-050c-tn12-mining-attempt/`.
- Private material for ENV-049, if needed, is stored only under ignored `spikes/tn12-minimal-covenant/local-secrets/`.

## What has not been proven / tested

- Signing.
- Real UTXO usage.
- ENV-050 Gate 2 funding completion.
- ENV-050A still needs a human-verified TN12 funding route before any submission.
- ENV-050B identified mining as the likely TN12 funding path if a later one-thread mining attempt is approved.
- ENV-050C showed the immediate blocker: `kaspa-miner` is not installed/available locally, so no mining run occurred.
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
6. Treat ENV-050B as complete preflight-only: direct TN12 mining against the existing synced node is the likely next funding path if explicitly approved later; no mining has started yet.
7. Treat ENV-050C as blocked: the approved one-thread mining attempt could not start because `kaspa-miner` is not installed/available locally, and no install was approved.
8. Treat ENV-053 as complete: TN10 faucet funding request succeeded for the ENV-052 address, with public evidence under `spikes/tn12-minimal-covenant/artifacts/env-053-tn10-faucet-funding/`.
9. Treat ENV-054 as complete: TN10 read-only balance and UTXO inspection passed for the funded ENV-052 address, with public evidence under `spikes/tn12-minimal-covenant/artifacts/env-054-tn10-readonly-utxo/`.
10. Treat ENV-055 as ready-only preflight: TN10 ordinary spend syntax and estimate/send path were identified without signing or broadcasting, with public evidence under `spikes/tn12-minimal-covenant/artifacts/env-055-tn10-ordinary-spend-preflight/`.
11. Treat ENV-056 as complete: one live ordinary TN10 send of 1 TKAS succeeded and was confirmed by read-only post-send checks, with public evidence under `spikes/tn12-minimal-covenant/artifacts/env-056-tn10-ordinary-send/`.
12. Do not proceed to covenant lifecycle work without explicit future approval.

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

## ENV-050B mining preflight status

- Status: complete (result = READY)
- No mining started yet
- Faucet route remains blocked
- Mining is now the likely TN12 funding path if a later explicit mining attempt is approved
- Existing ENV-049 address remains the only approved address: `kaspatest:qqaq5f4ju52g9r869c50n55lmtgku9nsf2pc56y76neaj7rksmewg2ytrxccg`
- Current synced node is still running without `--utxoindex`
- Mining preflight evidence path: `spikes/tn12-minimal-covenant/artifacts/env-050b-tn12-mining-preflight/env-050b-summary.txt`
- UTXO/signing/broadcast/covenant lifecycle remains NOT TESTED
- Roulette remains PAUSED

## ENV-050C mining attempt status

- Status: blocked (result = BLOCKED)
- Approved one-thread mining attempt did not start because `kaspa-miner` was not installed/available locally
- Current synced TN12 `kaspad` node remained running; no restart and no `--utxoindex` change were performed
- Existing ENV-049 address remained the only address in scope: `kaspatest:qqaq5f4ju52g9r869c50n55lmtgku9nsf2pc56y76neaj7rksmewg2ytrxccg`
- No miner process connected to the local node
- No UTXO inspection, signing, broadcast, or covenant lifecycle work was performed
- Mining-attempt evidence path: `spikes/tn12-minimal-covenant/artifacts/env-050c-tn12-mining-attempt/env-050c-summary.txt`
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

## ENV-051 TN10 public wRPC proof

ENV-051 established that the live path should pivot from TN12 to TN10.

Reason:
- TN12 funding/mining route remained blocked or unclear.
- Official Rusty Kaspa release evidence points to TN10 Toccata validation via `tn10-toc3`.
- Official `tn10-toc3` `kaspa-wallet` connected successfully to public resolver-backed TN10 wRPC.
- Read-only calls succeeded against TN10:
  - `rpc get-server-info`
  - `rpc get-current-network`
  - `rpc get-sync-status`
  - `rpc get-block-dag-info`

Evidence:
`spikes/tn12-minimal-covenant/artifacts/env-051-tn10-public-rpc-proof/env-051-summary.txt`

Safety:
No wallet/key/faucet/signing/broadcast/covenant action was performed in ENV-051.

## ENV-051 TN10 public wRPC proof

ENV-051 established that the live path should pivot from TN12 to TN10.

Reason:
- TN12 funding/mining route remained blocked or unclear.
- Official Rusty Kaspa release evidence points to TN10 Toccata validation via `tn10-toc3`.
- Official `tn10-toc3` `kaspa-wallet` connected successfully to public resolver-backed TN10 wRPC.
- Read-only calls succeeded against TN10:
  - `rpc get-server-info`
  - `rpc get-current-network`
  - `rpc get-sync-status`
  - `rpc get-block-dag-info`

Evidence:
`spikes/tn12-minimal-covenant/artifacts/env-051-tn10-public-rpc-proof/env-051-summary.txt`

Safety:
No wallet/key/faucet/signing/broadcast/covenant action was performed in ENV-051.


## ENV-052 TN10 test-only wallet/address generation

- Status: complete
- Network: TN10 / `testnet-10`
- Wallet name: `env052-tn10-test-only`
- Public receive address: `kaspatest:qrhszwr4r2ejukpxyjp7jvn40tth5s8zy0538zvkkrvtkxvvyhlmjhe275slx`
- Evidence path: `spikes/tn12-minimal-covenant/artifacts/env-052-tn10-wallet-address/env-052-summary.txt`
- Tool path used: `tools/rusty-kaspa-releases/tn10-toc3/bin/kaspa-wallet`
- Server mode: public resolver-backed TN10 wRPC
- No faucet request, explicit UTXO inspection, signing, broadcast, covenant action, or mainnet action was performed.
- No mnemonic/seed/private key material was committed or included in public artifacts.

## ENV-053 TN10 faucet funding only

- Status: complete
- Network: TN10 / `testnet-10`
- Address funded: `kaspatest:qrhszwr4r2ejukpxyjp7jvn40tth5s8zy0538zvkkrvtkxvvyhlmjhe275slx`
- Faucet source: `https://faucet.kaspanet.io/`
- Faucet URL used: `https://faucet-tn10.kaspanet.io/`
- Amount requested: `1000 TKAS`
- Faucet response: success
- Txid: `29d76273819d519bea146e881554c633bac4d30989bfc8e1862fed965d8f5116`
- Timestamp: `2026-06-25T20:14:49Z`
- Evidence path: `spikes/tn12-minimal-covenant/artifacts/env-053-tn10-faucet-funding/env-053-summary.txt`
- No new wallet/address was created.
- No balance/UTXO inspection, signing, wallet-driven broadcast, covenant action, or mainnet action was performed.
- No private material was accessed or exposed.

## ENV-054 TN10 read-only balance and UTXO inspection

- Status: complete
- Network: TN10 / `testnet-10`
- Address inspected: `kaspatest:qrhszwr4r2ejukpxyjp7jvn40tth5s8zy0538zvkkrvtkxvvyhlmjhe275slx`
- Faucet txid referenced: `29d76273819d519bea146e881554c633bac4d30989bfc8e1862fed965d8f5116`
- Observed balance: `1000.0 TKAS`
- Observed UTXO count: `1`
- Observed UTXO outpoint: `29d76273819d519bea146e881554c633bac4d30989bfc8e1862fed965d8f5116:0`
- Observed server state: `suffix=10`, `is_synced=true`, `has_utxo_index=true`
- Evidence path: `spikes/tn12-minimal-covenant/artifacts/env-054-tn10-readonly-utxo/env-054-summary.txt`
- Transaction lookup attempt via wallet RPC command surface was not available (`No such rpc method: 'get-transaction-by-id'`).
- Commands executed were read-only only.
- No signing, broadcast, spend construction, covenant action, private-material access, or mainnet action was performed.

## ENV-055 TN10 ordinary spend preflight / dry-run only

- Status: ready
- Network: TN10 / `testnet-10`
- Wallet: `env052-tn10-test-only`
- Address in scope: `kaspatest:qrhszwr4r2ejukpxyjp7jvn40tth5s8zy0538zvkkrvtkxvvyhlmjhe275slx`
- Carry-forward balance/UTXO from ENV-054: `1000.0 TKAS`, `1 UTXO`, outpoint `29d76273819d519bea146e881554c633bac4d30989bfc8e1862fed965d8f5116:0`
- Future ordinary spend syntax identified: `send <address> <amount> <priority fee>`
- Estimate syntax identified: `estimate <amount> [<priority fee>]`
- No explicit send dry-run/preview mode was discovered
- Recommended future pre-send check: `estimate 1 0`
- Recommended future tiny send command path: open wallet, run `estimate 1 0`, then `send <fresh-tn10-test-only-recipient-address> 1 0`
- Evidence path: `spikes/tn12-minimal-covenant/artifacts/env-055-tn10-ordinary-spend-preflight/env-055-summary.txt`
- No signing, broadcast, covenant action, private-material exposure, or mainnet action was performed.

## ENV-056 TN10 live ordinary send of 1 TKAS

- Status: complete
- Network: TN10 / `testnet-10`
- Source wallet: `env052-tn10-test-only`
- Source address: `kaspatest:qrhszwr4r2ejukpxyjp7jvn40tth5s8zy0538zvkkrvtkxvvyhlmjhe275slx`
- Recipient address: `kaspatest:qpf0pc97d4vxtd99gppqtzhrjtna4t396lvu2t249p9f0rkh05pxxc5mj9yf2`
- Amount sent: `1 TKAS`
- Priority fee: `0`
- Estimate observed before send: fees `0.002036 TKAS`, total `1.002036 TKAS`, UTXOs `1`
- Confirmed transaction id: `c9b4532f217d66987997e972963ec5dbfa5a9e7bf18f3e38910763274fb05135`
- Post-send read-only confirmation: source balance `0.0`, recipient balance `1.0`, wallet total `999.997964 TKAS`, change output `998.997964 TKAS`
- Evidence path: `spikes/tn12-minimal-covenant/artifacts/env-056-tn10-ordinary-send/env-056-summary.txt`
- No covenant action, mainnet action, or private-material exposure occurred.
