# Project Brief

## 1) Why this project exists

This lab exists to answer a narrow readiness question before engineering time is spent on a gaming product:

> Can the Kaspa ecosystem (especially Toccata covenant functionality) currently support a tiny, independently inspectable fairness artifact that can be created and consumed on TN12/testnet with publicly understandable evidence?

The research is intentionally constrained to **small, auditable** tasks. The objective is to reduce blind implementation risk by validating fundamental capabilities first.

## 2) Why roulette is not being built yet

Roulette introduces multiple unresolved engineering dimensions (UI, state orchestration, UX, bankroll handling, odds engine, payouts, anti-fraud policy). Building it now would:

- Couple us to assumptions we have not yet validated on-chain.
- Hide failures in covenant logic behind application layers.
- Create a false impression of progress even if core primitives are not working.

For this phase, we will only validate primitives and evidence generation. Roulette stays explicitly out of scope.

## 3) What a real Kaspa/Toccata "wow" demo would be

A meaningful demo should demonstrate all of the following:

1. **On-chain covenant-like control** is actually enforced.
2. **End-to-end artifact lifecycle** is clear (create → spend → inspect → explain).
3. **Publicly verifiable fairness artifact** is small and reproducible.
4. **Observer verification** is feasible by someone not involved in generation.
5. **Failure modes and edge cases** are documented (not just the happy path).

If those are proven on TN12 with reproducible steps, a constrained non-custodial roulette proof-of-concept can be considered.

## 4) What must be proven before a roulette PoC

Before any roulette prototype, we need evidence for:

- The minimum command/tooling path to create and spend a covenant artefact.
- A deterministic inspection process that can independently recover meaningful fields.
- A clear explanation method that is understandable to a technical reviewer.
- Repeatable behavior on TN12 (or clear reasoned alternatives if TN12 tooling blocks this).
- Security properties and trust boundaries (where trust is still required, and where not).
- Operational readiness for tooling (installation, versions, reproducibility, failures).

## 5) What the TN12 minimal covenant spike is testing

The first spike tests the minimum covenant path only:

- Is creation possible with available tooling?
- Can we submit and spend the artefact?
- Can inspection commands produce outputs that can be explained from source inputs?
- Can results be recorded with explicit assumptions and unknowns?

The spike is intentionally narrow and does not include any payout logic, bankroll model, or web interface.

## 6) Known / unknown / assumed

### Known

- A research branch exists and is expected to be tool-and-version sensitive.
- TN12/testnet constraints are not identical to mainnet behavior.
- We are in exploratory phase and no claims are verified yet.

### Unknown (to be resolved by spike)

- Exact command chain for robust create/spend/inspect behavior.
- Whether current CLI tooling supports every required step in a reproducible way.
- The depth of manual inspection needed for confidence.
- Whether artefact formats and metadata are stable across tooling versions.

### Assumed (until disproven)

- Public key concepts and transaction signing are understood by the team.
- Public networks/tools can be accessed with minimal setup.
- Results can be captured in a way that survives reviewer scrutiny.
- Early docs and findings can be kept lightweight and still decisive.
