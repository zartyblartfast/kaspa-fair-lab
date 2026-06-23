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

- Status: env-009 command inventory and artefact-path discovery executed.
- Target network: TN12/testnet.
- Next step: compile an existing upstream `.sil` example into a JSON artefact (no tx submit/network action yet), then continue minimal command discovery from that verified artifact.

## How results are recorded

Update `findings.md` with:

- date/time,
- exact commands,
- outputs,
- success/failure,
- knowns,
- unknowns,
- and assumptions introduced.

## Next-step technical plan

Goal for the next run: based on env-009 command discovery, convert one upstream `.sil` into an inspectable compiled artifact and log the exact minimum command sequence.

1. **Next approved experiment (preliminary, non-transactional):**
   - Keep the clone in `external/silverscript`.
   - Use `cargo run -p silverscript-lang -- <example>.sil` as the minimum command sequence to emit `<example>.json`.
   - Prefer a no-ctor example (`contract ...()`), e.g. `silverscript-lang/tests/examples/num2bin.sil`, to avoid constructor-arg setup.
   - Capture the produced artifact path and sample fields (`contract_name`, `script`, `abi`).

2. **Failure handling for reproducibility:**
   - If a future compile command is blocked, run one retry in a clean shell context before changing route.

3. **Fallback plan (after a build failure only):**
   - If SilverScript remains blocked after a targeted retry, fall back to Rusty Kaspa / Rust crates for the first full create/spend path.

No path is treated as valid until a live command sequence is recorded in `findings.md` with outputs.