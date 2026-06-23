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

## env-006 SilverScript read-only live probe

- **Run ID:** env-006
- **Date/time:** 2026-06-23T13:53:11Z
- **Network:** TN12/testnet (not networked in this check)

Observed (factual):
- Local command discovery (read-only):
  - `command -v silverscript` -> not found
  - `command -v silver` -> not found
  - `command -v ssc` -> not found
- Because no command existed, no `--help` or `--version` runs were possible.
- Repo text search for SilverScript notes/examples (`silver`, `silverscript`, `ssc`) found only existing spike planning references; no runnable command examples or tool-specific setup commands were found.
- Metadata scan of local files/docs found no official SilverScript source/repo/package identifier.

Success/failure: **pass** for read-only probe execution, with **fail** for current SilverScript route readiness (no local SilverScript command/tooling available).

Assumptions:
- The shell `PATH` reflects current command lookup state.
- `command -v` accurately reflects executable availability in this environment.

Unverified:
- No SilverScript installation, clone, or package query was performed.
- No SilverScript `--help/--version` run, compile, or live execute evidence.
- No confirmed SilverScript TN12 create/spend/inspect output.

Notes:
- All SilverScript evidence for functional viability remains **UNVERIFIED** until a live command path is demonstrated.
- Next step should be either metadata acquisition (official source/package) followed by an explicit SilverScript build probe, or route pivoting if this is blocked.

## env-007 SilverScript official-source and workspace metadata probe

- **Run ID:** env-007
- **Date/time:** 2026-06-23T14:00:49Z
- **Network:** TN12/testnet (not networked in this check)

Observed (factual):
- URL probe:
  - `curl -I https://github.com/kaspanet/silverscript`
- Metadata fetch:
  - `curl -L https://raw.githubusercontent.com/kaspanet/silverscript/master/Cargo.toml`
  - `curl -L https://raw.githubusercontent.com/kaspanet/silverscript/master/README.md`
- Evidence:
  - The upstream repo URL resolves and is reachable (`kaspanet/silverscript`).
  - `Cargo.toml` shows a Rust `[workspace]` with members `silverscript-lang`, `debugger/session`, and `debugger/cli`.
  - `README.md` says: this repo is a Rust workspace; build target command is `cargo test -p silverscript-lang`.
  - `README.md` also shows a debugger invocation:
    - `cargo run -p cli-debugger -- silverscript-lang/tests/examples/if_statement.sil --function hello --ctor-arg 3 --ctor-arg 10 --arg 1 --arg 2`
  - `README.md` explicitly says outputs are currently TN12-only.
- Local command probe is still negative for installed CLI (from env-006):
  - `command -v silverscript` -> not found
  - `command -v silver` -> not found
  - `command -v ssc` -> not found

Success/failure: **pass** for remote metadata discovery, **blocked** for executable readiness (no local command and no clone/build performed).

Assumptions:
- `curl` outputs are from this host/session and can represent repository availability at the time of write.
- No clone/build side effects occurred.

Unverified:
- `kaspanet/silverscript` is the official/source-of-truth repo for SilverScript (high-confidence inference from URL + namespace, not officially confirmed in project docs).
- `cargo test -p silverscript-lang` and debugger command outputs are **UNVERIFIED** because they were not run.
- No SilverScript CLI install, build, `--help`, or tx-level covenant workflow has been executed locally.

Notes:
- This run closes the gap from env-006 by identifying a probable upstream workspace source and likely first build/test entry points.
- Recommendation (from this run): proceed to a controlled clone/build proof-of-readiness step in an isolated location before any covenant tx work.

## env-008 SilverScript isolated clone/build probe

- **Run ID:** env-008
- **Date/time:** 2026-06-23T14:07:18Z
- **Network:** TN12/testnet (not networked in this check)

