# TN12 Spike Handoff

## Current project goal
Continue TN12 minimal covenant spike route discovery with documentation-first evidence, while staying constrained to:

- no roulette build,
- no web app,
- no own covenant implementation,
- no transaction submission,
- no mainnet usage.

## Current status
- Working path is `external/silverscript` (isolated clone, uncommitted and git-ignored).
- `env-008` validated: clone/build/probe baseline succeeded for SilverScript.
- `env-009` completed: command inventory + artefact-path discovery.
- `spikes/tn12-minimal-covenant/findings.md` now contains verified command/help/probe evidence including a minimum compile-to-artifact path.
- `spikes/tn12-minimal-covenant/README.md` updated to make compile-to-artifact as the next-step minimum.
- No transaction create/spend/inspect has been claimed or executed in code-path yet.
- `command -v silverscript`, `command -v silver`, `command -v ssc` remain unavailable on PATH (as of last probe), which is handled by using the repository crate build.

## Current evidence anchors
- Clone URL: `https://github.com/kaspanet/silverscript`
- Commit checked: `faaa074915edd1e885e4dd552051e348d1854c87`
- Confirmed command/help probes:
  - `cargo run -p silverscript-lang -- --help`
  - `cargo run -p cli-debugger -- --help`
- Confirmed compile path:
  - `cargo run -p silverscript-lang -- <example>.sil` writes `<example>.json`
  - Verified path: `external/silverscript/silverscript-lang/tests/examples/num2bin.json`
- Example pool:
  - 81 `.sil` files in `silverscript-lang/tests/examples`
  - Includes covenant-related examples such as `simple_covenant.sil`.

## Active constraints (enforced)
- Do not install new dependencies.
- Do not clone additional repositories.
- Do not submit any Kaspa transaction.
- Do not use mainnet.
- Do not implement covenant logic yet.
- Do not claim transaction create/spend/inspect works until observed and documented with outputs.

## Branch / git context
- Branch: `main`
- Last commit: `75d7a12` — "Document SilverScript artefact path discovery"
- Working tree currently dirty with:
  - `spikes/tn12-minimal-covenant/findings.md`
  - `spikes/tn12-minimal-covenant/README.md`

## Suggested next step after /new
Continue from env-009 and run one constrained SilverScript step:
- compile one upstream `.sil` example to JSON with
  - `cargo run -p silverscript-lang -- <example>.sil`
- record outputs in `findings.md`, then proceed only to command discovery/inspection prep.

## Unverified / must-do next
- No verified TN12 create/spend/inspect command chain yet.
- No live transaction inspection output yet.
- Artifact provenance for a covenant-relevant `.sil` in TN12 context is still pending.
