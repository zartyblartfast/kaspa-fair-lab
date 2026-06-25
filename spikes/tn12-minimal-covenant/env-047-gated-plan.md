# ENV-047 gated TN12-only planning document

Status: planning-only

Purpose of ENV-047:
- define the next-phase gated TN12-only execution sequence after ENV-046
- document the approval boundaries before any wallet, faucet, signing, broadcast, or covenant lifecycle work occurs
- keep the next phase evidence-first, incremental, and reversible

This document does not authorize execution.

Hard scope limits for this planning pass:
- do not create a wallet
- do not generate or store private keys yet
- do not request faucet funds
- do not sign transactions
- do not broadcast transactions
- do not use mainnet
- do not build roulette
- do not create a web app

Current baseline from ENV-046:
- ENV-046 is complete and pushed
- local TN12 synced node status is GREEN, with the documented RAM/swap caveat
- local TN12 read-only RPC confirmation is GREEN
- synced-node evidence is preserved under `spikes/tn12-minimal-covenant/artifacts/env-046-rpc-readonly-suite/`
- `hasUtxoIndex=false` was observed
- wallet/faucet/signing/broadcast/covenant lifecycle remains NOT TESTED
- roulette remains PAUSED

Global rules for all ENV-047 gates:
- TN12/testnet only
- explicit approval is required before executing each gate
- stop immediately if a gate would exceed the approved scope
- prefer repo-owned artifacts under `spikes/tn12-minimal-covenant/artifacts/`
- do not treat completion of one gate as implicit approval for the next gate

## Gate 1: key/address generation

Purpose:
- establish a test-only TN12 identity/address for later funding and inspection steps

Preconditions:
- ENV-046 baseline remains the current readiness baseline
- the user explicitly approves key/address generation
- an approved TN12-only route is identified before any command is run
- a plan exists for excluding any secret material from committed artifacts

Likely command/tool, if known:
- not yet fixed
- likely either a Kaspa wallet CLI or a small repo-owned Rust helper
- exact command should be documented only after the tool choice is explicitly approved

Expected evidence artifact:
- `spikes/tn12-minimal-covenant/artifacts/env-047-key-address-generation.txt`
- should record the command/tool used, the resulting TN12 address, and confirmation that private key material was not stored in the repo artifact

Explicit approval required before execution:
- yes
- approval must explicitly allow test-only key/address generation

Stop condition:
- stop if the only available route would print, persist, or expose private key material unsafely
- stop if the route is not clearly TN12/testnet-only
- stop if the route would create wallet/key state outside the explicitly approved location

Risks / unknowns:
- the safest approved tool for TN12-only key/address generation is not yet fixed
- private key handling policy for later signing has not yet been approved
- the exact artifact format for proving address generation without leaking secrets is not yet fixed

Safety boundary:
- no faucet request follows automatically from address generation
- no private keys, seed phrases, or unsafe secret material may be committed
- no mainnet address generation

## Gate 2: faucet funding

Purpose:
- fund the approved TN12 test-only address with the minimum amount needed for later read-only UTXO inspection and ordinary transaction planning

Preconditions:
- Gate 1 has been explicitly approved and completed
- the funding address is already captured in a safe artifact
- a specific TN12 faucet path has been identified and approved
- the funding amount is intentionally minimal

Likely command/tool, if known:
- not yet fixed
- likely an approved TN12 faucet web, API, or manual request path

Expected evidence artifact:
- `spikes/tn12-minimal-covenant/artifacts/env-047-faucet-funding.txt`
- should record the address funded, faucet route used, timestamp, requested amount, and txid or equivalent acknowledgment if available

Explicit approval required before execution:
- yes
- approval must explicitly allow faucet funding for the approved test-only TN12 address

Stop condition:
- stop if the faucet route is unofficial, ambiguous, or broader than expected
- stop if no minimally verifiable funding record is produced
- stop if the faucet appears to target the wrong network

Risks / unknowns:
- the reproducible TN12 faucet route is not yet fixed
- faucet availability, rate limits, and confirmation timing may vary
- the exact minimum practical funding amount has not yet been justified in a captured plan