Observed (factual):
- Clone location: `external/silverscript`
- Repo URL: `https://github.com/kaspanet/silverscript`
- Commit checked out: `faaa074915edd1e885e4dd552051e348d1854c87`
- Commands run:
  - `git clone https://github.com/kaspanet/silverscript external/silverscript`
  - `cargo test -p silverscript-lang`
  - `cargo run -p cli-debugger -- silverscript-lang/tests/examples/if_statement.sil --function hello --ctor-arg 3 --ctor-arg 10 --arg 1 --arg 2`

Success/failure:
- `cargo test -p silverscript-lang`: **pass**
  - Outcome summary: 13/13 parser tests + 5/5 silverc tests + 1/1 tutorial example + 2/2 rust examples passed.
- `cargo run -p cli-debugger ...`: **pass** (command runs, exit code 0) with expected script-level runtime behavior:
  - Key error emitted by example execution:
  
    `sdb error: script ran, but verification failed`
    `--> 12:13` `require(d == a)`

- Local command probes remain relevant: no preinstalled `silverscript`/`silver`/`ssc` command discovered prior to clone (per env-006 record).

Assumptions:
- `external/silverscript` is the only clone created for this probe.
- No environment/toolchain changes outside this command sequence.

Unverified:
- No create/spend tx flow has been executed.
- No covenant end-to-end transaction artifacts are collected yet.
- No transaction inspection outputs captured.

Notes:
- This probe confirms the official Kaspa Network GitHub repo is cloneable and its Rust workspace builds in this environment.
- It does **not** confirm any full TN12 covenant create/spend pipeline; example contract run demonstrates debugger execution path and script verification failure for this input.

## env-009 SilverScript command inventory and artefact path

- **Run ID:** env-009
- **Date/time:** 2026-06-23T14:21:00Z
- **Network:** TN12/testnet (not networked in this check)

Observed (factual):
- **Repo commit hash checked:** `faaa074915edd1e885e4dd552051e348d1854c87`
- **Commands inspected/executed in `external/silverscript`:**
  - `git rev-parse HEAD`
  - `cargo metadata --no-deps --format-version 1`
  - `cargo run -p silverscript-lang -- --help`
  - `cargo run -p cli-debugger -- --help`
  - `find . -maxdepth 4 -type f | grep -E '(\.sil|README|Cargo.toml|\.md)$'`
  - `rg -n "compile|silverc|covenant|tn12|testnet|script|artifact|output|cli-debugger" -S docs/TUTORIAL.md README.md debugger/cli/README.md`
  - `cargo run -p silverscript-lang -- --stdout silverscript-lang/tests/examples/num2bin.sil`
  - `cargo run -p silverscript-lang -- silverscript-lang/tests/examples/num2bin.sil`
- **Available crates/binaries discovered:**
  - Workspace packages: `silverscript-lang`, `cli-debugger`, `debugger-session`
  - Binaries available from source/help:
    - `silverc` (from `silverscript-lang`)
    - `cli-debugger` (from `debugger/cli`)
- **Available `.sil` examples:**
  - Total examples under `silverscript-lang/tests/examples`: `81`
  - No-constructor examples (compile-ready without `--constructor-args`): `31`
  - Representative files: `announcement.sil`, `num2bin.sil`, `return_basic.sil`, `return_loop.sil`, `simple_covenant.sil`, `simple_multisig.sil`
- **Candidate compile/debug commands identified:**
  - `cargo run -p silverscript-lang -- <example>.sil`
  - `cargo run -p silverscript-lang -- <example>.sil -o <artifact>.json`
  - `cargo run -p silverscript-lang -- --stdout <example>.sil`
  - `cargo run -p cli-debugger -- <path-to-example>.sil -f <function> [--ctor-arg ...] [--arg ...]`
  - equivalent docs-level forms: `silverc <contract>.sil`, `silverc <contract>.sil -o <artifact>.json`, `cli-debugger <contract-path> --run-all`
- **Compiled/script artefact path discovery:**
  - Verified default artifact path is `<SOURCE>.json`.
  - Verified file created from this run:
    - `/root/kaspa-fair-lab/external/silverscript/silverscript-lang/tests/examples/num2bin.json`
  - JSON artifact contains `contract_name`, `compiler_version`, `script`, `ast`, and `abi` fields.

