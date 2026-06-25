# ENV-047 gated execution plan

Status: planning-only

Scope: define the next gated TN12 sequence after ENV-046 without executing any wallet, faucet, signing, broadcast, or mainnet action.

Current baseline from ENV-046:
- local TN12 full sync: GREEN (RAM/swap caveat)
- local read-only RPC confirmation: GREEN
- synced-node evidence preserved under `spikes/tn12-minimal-covenant/artifacts/env-046-rpc-readonly-suite/`
- `hasUtxoIndex=false` was observed
- wallet/faucet/signing/broadcast/live covenant operations remain NOT TESTED
- roulette remains PAUSED

Global safety boundaries for ENV-047:
- TN12/testnet only
- explicit approval required before each gate is executed
- stop immediately if any step would imply unstated wallet creation, faucet use, signing, broadcast, or mainnet access
- keep evidence in repo-owned artifacts under `spikes/tn12-minimal-covenant/artifacts/`

## Gate 1: key/address generation

Purpose:
- establish a test-only identity/address for later TN12 funding and spend planning

Likely command/tool:
- not yet fixed
- likely either a dedicated Kaspa wallet CLI or a small repo-owned Rust helper, depending on what is approved

Expected evidence artifact:
- `spikes/tn12-minimal-covenant/artifacts/env-047-key-address-generation.txt`
- should record command, generated test-only address, and whether any secret material was intentionally excluded from the artifact

Explicit approval required before execution:
- yes
- approval must explicitly allow wallet/key generation

Rollback / stop condition:
- stop if the only available route would expose private key material unsafely
- stop if the route would create persistent wallet state outside the approved repo/VPS scope
- stop if the route is not clearly TN12/testnet-only

Safety boundary:
- do not proceed to faucet use automatically after address generation
- do not publish or commit sensitive secret material
- do not reuse for mainnet

## Gate 2: faucet funding

Purpose:
- fund the approved TN12 test-only address with a minimal amount sufficient for later inspection and ordinary transaction tests

Likely command/tool:
- not yet fixed
- likely TN12 faucet web/API/manual request path, to be approved separately

Expected evidence artifact:
- `spikes/tn12-minimal-covenant/artifacts/env-047-faucet-funding.txt`
- should record address funded, faucet request method, timestamp, and resulting txid or equivalent acknowledgment if available

Explicit approval required before execution:
- yes
- approval must explicitly allow faucet funding for the previously approved test-only address

Rollback / stop condition:
- stop if faucet path is unclear, unofficial, rate-limited in a way that undermines reproducibility, or requests broader permissions than expected
- stop if no minimally verifiable funding artifact is produced

Safety boundary:
- TN12 faucet only
- minimum funding only
- no automatic follow-on signing or broadcast approval implied

## Gate 3: read-only UTXO inspection

Purpose:
- verify that funded UTXOs for the approved TN12 address are visible and inspectable before any spend construction

Likely command/tool:
- exact command not yet fixed
- likely read-only RPC against the local TN12 node or another approved read-only inspection tool
- note: ENV-046 observed `hasUtxoIndex=false`, so this gate may require an approved inspection route that does not depend on unavailable UTXO index support, or a separately approved node/index configuration change

Expected evidence artifact:
- `spikes/tn12-minimal-covenant/artifacts/env-047-utxo-inspection.txt`
- should record query method, address inspected, and concrete UTXO fields observed

Explicit approval required before execution:
- yes
- approval must explicitly allow the chosen read-only UTXO inspection method

Rollback / stop condition:
- stop if UTXOs cannot be inspected reproducibly
- stop if the only route requires non-localhost exposure or unapproved node reconfiguration
- stop if the evidence does not clearly prove the funded output set

Safety boundary:
- read-only only
- no transaction construction, signing, or broadcast in this gate

## Gate 4: ordinary transaction construction

Purpose:
- prove the non-covenant baseline spend path by constructing an ordinary TN12 transaction from known funded inputs before attempting covenant-bound flows

Likely command/tool:
- `spikes/tn12-minimal-covenant/rust-tx-assembly` Rust scaffold is the most likely starting point
- likely command shape if approved later:
  - `cd /root/kaspa-fair-lab/spikes/tn12-minimal-covenant/rust-tx-assembly && cargo run`
- expected to require adaptation from current local-only unsigned scaffold to real funded-input construction

