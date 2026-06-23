# TN12 Spike Handoff

## Current project goal
Continue TN12 minimal covenant spike route discovery with documentation-first evidence, while staying constrained to:

- no roulette build,
- no web app,
- no own covenant implementation,
- no transaction submission,
- no mainnet usage.

## Current status
- `simple_covenant.sil` compiles successfully to JSON: `external/silverscript/silverscript-lang/tests/examples/simple_covenant.json`.
- `simple_covenant.test` fixture was added locally at:
  - `spikes/tn12-minimal-covenant/fixtures/simple_covenant.test.json`
- Fixture used in local simulation:
  - `function: "covenant"`
  - `expect: "pass"`
  - `tx.version: 2`
  - minimal input/output values (`utxo_value` / `value`)
- Local simulation command passed:
  - `/root/.cargo/bin/cargo run -p cli-debugger -- silverscript-lang/tests/examples/simple_covenant.sil --run-all --test-file /root/kaspa-fair-lab/spikes/tn12-minimal-covenant/fixtures/simple_covenant.test.json`
- Observed output lines:
  - `RUN version_2_pass`
  - `PASS version_2_pass`
  - `1 tests: 1 passed, 0 failed`
- No TN12 transaction broadcast has been attempted.
- TN12-style no-broadcast spend-like covenant execution has been run locally via `cli-debugger` with an explicit stateful `covenant` fixture:
  - `/root/.cargo/bin/cargo run -p cli-debugger -- /tmp/cov_debug_demo.sil --run-all --test-file /tmp/cov_debug_demo.test.json`
  - observed: `RUN tn12_demo_transition_ok` / `PASS tn12_demo_transition_ok` / `1 tests: 1 passed, 0 failed`
- `/tmp/cov_debug_demo.sil`, `/tmp/cov_debug_demo.test.json`, and `/tmp/simple_covenant_tx_structured.test.json` were temporary and are **not repo-persistent artifacts**.
- No transaction has been submitted or broadcast.
- Working tree currently includes only repo-doc updates:
  - `spikes/tn12-minimal-covenant/findings.md`
  - `spikes/tn12-minimal-covenant/README.md`
  - `spikes/tn12-minimal-covenant/fixtures/simple_covenant.test.json`

## Current evidence anchors
- Source path: `external/silverscript` in-repo clone (no dependency installs, no new repos).
- Confirmed docs/tests inspected:
  - `external/silverscript/debugger/cli/README.md`
  - `external/silverscript/debugger/cli/tests/cli_tests.rs`
  - `external/silverscript/silverscript-lang/tests/examples/simple_covenant.sil`
  - `external/silverscript/silverscript-lang/tests/examples/simple_covenant.json`

## Active constraints (enforced)
- Do not build roulette.
- Do not create a web app.
- Do not submit any Kaspa transaction.
- Do not use mainnet.
- Do not install dependencies.
- Do not clone new repositories.
- Do not modify external SilverScript source.

## Branch / git context
- Branch: `main` (tracking `origin/main`)
- Local changes since last clean state are documentation-only and fixture file updates in this repo.

## Suggested first prompt after /new
- Plan a no-broadcast TN12 transaction-construction path for create/spend/inspect using local artifacts only.
- Identify which SDK/crate/tool currently supports constructing a TN12 transaction from the compiled SilverScript artifact (`simple_covenant.json` + `.test.json` context).
- Do not submit or broadcast any transaction until the construction path is understood and documented.

## Unverified / next
- Canonical `simple_covenant` path is now exercised with tx-structured test input via `cli-debugger` and verified as pass.
- Remaining next step: turn this into a documented TN12 transaction-construction workflow (create/spend/inspect sequence) that is directly reusable by the spike.
- No transaction has been submitted or broadcast.