Success/failure:
- `cargo metadata --no-deps`: **pass** (workspace members and package set enumerated)
- `cargo run -p silverscript-lang -- --help`: **pass**
- `cargo run -p cli-debugger -- --help`: **pass**
- `cargo run -p silverscript-lang -- --stdout silverscript-lang/tests/examples/num2bin.sil`: **pass**
- `cargo run -p silverscript-lang -- silverscript-lang/tests/examples/num2bin.sil`: **pass** (artifact file written)

Unverified:
- `silverc` direct binary execution (`silverc ...`) was not invoked independently (only through `cargo run -p silverscript-lang ...`).
- No transaction create/spend/inspect command chain was executed (per current scope and constraints).
- No mainnet tooling/workflow was used.

Notes:
- Minimum compile-to-artifact path is now confirmed in the isolated clone and can be reproduced with no transaction/network work.
- Next step is to target a covenant-relevant `.sil` and capture one minimal compile artifact under a controlled TN12-only workflow before any tx attempt.

## env-010 SilverScript artefact inspection

- **Run ID:** env-010
- **Date/time:** 2026-06-23T14:29:31Z
- **Network:** TN12/testnet (not networked in this check)

Observed (factual):
- **Command run:** `cargo run -p silverscript-lang -- silverscript-lang/tests/examples/simple_covenant.sil`
- **Execution context:** executed via `/root/.cargo/bin/cargo` because this shell did not expose `cargo` on PATH; behavior/output matched the target command.
- **Generated JSON artefact path:**
  - `/root/kaspa-fair-lab/external/silverscript/silverscript-lang/tests/examples/simple_covenant.json`
- **High-level JSON structure (sampled from artifact):**
  - `contract_name`
  - `compiler_version`
  - `script`
  - `ast`
  - `abi`
  - `without_selector`
  - `state_layout`
  - `debug_info`
- **Covenant/script-related fields visible:** yes
  - `ast.functions[0]` is `covenant()` and is marked `entrypoint`
  - `ast.functions[0].body` includes `require(tx.version == 2)`
  - `script` includes the compiled bytecode payload

Success/failure:
- `cargo run -p silverscript-lang -- silverscript-lang/tests/examples/simple_covenant.sil`: **pass**

Unverified:
- TN12 covenant create transaction has not yet been executed.
- TN12 covenant spend transaction has not yet been executed.
- No inspection output from a real covenant tx has been captured.

Notes:
- This is a covenant-relevant, no-constructor example that compiles cleanly to a JSON artifact.
- The artifact appears structurally useful for TN12 planning but is not yet validated through on-chain-like behavior.

## env-011 SilverScript covenant-local workflow discovery

- **Run ID:** env-011
- **Date/time:** 2026-06-23T14:42:29Z
- **Network:** TN12/testnet (not networked in this check)

Observed (factual):
- Repository-level docs and tests were inspected to ground the next minimal live experiment:
  - `external/silverscript/docs/TUTORIAL.md` (compile/internals and transaction introspection)
  - `external/silverscript/docs/DECL.md` (covenant declaration lowering semantics)
  - `external/silverscript/debugger/cli/README.md` (CLI `.test.json` workflow)
  - `external/silverscript/debugger/session/src/test_runner.rs` (`ContractTestCase`, `TestTxScenario`, tx field parsing)
  - `external/silverscript/debugger/cli/tests/cli_tests.rs` (embedded `.test.json` covenant fixtures in test helpers)
  - `external/silverscript/silverscript-lang/src/compiler/debug_value_types.rs` (`ScriptPubKeyP2SHFromRedeemScript`, `OpInputCovenantId`) mapping table
  - `external/silverscript/tests/common.rs` (`covenant_decl_sigscript`, `execute_input_with_covenants` helpers)
  - `external/silverscript/silverscript-lang/src/compiler/covenant_declarations.rs` and `compiler_tests.rs` (covenant declaration call patterns)
