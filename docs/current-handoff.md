# TN12 Spike Handoff

## Current project goal
Continue TN12 minimal covenant spike route discovery with documentation-first evidence, while staying constrained to:

- no roulette build,
- no web app,
- no own covenant implementation,
- no transaction submission,
- no mainnet usage.

## Current status
- `simple_covenant.sil` compiles successfully to `external/silverscript/silverscript-lang/tests/examples/simple_covenant.json`.
- The artifact contains `covenant()` as entrypoint, `require(tx.version == 2)`, and a compiled script byte array (`"script": [178,82,156,105]`).
- `env-011` identified SilverScript's local debugger/test-fixture workflow.
- `env-012` executed:
  - `/root/.cargo/bin/cargo run -p cli-debugger -- silverscript-lang/tests/examples/simple_covenant.sil --run-all`
- `env-012` failed only because inferred fixture path was missing:
  - `silverscript-lang/tests/examples/simple_covenant.test.json`.
- No TN12 transaction create/spend/inspect has been attempted.
- Working tree currently includes only repo-doc updates:
  - `spikes/tn12-minimal-covenant/findings.md`
  - `spikes/tn12-minimal-covenant/README.md`

## Current evidence anchors
- Source path: `external/silverscript` in-repo clone (no dependency installs, no new repos).
- Confirmed docs/tests inspected:
  - `external/silverscript/debugger/cli/README.md`
  - `external/silverscript/debugger/cli/tests/cli_tests.rs`
  - `external/silverscript/silverscript-lang/tests/examples/simple_covenant.sil`
  - `external/silverscript/silverscript-lang/tests/examples/simple_covenant.json`

## Active constraints (enforced)
- Do not install dependencies.
- Do not clone repositories.
- Do not submit Kaspa transactions.
- Do not use mainnet.
- Do not modify external SilverScript source.
- Do not implement a web app.

## Branch / git context
- Branch: `main` (tracking `origin/main`)
- Local changes since last clean state are documentation-only in this repo.

## Suggested first prompt after /new
- Inspect upstream embedded fixture examples in local SilverScript tests/docs.
- Design a minimal local fixture for `simple_covenant.sil` in our repo (not in `external/silverscript` source).
- Run the local simulation with:
  - `cargo run -p cli-debugger -- silverscript-lang/tests/examples/simple_covenant.sil --run-all --test-file <path-to-local-fixture>`
- Keep work simulation-only (no broadcast / no network tx chain).

## Unverified / next
- No live TN12 create/spend/inspect path is verified yet.
- No local covenant transition PASS/FAIL exists yet for `simple_covenant.sil` due to missing fixture.
