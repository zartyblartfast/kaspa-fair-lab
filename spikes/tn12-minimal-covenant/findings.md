# TN12 Minimal Covenant Spike Findings

## Status

- _Date_: not started
- _Phase_: planning
- _Outcome_: **Unverified**

## Test matrix (to be filled)

| Test | Command / Input | Result | Verified | Notes |
| --- | --- | --- | --- | --- |
| Create minimal covenant artefact | (to be added) | Unknown | No | Toolchain/path not yet run |
| Spend artefact in follow-up tx | (to be added) | Unknown | No | Depends on creation success |
| Inspect transaction and artefact fields | (to be added) | Unknown | No | Need concrete output sample |
| Explain the flow to reviewer | (to be added) | Unknown | No | Requires complete artifact capture |

## Known

- No implementation of covenant logic exists in repo yet.
- This spike is constrained to TN12/testnet exploration.

## Unknown

- Whether current tooling fully supports the full cycle.
- Whether inspection output contains enough detail for independent explanation.

## Assumptions

- Network endpoints and wallets are available when execution is attempted.
- Required dependencies can be installed without adding heavy packages.

## Verification record

To be updated after each run.

For each run, record the following blocks:

```text
Run ID: <short name>
Date/time: <UTC timestamp>
Network: TN12/testnet

Observed (factual):
- commands run:
  - <command>
- raw outputs:
  - <output excerpt or artifact path>
- success/failure: <pass/fail>

Assumptions:
- <assumption 1>
- <assumption 2>

Unverified:
- <what was not demonstrated>

Notes:
- <why this run passed/failed and caveats>
- <next follow-up action>
```

- Every finding must include command-output references and explicit caveats.
- Keep each run block self-contained and avoid claiming outcome without evidence references.

## Concrete starter entry (copy/paste)

```text
Run ID: run-001-minimal-covenant-planned
Date/time: 2026-06-23T00:00:00Z
Network: TN12/testnet

Observed (factual):
- commands run:
  - ./scripts/check-env.sh
  - <tn12_create_command>
  - <tn12_spend_command>
  - <inspect_command>
- raw outputs:
  - ./artifacts/run-001-env.txt
  - ./artifacts/run-001-tx-create.txt
  - ./artifacts/run-001-tx-spend.txt
- success/failure: failure (blocked by missing precondition)

Assumptions:
- Node/wallet tooling versions in this environment are representative for the spike.
- Network RPC availability on TN12 is stable during the run.

Unverified:
- No real covenant tx was observed yet (no live commands executed in this run).
- Fairness explanation depth not yet validated.

Notes:
- check-env reported environment status; covenant commands were not yet run.
- Next follow-up action: execute the first full command sequence and attach txid/output artifacts.
```
