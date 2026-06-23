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

## Environment check run

- **Run ID:** env-002
- **Date/time:** 2026-06-23T13:32:06Z
- **Network:** TN12/testnet (not networked in this check)

Observed (factual):
- Command:
  - `./scripts/check-env.sh`
- Raw output summary:
  - OK: `git` `node` `npm` `python3` `cargo` `rustc` `codex`
  - Exit code: `0`

Success/failure: **pass** (all listed tools were present)

Assumptions:
- Command output lines are from this host/session only.

Unverified:
- No covenant-tooling command was executed.
- No SilverScript / Rusty Kaspa / WASM SDK / Python SDK command availability was proven in this run.

## Notes:
- The earlier `cargo`/`rustc` failure is most likely from shell/session PATH visibility immediately after Rust installation rather than a failed Rust install.
- The task requirement “run check-env and record result” is satisfied.
- Next action: proceed with route-discovery checks in README plan using the next run block.

## env-003 tooling route discovery

- **Run ID:** env-003
- **Date/time:** 2026-06-23T13:36:27Z
- **Network:** TN12/testnet (not networked in this run)

Observed (factual):
- Files reviewed:
  - `docs/toccata-feasibility.md`
  - `spikes/tn12-minimal-covenant/README.md`
  - `spikes/tn12-minimal-covenant/findings.md`
- Route candidates are documented from existing project docs only; no new covenant tooling was executed.

Success/failure: **pass** for route-discovery documentation pass (no command-path claims made)

### Candidate route assessment

| Route | Intended use | What needs install/clone later | Evidence needed to prove viable | Risks / unknowns |
| --- | --- | --- | --- | --- |
| SilverScript | Highest-level covenant script authoring path for building/signing minimal artefacts with least custom transaction plumbing. | If available, a SilverScript CLI/tool install or repository checkout (to be confirmed later), plus any node-based runtime used by examples. | Presence of command/help, minimal compile example output, and successful create/spend/inspect command traces on TN12. | Untested locally; may be unavailable, immature, or not TN12-ready. Current docs only contain planned probe steps. |
| Rusty Kaspa / Rust crates | Lower-level, likely more reliable control of script/payload construction and transaction assembly through native libraries. | Rust crate dependencies and source artifacts (to be fetched later), plus any project/tool templates for signing and tx submission. | Verified crate API docs/examples, successful minimal compile and run that produces a tiny artefact and spend with inspectable outputs. | Heavier setup/build time and extra dependency surface; requires compiling and version alignment before live testing. |
| WASM SDK | Client-side/helper route for covenant payload building via wasm tooling if available. | WASM runtime/tooling plus relevant SDK package(s) or repo checkout. | A minimal wasm helper run that emits expected covenant payload and successful handoff to TN12 transaction tooling. | Could be simulation-only or require browser-specific glue not available in this CLI environment. |
| Python SDK | Orchestration route for automation/inspection if mature Kaspa/Toccata Python bindings exist. | Python package installation + module import availability (to be done later). | Confirmed module import/version, then successful command output that creates, spends, and inspects a tiny artefact. | No explicit Python covenant path exists in current docs; may not support covenant payload compilation. |

Notes:
- All candidate viability items remain **UNVERIFIED** until live outputs are produced.
- `docs/toccata-feasibility.md` still states cargo/rustc were previously missing; this should be updated in a later documentation-cleanup pass.

## env-004 tool-check rerun after Rust/Cargo PATH refresh

- **Run ID:** env-004
- **Date/time:** 2026-06-23T13:38:30Z
- **Network:** TN12/testnet (not networked in this check)

Observed (factual):
- Command:
  - `./scripts/check-env.sh`
- Raw output summary:
  - OK: `git` `node` `npm` `python3` `cargo` `rustc` `codex`
  - Exit code: `0`

Success/failure: **pass**

Assumptions:
- Output reflects this host/session after PATH refresh.

Unverified:
- No covenant-tooling command was executed.

Notes:
- The earlier `cargo`/`rustc` failure remains most consistent with shell/session PATH visibility immediately after installation rather than a Rust toolchain failure.
- No additional environment changes were made for this update; this is a revalidation only.

## env-005 tooling route discovery

- **Run ID:** env-005
- **Date/time:** 2026-06-23T13:43:18Z
- **Network:** TN12/testnet (not networked in this check)

Observed (factual):
- Files reviewed:
  - `docs/toccata-feasibility.md`
  - `spikes/tn12-minimal-covenant/README.md`
  - `spikes/tn12-minimal-covenant/findings.md`
- No covenant runtime/tooling command was executed.

Success/failure: **pass** for documentation pass (route candidates captured, no runtime claims yet)

### Candidate route assessment

| Route | Possible use | What would need to be installed/cloned later | Evidence needed to prove viability | Risks / unknowns |
| --- | --- | --- | --- | --- |
| SilverScript | High-level covenant script workflow for building/spending/inspecting a tiny artefact with minimal custom tx plumbing. | Need a working SilverScript distribution path (CLI/package or repository checkout) and any runtime/docs assumptions it requires. | Presence of a usable `silverscript` command (or equivalent), successful help/usage output, and a live TN12 create/spend/inspect sequence with concrete tx/artifact outputs. | Current docs only indicate this as a candidate; actual command availability and TN12 compatibility are unproven. |
| Rusty Kaspa / Rust crates | Low-level/native construction and signing path for script payloads and transactions, suitable for explicit covenant inspection. | Need Rust crate sources/dependencies for Kaspa/Toccata plus any project templates needed for tx assembly/submission. | Successful minimal compile/build of a tiny create flow, followed by successful spend and inspection outputs on TN12. | Requires compile/build effort and dependency alignment; heavier setup footprint and longer feedback loop. |
| WASM SDK | JavaScript/wasm-assisted path for constructing covenant payloads or helper logic before signing/submitting via other tooling. | Need WASM toolchain/runtime, package modules, and source package references for available examples. | A runnable WASM invocation that generates a usable covenant payload and a successful handoff into a TN12 tx pipeline with inspectable output. | Might be simulation-only, browser-focused, or missing TN12 covenant examples in current repos. |
| Python SDK | Automation/orchestration path for scripting checks, wallet setup, and inspection if relevant Kaspa/Toccata Python bindings exist. | Need Python package install and module availability (to be done later), plus any companion tooling for tx signing/submission. | Confirmed import/version and a live command path that creates an artefact, spends it, and prints inspection details. | No direct Python covenant path is confirmed in current spike docs; may only support inspection or be unavailable. |

Notes:
- All candidate routes remain **UNVERIFIED** until a live command sequence and output artifacts are captured in this file.
- `docs/toccata-feasibility.md` still states Rust tools were previously missing, which conflicts with the latest check output; treat that as historical context until reconciled in a cleanup pass.

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
