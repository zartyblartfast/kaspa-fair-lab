# Toccata Feasibility Notes

## Objective

Evaluate whether the project can meet the minimum technical promise:

- create a small Toccata covenant artefact,
- spend/close it,
- inspect it,
- and explain it in reviewer-friendly terms.

## Required local tooling

- `git`, `node`, `npm`, `python3`, `cargo`, `rustc`, `codex`
- Current VPS check (`./scripts/check-env.sh`) indicates missing:
  - `cargo`
  - `rustc`
- TODO install steps (not executed in this task):
  - `curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh`
  - `source ~/.cargo/env`
  - `rustup default stable`
  - `rustc --version`
  - `cargo --version`

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

Every time results appear, this file should be updated with links to command transcripts and hashes of evidence artifacts.

## Criteria for a positive finding

A positive feasibility call requires all of the following in a spike report:

- Reproducible command transcript for create and spend.
- Successful inspection results with consistent interpretation.
- Documented assumptions for anything not directly observed.
- Clear statement of failure modes and recovery steps.

Anything less should be labeled `Unverified` and carried as risk to planning.