- Evidence-supported interpretation:
  - SilverScript provides a documented local verifier path: compile `.sil` -> JSON + ABI, and a CLI debugger that can run function calls and structured `.test.json` suites.
  - CLI test format already supports covenant spend contexts (`tx` object, `active_input_index`, `inputs`, `outputs`, `covenant_id`, `constructor_args`, `state`, `authorizing_input`, `utxo_script_hex`, etc.).
  - Decl/COV lowering evidence indicates compiler paths use covenant-specific helpers (`OpAuth*`/`OpCov*`) plus covenant-id derivation via `OpInputCovenantId` and related context ops.

Success/failure:
- Documentation/test-source inspection: **pass**
- This run produced no live TN12 network transaction or signing operations.

Unverified:
- No `cli-debugger --run-all` invocation was executed against a covenant `.test.json` in this run.
- No signed create/spend flow was run.
- No tx inspection output from `kaspa` node RPC/submission path has been captured.

Notes:
- The next experiment should remain simulation-only to respect spike constraints: build a minimal `.test.json` for `simple_covenant.sil` and run `cargo run -p cli-debugger -- silverscript-lang/tests/examples/simple_covenant.sil --run-all --test-file <path>` for a spend-only transition check.
- Keep `mainnet` usage and any transaction submission out of scope until a local pass path is reproducible.

## env-012 simple_covenant local simulation

- **Run ID:** env-012
- **Date/time:** 2026-06-23T14:50:19Z
- **Network:** TN12/testnet (local simulation-only)

Observed (factual):
- Commands inspected:
  - `external/silverscript/debugger/cli/README.md` (CLI testing flow and `--run-all` behavior)
  - `external/silverscript/debugger/cli/tests/cli_tests.rs` (fixture example helpers and embedded `.test.json` covenant scenarios)
  - `external/silverscript/silverscript-lang/tests/examples/simple_covenant.sil`
  - `external/silverscript/silverscript-lang/tests/examples/simple_covenant.json`
  - local workspace scan for `.test.json` files in `external/silverscript` (excluding `target/`)
- Fixture path check:
  - `simple_covenant.test.json` does not exist in the upstream checkout under `external/silverscript`
  - no `.test.json` fixture was found for `simple_covenant.sil` in this repo snapshot
- Exact command run:
  - `cargo run -p cli-debugger -- silverscript-lang/tests/examples/simple_covenant.sil --run-all`

Pass/fail:
- **FAIL** (expected): CLI returned exit code `1`

Key output summary:
- `/root/.cargo/bin/cargo` found and used (`cargo 1.96.0`)
- `cli-debugger` built in `0.25s` (cached incremental dev build)
- Failure:
  - `failed to read test file 'silverscript-lang/tests/examples/simple_covenant.test.json': No such file or directory (os error 2)`

What this local simulation proves:
- `--run-all` on `simple_covenant.sil` resolves to a companion `.test.json` fixture named `simple_covenant.test.json`.
- Upstream checkout does not currently provide that fixture; without it, run cannot proceed to PASS/FAIL covenant transition checks.
- This verifies local-command wiring without any live Kaspa interaction.

What remains unverified:
- no covenant create/spend execution output (simulation command stops before fixture phase)
- no live TN12 transaction creation, broadcast, or inspect workflow
- no end-to-end no-broadcast transaction construction planning has been executed yet

## env-013 simple_covenant local fixture

- **Run ID:** env-013
- **Date/time:** 2026-06-23T14:54:50Z
- **Network:** TN12/testnet (local simulation-only)

Observed (factual):
- Source fixture and command:
  - `spikes/tn12-minimal-covenant/fixtures/simple_covenant.test.json`
  - `cargo run -p cli-debugger -- silverscript-lang/tests/examples/simple_covenant.sil --run-all --test-file /root/kaspa-fair-lab/spikes/tn12-minimal-covenant/fixtures/simple_covenant.test.json`
  - Contract under test: `external/silverscript/silverscript-lang/tests/examples/simple_covenant.sil`
  - Entrypoint: `covenant`
  - `tx.version`: `2`