Safety boundary:
- TN12 faucet only
- minimum funding only
- no signing or broadcast approval is implied by faucet success

## Gate 3: read-only UTXO inspection

Purpose:
- verify that funded UTXOs for the approved TN12 address are visible before any spend construction begins

Preconditions:
- Gate 2 has been explicitly approved and completed
- the funded address is known and recorded safely
- a read-only inspection route has been identified and approved
- the inspection route can produce evidence without requiring unapproved live changes

Likely command/tool, if known:
- exact command not yet fixed
- likely read-only RPC against the local TN12 node or another approved read-only inspection tool
- note: ENV-046 observed `hasUtxoIndex=false`, so this gate may require either a non-index-dependent inspection route or a separately approved node/index configuration change

Expected evidence artifact:
- `spikes/tn12-minimal-covenant/artifacts/env-047-utxo-inspection.txt`
- should record the query method, inspected address, and concrete observed UTXO fields

Explicit approval required before execution:
- yes
- approval must explicitly allow the chosen read-only UTXO inspection path

Stop condition:
- stop if funded UTXOs cannot be inspected reproducibly
- stop if the only route requires non-localhost exposure or other unapproved infrastructure changes
- stop if the evidence does not clearly prove the funded outputs

Risks / unknowns:
- current synced-node evidence shows `hasUtxoIndex=false`
- the exact TN12 UTXO inspection route that fits the project constraints is not yet fixed
- confirmation timing and data visibility may differ across candidate inspection routes

Safety boundary:
- read-only only
- no transaction construction, signing, or broadcast in this gate
- no mainnet lookups

## Gate 4: ordinary transaction construction

Purpose:
- prove the baseline non-covenant spend path on TN12 before attempting any covenant-bound lifecycle

Preconditions:
- Gate 3 has been explicitly approved and completed
- funded UTXOs are known and inspectable
- a recipient/destination policy has been defined for the test-only ordinary transaction
- fee handling assumptions are documented before construction begins

Likely command/tool, if known:
- `spikes/tn12-minimal-covenant/rust-tx-assembly` is the most likely starting point
- likely command shape if approved later:
  - `cd /root/kaspa-fair-lab/spikes/tn12-minimal-covenant/rust-tx-assembly && cargo run`
- expected to require adaptation from the current local-only unsigned scaffold to real funded-input construction

Expected evidence artifact:
- `spikes/tn12-minimal-covenant/artifacts/env-047-ordinary-transaction-summary.txt`
- optionally a structured request or hex artifact if produced safely and reproducibly

Explicit approval required before execution:
- yes
- approval must explicitly allow ordinary TN12 transaction construction using funded inputs

Stop condition:
- stop if funded inputs, fee selection, destination handling, or change handling are still ambiguous
- stop if the construction path depends on undocumented assumptions
- stop if the result cannot be inspected deterministically before signing

Risks / unknowns:
- the current scaffold is local-only and unsigned
- the exact funded-input construction path has not yet been exercised on TN12 artifacts
- fee and change behavior for the minimal test flow is not yet evidence-backed in this repo

Safety boundary:
- construction only
- no signing and no broadcast in this gate
- no covenant-bound logic yet

## Gate 5: signing

Purpose:
- prove that the ordinary TN12 transaction can be signed correctly using approved test-only key material

Preconditions:
- Gate 4 has been explicitly approved and completed
- the unsigned ordinary transaction artifact exists and is inspectable
- the signing key path has been explicitly approved
- secret handling rules are documented before signing begins

Likely command/tool, if known:
- likely Rusty Kaspa signing functions identified in prior findings:
  - `sign_with_multiple_v2(...)`
  - `sign(...)`
  - `sign_input(...)`
- exact wrapper command/tool is not yet fixed

Expected evidence artifact:
- `spikes/tn12-minimal-covenant/artifacts/env-047-signed-transaction-summary.txt`
- should record the signing route used and the signed transaction summary without exposing unsafe secret material

Explicit approval required before execution:
- yes
- approval must explicitly allow signing with approved test-only key material

