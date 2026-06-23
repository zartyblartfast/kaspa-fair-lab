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
- Repository-owned fixture set now includes:
  - `spikes/tn12-minimal-covenant/fixtures/simple_covenant.sil`
  - `spikes/tn12-minimal-covenant/fixtures/simple_covenant.test.json`
  - `spikes/tn12-minimal-covenant/fixtures/simple_covenant_tx_structured.test.json`
  - `spikes/tn12-minimal-covenant/fixtures/tn12_demo_transition.sil`
  - `spikes/tn12-minimal-covenant/fixtures/tn12_demo_transition.test.json`
- Canonical fixture inputs and checks:
  - `simple_covenant.test.json`: `function: "covenant"`, `expect: "pass"`, explicit `tx.version: 2`
  - `tn12_demo_transition.test.json`: transition-style `rebalance` scenario, `expect: "pass"`
  - `simple_covenant_tx_structured.test.json`: structured tx context variant for `simple_covenant`
- Local simulation commands now pass with repo-owned artifacts:
  - `cd /root/kaspa-fair-lab/external/silverscript && /root/.cargo/bin/cargo run -p cli-debugger -- /root/kaspa-fair-lab/spikes/tn12-minimal-covenant/fixtures/simple_covenant.sil --run-all --test-file /root/kaspa-fair-lab/spikes/tn12-minimal-covenant/fixtures/simple_covenant.test.json`
  - `cd /root/kaspa-fair-lab/external/silverscript && /root/.cargo/bin/cargo run -p cli-debugger -- /root/kaspa-fair-lab/spikes/tn12-minimal-covenant/fixtures/tn12_demo_transition.sil --run-all --test-file /root/kaspa-fair-lab/spikes/tn12-minimal-covenant/fixtures/tn12_demo_transition.test.json`
  - `cd /root/kaspa-fair-lab/external/silverscript && /root/.cargo/bin/cargo run -p cli-debugger -- /root/kaspa-fair-lab/spikes/tn12-minimal-covenant/fixtures/simple_covenant.sil --run-all --test-file /root/kaspa-fair-lab/spikes/tn12-minimal-covenant/fixtures/simple_covenant_tx_structured.test.json`
- Observed outputs in all three runs: PASS (`version_2_pass`, `tn12_demo_transition_ok`, `version_2_with_tx_context`), each with `1 tests: 1 passed, 0 failed`.
- No TN12 transaction has been submitted, spent, or broadcast; no mainnet activity.
- Working tree includes repo-doc updates plus fixture assets for the local workflow:
  - `spikes/tn12-minimal-covenant/README.md`
  - `spikes/tn12-minimal-covenant/findings.md`
  - `spikes/tn12-minimal-covenant/fixtures/` (repo-owned fixture assets)

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
