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
- Wallet, faucet, signing, broadcast, and live covenant operations remain NOT TESTED.
- Roulette remains PAUSED.

### Unknown

- Whether artefact creation and spend can be completed without undocumented side steps.
- Exact minimum metadata required for reliable inspection.
- Whether the observed outputs are sufficient for independent explanation.
- Which failure cases are expected vs. infra-induced artifacts.

### Assumptions

- Project participants have access to TN12/testnet endpoints.
- Any required wallet/private-key operations are within environment security policy.
- Verification can be done via command output and logs without production-like infrastructure.

## What remains unverified (explicitly)

- Any claim that covenant creation/spend/inspection works end-to-end.
- Any statement that a roulette architecture is currently safe to implement.
- Any performance, throughput, or cost assumptions.
- Any claim that wallet, faucet, signing, broadcast, or live covenant operations have been tested.

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
