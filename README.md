# Kaspa Fair Lab

Kaspa Fair Lab is a lightweight research repository for evaluating whether Kaspa/Toccata tooling is ready for a provably fair gaming proof-of-concept.

## Why this project exists

- Assess whether covenant-controlled flows on Kaspa can be used to model verifiable fairness mechanics with minimal setup.
- Test whether TN12/testnet tooling and workflows are reliable enough for a small, explainable experiment before building any product-like layer.
- Keep all early work documentation-first and evidence-first so decisions are traceable.

The repo intentionally starts with exploration, not implementation.

## Immediate technical goal

**Can we create, spend, inspect, and explain a tiny Toccata covenant artefact on TN12/testnet using available tooling?**

## Ground rules for this phase

- Do not build roulette yet.
- Do not create a web app yet.
- Keep this phase lightweight and mostly documentation-driven.
- Do not claim covenant functionality is working until it has been demonstrated and recorded.
- Clearly separate knowns, unknowns, assumptions, and unverified items in every report.

## Current status

- No production-ready implementation has started yet.
- The first commit is documentation and planning led.
- Scripts are utility-only and non-invasive.

## Repository layout

- `docs/` — project framing, roadmap, and feasibility notes.
- `spikes/tn12-minimal-covenant/` — documentation-only spike folder for TN12 covenant experiments.
- `verifier/` — intended place for validation and explanation artifacts.
- `roulette-demo/` — explicit placeholder for the future roulette PoC (not yet built).
- `scripts/check-env.sh` — quick local environment sanity check.

## Quick checks

```bash
./scripts/check-env.sh
```

This script only inspects the environment and does not install anything.
