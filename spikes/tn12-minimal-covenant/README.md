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
- `scripts/check-env.sh` currently reports `cargo` and `rustc` as not found in this shell PATH.
- TODO verification/install steps (not executed in this task):
  - confirm runtime PATH includes the Rust install location (for example `~/.cargo/bin`)
  - run `rustc --version` and `cargo --version`

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

1. **Normalize environment for local Rust tooling**
   - Re-check PATH and ensure `cargo` and `rustc` are discoverable in the active shell session.
   - Record exact versions to use in all subsequent run logs.

2. **Probe SilverScript route (if available)**
   - Detect tool availability (`silverscript*` command names if present).
   - Record `--help` and version output only.
   - Check for TN12/sandbox transaction example commands in docs/help output.

3. **Probe Rusty Kaspa / Rust crates route (if available)**
   - Detect whether Rust crates for Kaspa/Toccata are available locally.
   - Record command/library API entry points and required inputs from documentation/help output only.
   - Prefer minimal examples and the smallest compile target surface.

4. **Probe WASM SDK route**
   - Identify whether a WASM-based helper path exists for covenant payload building.
   - Capture any minimal invocation signatures and required parameters.

5. **Probe Python SDK route (if relevant)**
   - Check for any local Python Kaspa/Toccata client module availability.
   - Prefer read-only introspection (`python` import/version/help) before any coding.

6. **Select first live-candidate path**
   - Compare candidate routes by command footprint and inspection-output clarity.
   - Choose the narrowest route for the first live experiment.

No path is treated as valid until a live command sequence is recorded in `findings.md` with outputs.