Expected evidence artifact:
- `spikes/tn12-minimal-covenant/artifacts/env-047-ordinary-transaction-summary.txt`
- optionally a structured hex/request artifact if produced safely and reproducibly

Explicit approval required before execution:
- yes
- approval must explicitly allow ordinary transaction construction using funded TN12 inputs

Rollback / stop condition:
- stop if funded inputs, fees, or destination handling are still ambiguous
- stop if construction depends on undocumented assumptions not backed by artifacts
- stop if the result cannot be inspected deterministically before signing

Safety boundary:
- construction only unless later signing approval is granted
- no broadcast implied by artifact creation

## Gate 5: signing

Purpose:
- prove that the ordinary transaction can be signed correctly with the approved test-only key path

Likely command/tool:
- likely Rusty Kaspa signing functions identified in prior findings:
  - `sign_with_multiple_v2(...)`
  - `sign(...)`
  - `sign_input(...)`
- exact wrapper command/tool is not yet fixed

Expected evidence artifact:
- `spikes/tn12-minimal-covenant/artifacts/env-047-signed-transaction-summary.txt`
- should record signed transaction id / summary fields and the exact signing route used, without exposing unsafe secret material

Explicit approval required before execution:
- yes
- approval must explicitly allow signing with the approved test-only key material

Rollback / stop condition:
- stop if key handling is unclear or unsafe
- stop if signatures cannot be reproduced or inspected confidently
- stop if signed output format is ambiguous

Safety boundary:
- signing approval does not imply broadcast approval
- no secret key material should be committed into repo artifacts

## Gate 6: broadcast

Purpose:
- test live TN12 network acceptance for the ordinary signed transaction before attempting any covenant-bound transaction lifecycle

Likely command/tool:
- exact command not yet fixed
- likely RPC submission path built around the already-proven `RpcTransaction` / `SubmitTransactionRequest` object route

Expected evidence artifact:
- `spikes/tn12-minimal-covenant/artifacts/env-047-broadcast-result.txt`
- should record submission method, txid, immediate RPC response, and any error or acceptance status

Explicit approval required before execution:
- yes
- approval must explicitly allow live TN12 broadcast

Rollback / stop condition:
- stop if mempool rejection occurs without clear diagnosis
- stop if submission route differs materially from the approved plan
- stop if network response cannot be captured reproducibly

Safety boundary:
- TN12 only
- one explicitly approved broadcast step at a time
- no covenant-bound broadcast should be attempted just because ordinary broadcast succeeds

## Gate 7: covenant-bound create / spend / inspect

Purpose:
- prove the full target lifecycle: create a covenant-bound artifact, spend it under the intended rule set, and inspect both stages with captured evidence

Likely command/tool:
- likely combination of:
  - repo-owned SilverScript fixture knowledge from `external/silverscript`
  - Rust transaction assembly path for real TN12 create/spend objects
  - read-only RPC inspection for post-broadcast verification if broadcast is separately approved for each step
- exact command set is not yet fixed

Expected evidence artifact:
- `spikes/tn12-minimal-covenant/artifacts/env-047-covenant-lifecycle-summary.txt`
- plus any create/spend/inspect sub-artifacts needed to show txids, script/covenant binding, and observed chain state

Explicit approval required before execution:
- yes
- approval must explicitly allow covenant-bound live execution, and should distinguish create, spend, and inspect if separate approvals are preferred

Rollback / stop condition:
- stop if ordinary transaction path is not already proven first
- stop if covenant bytecode/binding assumptions are still ambiguous
- stop if inspection cannot independently prove what was created and spent
- stop if any step would require broader scope than explicitly approved

Safety boundary:
- TN12 only
- proceed in smallest separately evidencable increments
- roulette work remains out of scope until this lifecycle is proven

## Recommended approval order

1. approve planning only (this document)
2. approve key/address generation only
3. approve faucet funding only
4. approve read-only UTXO inspection only
5. approve ordinary transaction construction only
6. approve signing only
7. approve broadcast only
8. approve covenant-bound create/spend/inspect only after the ordinary path is proven

## Planning conclusion

ENV-047 should remain planning-only until the user explicitly approves the first live gate. ENV-046 is the current readiness baseline; ENV-047 is a gated execution map, not authorization to perform any live action.