Stop condition:
- stop if key handling is unclear or unsafe
- stop if signed output format is ambiguous
- stop if signatures cannot be inspected or reproduced confidently enough for the next gate

Risks / unknowns:
- the exact secure signing wrapper/tool has not yet been selected
- artifact design must prove signing happened without leaking private material
- the deterministic handling of signatures/witness data is not yet documented in this repo

Safety boundary:
- signing approval does not imply broadcast approval
- no private keys, seeds, or secret material may be committed
- TN12 only

## Gate 6: broadcast

Purpose:
- test live TN12 network acceptance for the signed ordinary transaction before any covenant-bound live flow is attempted

Preconditions:
- Gate 5 has been explicitly approved and completed
- the signed ordinary transaction exists as an inspectable artifact
- the submission route is explicitly chosen and approved
- the planned broadcast step is limited to one clearly identified ordinary transaction

Likely command/tool, if known:
- exact command not yet fixed
- likely an RPC submission path built around the already-proven `RpcTransaction` and `SubmitTransactionRequest` object route

Expected evidence artifact:
- `spikes/tn12-minimal-covenant/artifacts/env-047-broadcast-result.txt`
- should record the submission method, txid, immediate RPC response, and any error or acceptance status

Explicit approval required before execution:
- yes
- approval must explicitly allow live TN12 broadcast

Stop condition:
- stop if the submission route differs materially from the approved plan
- stop if mempool rejection occurs without clear diagnosis
- stop if the network response cannot be captured reproducibly

Risks / unknowns:
- live TN12 mempool acceptance has not yet been proven in this repo
- the exact broadcaster/tooling path is not yet fixed
- network timing and relay behavior may complicate evidence capture

Safety boundary:
- TN12 only
- one explicitly approved broadcast at a time
- no covenant-bound broadcast is implied by ordinary transaction broadcast success

## Gate 7: covenant-bound create / spend / inspect

Purpose:
- prove the target lifecycle by creating a covenant-bound output, spending it under the intended rules, and inspecting both stages with evidence

Preconditions:
- Gate 6 has been explicitly approved and completed for the ordinary path
- the ordinary create/sign/broadcast baseline is already proven
- covenant bytecode/binding assumptions are documented before live use
- a minimal create/spend/inspect sequence is defined before execution

Likely command/tool, if known:
- likely a combination of:
  - repo-owned SilverScript fixture knowledge from `external/silverscript`
  - Rust transaction assembly for real TN12 create/spend objects
  - read-only RPC inspection for post-broadcast verification, if separately approved
- exact command set is not yet fixed

Expected evidence artifact:
- `spikes/tn12-minimal-covenant/artifacts/env-047-covenant-lifecycle-summary.txt`
- plus any create/spend/inspect sub-artifacts needed to show txids, covenant binding, and observed chain state

Explicit approval required before execution:
- yes
- approval must explicitly allow covenant-bound live execution
- if preferred, approval should distinguish create, spend, and inspect as separate sub-gates

Stop condition:
- stop if the ordinary transaction path is not already proven first
- stop if covenant bytecode, binding, funding, or spend assumptions remain ambiguous
- stop if inspection cannot independently prove what was created and spent
- stop if the live path would require scope broader than what was approved

Risks / unknowns:
- the end-to-end covenant lifecycle remains unproven on live TN12 in this repo
- the minimal safe create/spend funding pattern is not yet fixed
- post-broadcast inspection requirements may be more complex than the ordinary path

Safety boundary:
- TN12 only
- proceed in the smallest separately evidencable increments
- roulette remains PAUSED until this lifecycle is proven and separately approved for any downstream use

## Recommended approval order

1. approve planning only
2. approve key/address generation only
3. approve faucet funding only
4. approve read-only UTXO inspection only
5. approve ordinary transaction construction only
6. approve signing only
7. approve broadcast only
8. approve covenant-bound create/spend/inspect only after the ordinary path is proven

## Planning conclusion

ENV-047 remains planning-only. No wallet, no key generation/storage, no faucet funding, no signing, no broadcast, no mainnet work, no roulette build, and no web app work were performed while creating this document.