Pass/fail:
- **PASS** (exit code `0`)

Observed output (tool-backed):
- `RUN   version_2_pass`
- `PASS  version_2_pass`
- `1 tests: 1 passed, 0 failed`

Minimum `.test.json` structure needed for this contract/check:
- Root:
  - `{"tests": [...]}`
- Test entry (for local run):
  - `name`
  - `function`
  - `expect` (`"pass"` / `"fail"`)
  - `tx`
- `tx` object:
  - `version: 2` (required here to satisfy `require(tx.version == 2)`).
  - `inputs` (non-empty)
  - `outputs` (non-empty)
- Input minimum fields used here:
  - `utxo_value`
- Output minimum fields used here:
  - `value`
- Optional context fields that can be omitted in this minimal local pass case: `lock_time`, `active_input_index`, `covenant_id`, `constructor_args`, `state`, `authorizing_input`, `sig script / utxo fields`.
- Optional fixture defaults used by debugger when omitted:
  - `tx.active_input_index` defaults to `0`
  - `tx.lock_time` defaults to `0`

What remains unverified:
- No live TN12 create/spend/inspect path was run.
- No actual Kaspa transaction submission or broadcast.

Notes:
- This is a strict local simulation validation of contract code path only.
- Next follow-up should keep non-submitting constraints and focus on no-broadcast transaction-construction planning.

## env-014 no-broadcast local execution evidence

- **Run ID:** env-014
- **Date/time:** 2026-06-23T15:03:00Z (per previous run record)
- **Network:** TN12/testnet (local simulation-only)

Observed (factual):
- `cargo test -p silverscript-lang singleton_transition_allows_correct_state_update -- --nocapture`
  - Pass/fail: **PASS** (exit code `0`)
- `cargo test -p silverscript-lang kcc20_can_split_then_merge_tokens_with_two_way_fanout -- --nocapture`
  - Pass/fail: **PASS** (exit code `0`)
- No-broadcast covenant spend-like run via `cli-debugger`:
  - `/tmp/cov_debug_demo.sil`
  - `/tmp/cov_debug_demo.test.json`
  - Command: `cargo run -p cli-debugger -- /tmp/cov_debug_demo.sil --run-all --test-file /tmp/cov_debug_demo.test.json`
  - Observed output:
    - `RUN tn12_demo_transition_ok`
    - `PASS tn12_demo_transition_ok`
    - `1 tests: 1 passed, 0 failed`
- Canonical `simple_covenant` with explicit tx structure:
  - `/tmp/simple_covenant_tx_structured.test.json`
  - Command: `cargo run -p cli-debugger -- silverscript-lang/tests/examples/simple_covenant.sil --run-all --test-file /tmp/simple_covenant_tx_structured.test.json`
  - Observed output:
    - `RUN   version_2_with_tx_context`
    - `PASS  version_2_with_tx_context`
    - `1 tests: 1 passed, 0 failed`

Artifact reproducibility caveat:
- `/tmp/cov_debug_demo.sil`, `/tmp/cov_debug_demo.test.json`, and `/tmp/simple_covenant_tx_structured.test.json` were temporary test inputs and are **not preserved in repo**. They are not reproducible repo artifacts and should not be treated as canonical evidence.

Notes:
- This run still did not execute live TN12 create/spend/inspect transaction construction.
- No actual Kaspa transaction submission or broadcast was performed.
- Recommended follow-up: move these temporary fixture concepts into repo-owned files under `spikes/tn12-minimal-covenant/fixtures/` and convert this into a reproducible local command sequence.

## env-015 reproducible local no-broadcast workflow

- **Run ID:** env-015
- **Date/time:** 2026-06-23T15:15:32Z
- **Network:** TN12/testnet (local simulation-only)

Observed (factual):

