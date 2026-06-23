# TN12 Minimal Covenant Spike

## Intent

Validate the absolute minimum covenant flow needed to support a future provably fair demo.

## Why this spike exists

Before implementing roulette, we need confidence that base primitives actually work:

- create a tiny covenant artefact,
- spend it,
- inspect it,
- and explain the observed behavior.

## Required local tooling

- Planned local tools: `git`, `node`, `npm`, `python3`, `cargo`, `rustc`, `codex`
- `scripts/check-env.sh` now reports `git`, `node`, `npm`, `python3`, `cargo`, `rustc`, and `codex` as present in this session (verify per session).
- Rust PATH visibility should be re-checked at the start of each new shell session.

## Non-goals

- No roulette mechanics.
- No web app.
- No gameplay economics or bankroll logic.
- No production deployment.

## Acceptance criteria

1. Documented command or API sequence for create/spend.
2. Captured raw outputs (hashes, txids, payloads where available).
3. Verification notes showing what was observed, what was assumed, and what remains unverified.

## Current status

- Status: planned, not yet executed.
- Target network: TN12/testnet.
- Next step: run the first full command sequence and record results in `findings.md`.

## How results are recorded

Update `findings.md` with:

- date/time,
- exact commands,
- outputs,
- success/failure,
- knowns,
- unknowns,
- and assumptions introduced.

## Next-step technical plan (minimum-route discovery only)

Goal for the next run: identify the smallest reproducible path to create/inspect a tiny artefact, without assuming any path works yet.

1. **Proposed first experiment (planned, do not run yet)**
   - Conduct a read-only route probe for command availability and docs/help output:
     - `silverscript*` discovery and `--help` output (if present)
     - Rust crate/tool availability references in docs
   - Do **not** build, compile, or submit any transaction in this experiment.
   - If SilverScript yields actionable TN12 create/spend guidance, use it as the first live experiment.

2. **Fallback decision rule (if SilverScript is not actionable)**
   - Use Rusty Kaspa / Rust crates as the next candidate for the first live experiment because it is the most direct lower-level route for native tx and covenant payload control.

3. **Secondary probes (still read-only/documentation-only before live execution)**
   - Check whether WASM SDK route is available via docs/examples.
   - Check whether a Python SDK route is documented and importable for minimal orchestration.

4. **Select first live route**
   - Choose the single narrowest viable path, then execute one minimal command sequence only.
   - No implementation path is considered working until tx output/artifact is recorded in `findings.md`.

No path is treated as valid until a live command sequence is recorded in `findings.md` with outputs.