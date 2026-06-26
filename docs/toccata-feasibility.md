# Toccata Feasibility Notes

## Objective

Evaluate whether the project can meet the minimum technical promise:

- create a small Toccata covenant artefact,
- spend/close it,
- inspect it,
- and explain it in reviewer-friendly terms.

## Required local tooling

- `git`, `node`, `npm`, `python3`, `cargo`, `rustc`, `codex`
- Current repo remote remains:
  - `https://github.com/zartyblartfast/kaspa-fair-lab.git`
- Local TN12 synced-node read-only RPC evidence is preserved under:
  - `spikes/tn12-minimal-covenant/artifacts/env-046-rpc-readonly-suite/`
- Working VPS TN12 procedure is documented in:
  - `spikes/tn12-minimal-covenant/tn12-node-runbook.md`

## What is being tested

1. **Toolchain readiness**: available CLIs/scripts can run the needed operations on TN12.
2. **Transaction flow viability**: create + spend steps can be executed and repeated.
3. **Inspectability**: outputs are deterministic and interpretable.
4. **Verifiability**: an external observer can confirm claims from raw outputs and documented assumptions.

## Known / unknown / assumed

### Known

- No technical guarantee exists yet that the full flow is supported today.
- TN12 is a test environment and may differ materially from future or mainnet behavior.
- Covenant-related tooling evolves quickly; versions matter.
- Local TN12 full sync is now GREEN, with a RAM/swap caveat.
- Read-only RPC confirmation is now GREEN.
- Earlier `kaspad` exits were caused by the Linux OOM killer.
- Adding 8 GB swap allowed sync to complete on the VPS.
- `hasUtxoIndex: false` was observed on the synced-node read-only RPC check.
- TN10 wallet/faucet/signing/broadcast and live corrected covenant create/spend/settlement are tested on testnet-10; TN12 and mainnet remain NOT TESTED.
- Roulette remains PAUSED.

### Unknown

- Whether the TN10-proven flow transfers unchanged to TN12 or any future/mainnet Toccata environment.
- Exact minimum metadata required for reliable inspection.
- Whether the observed outputs are sufficient for independent explanation.
- Which failure cases are expected vs. infra-induced artifacts.

### Assumptions

- Project participants have access to TN12/testnet endpoints.
- Any required wallet/private-key operations are within environment security policy.
- Verification can be done via command output and logs without production-like infrastructure.

## What remains unverified (explicitly)

- Any claim that covenant creation/spend/inspection works on TN12 or mainnet.
- Any statement that a roulette architecture is currently safe to implement.
- Any performance, throughput, or cost assumptions.
- Any claim that roulette/web-app integration has been tested, or that the TN10 spike result is a mainnet result.

## Current TN12 infrastructure conclusion

- The infrastructure path for a localhost-only TN12 node plus read-only RPC verification is now feasible and evidence-backed.
- The remaining risk is not whether a synced node can run at all, but whether later wallet/UTXO/signing/covenant lifecycle work can be proven safely under the project constraints.

Every time results appear, this file should be updated with links to command transcripts and hashes of evidence artifacts.

## Criteria for a positive finding

A positive feasibility call requires all of the following in a spike report:

- Reproducible command transcript for create and spend.
- Successful inspection results with consistent interpretation.
- Documented assumptions for anything not directly observed.
- Clear statement of failure modes and recovery steps.

Anything less should be labeled `Unverified` and carried as risk to planning.

## Final TN10 covenant spike evidence (ENV-066)

Result: COMPLETE for the constrained TN10 covenant feasibility spike. The corrected live path is evidence-backed by:

- ENV-063 corrected live TN10 covenant create: accepted txid `2c7802ff9a6eec2828a96168d8f62a9a276176441ed8cb6086cd5d5d0cb26849`; output `2c7802ff9a6eec2828a96168d8f62a9a276176441ed8cb6086cd5d5d0cb26849:0` observed unspent after mining with covenant id `e2bdd874add81ebcdba4d0f9ef650967ddadf1085ce4ab15f5eb29fddbf79ff7`.
- ENV-064 corrected live TN10 covenant spend: accepted spend txid `4cb31dbad4465665b978ba3ec5eeecb21824a3ea686f5085b46a97066446466c`, spending the ENV-063 corrected covenant UTXO only.
- ENV-065 read-only settlement confirmation: original ENV-063 UTXO absent/spent; continuing output `4cb31dbad4465665b978ba3ec5eeecb21824a3ea686f5085b46a97066446466c:0` visible as a UTXO with `99700000` sompi and covenant id `e2bdd874add81ebcdba4d0f9ef650967ddadf1085ce4ab15f5eb29fddbf79ff7`.

Important scope notes:

- Network proven: TN10 / testnet-10 only.
- The old ENV-060C/ENV-061 UTXO `f4941c478e9540c477e04d0a2dff7ab1b0d0d794a3ae453c8148d25d125fe53d:0` was superseded by the corrected v1 path and was not used for final corrected feasibility.
- ENV-066 performed documentation/evidence consolidation only: no signing, submitting, broadcasting, transaction creation, spend action, mainnet action, wallet-secret access, helper-private-key exposure, roulette, or web-app work.
- Final evidence index: `spikes/tn12-minimal-covenant/artifacts/env-066-final-covenant-spike-summary/evidence-index.txt`.