- Commands run:
  1. `cd /root/kaspa-fair-lab/external/silverscript && /root/.cargo/bin/cargo run -p cli-debugger -- /root/kaspa-fair-lab/spikes/tn12-minimal-covenant/fixtures/simple_covenant.sil --run-all --test-file /root/kaspa-fair-lab/spikes/tn12-minimal-covenant/fixtures/simple_covenant.test.json`
  2. `cd /root/kaspa-fair-lab/external/silverscript && /root/.cargo/bin/cargo run -p cli-debugger -- /root/kaspa-fair-lab/spikes/tn12-minimal-covenant/fixtures/tn12_demo_transition.sil --run-all --test-file /root/kaspa-fair-lab/spikes/tn12-minimal-covenant/fixtures/tn12_demo_transition.test.json`
  3. `cd /root/kaspa-fair-lab/external/silverscript && /root/.cargo/bin/cargo run -p cli-debugger -- /root/kaspa-fair-lab/spikes/tn12-minimal-covenant/fixtures/simple_covenant.sil --run-all --test-file /root/kaspa-fair-lab/spikes/tn12-minimal-covenant/fixtures/simple_covenant_tx_structured.test.json`

- Raw output excerpts:
  - `version_2_pass` / `PASS`
  - `tn12_demo_transition_ok` / `PASS`
  - `version_2_with_tx_context` / `PASS`
  - Summary lines in all three runs: `1 tests: 1 passed, 0 failed`

Success/fail: **PASS** (all three fixtures executed locally with no-broadcast)

What this proves:

- Reproducible repo-owned fixture artifacts can drive `cli-debugger --run-all` for `simple_covenant` and a covenant state-transition contract without using `/tmp` files.
- `simple_covenant` test fixture at `simple_covenant.test.json` includes explicit `tx.version: 2` and passes with `covenant` entrypoint.
- A minimal local spend-like/state-transition fixture (`tn12_demo_transition.sil` + `.test.json`) is now tracked in-repo and also passes.
- Commands are now deterministic and re-runnable from fixed repo paths.

What remains unverified:

- No live TN12 transaction has been created, spent, or inspected against a node.
- No Kaspa transaction has been submitted or broadcast.
- No mainnet usage.
- Transaction payload fields beyond what the local debugger fixture checks (real tx serialization/signing/submission) remain untested.

Notes:

- Constraints were respected: no roulette, no web app, no dependencies installation, no clone operations, no external source edits, and no broadcast activity.

## env-016 local no-broadcast helper script

- **Run ID:** env-016
- **Date/time:** 2026-06-23T15:23:37Z
- **Network:** TN12/testnet (local simulation-only)
- **Script path:** `./spikes/tn12-minimal-covenant/run_no_broadcast_checks.sh`
- **Commands covered:**
  1. `cargo run -p cli-debugger -- /root/kaspa-fair-lab/spikes/tn12-minimal-covenant/fixtures/simple_covenant.sil --run-all --test-file /root/kaspa-fair-lab/spikes/tn12-minimal-covenant/fixtures/simple_covenant.test.json`
  2. `cargo run -p cli-debugger -- /root/kaspa-fair-lab/spikes/tn12-minimal-covenant/fixtures/tn12_demo_transition.sil --run-all --test-file /root/kaspa-fair-lab/spikes/tn12-minimal-covenant/fixtures/tn12_demo_transition.test.json`
  3. `cargo run -p cli-debugger -- /root/kaspa-fair-lab/spikes/tn12-minimal-covenant/fixtures/simple_covenant.sil --run-all --test-file /root/kaspa-fair-lab/spikes/tn12-minimal-covenant/fixtures/simple_covenant_tx_structured.test.json`
- **Pass/fail result:** PASS (all three checks completed with no-broadcast)
- **Artifacts logged to:** `spikes/tn12-minimal-covenant/artifacts/`
- **Log files:**
  - `simple-covenant-version2.log`
  - `transition-demo.log`
  - `simple-covenant-tx-structured.log`
- **What remains unverified:** no live TN12 create/spend/inspect transaction sequence has been attempted; no transaction has been submitted or broadcast; no mainnet usage.
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
