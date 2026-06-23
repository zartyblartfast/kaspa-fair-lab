# Terminology

## Kaspa

Public blockchain used in this lab.

## TN12

Kaspa testnet identifier used as the current experimental network target.

## Toccata

Kaspa covenant-related feature set used to enforce transaction-level conditions. For this project we treat it as a potential fairness-enforcement mechanism to be verified.

## Covenant artefact

A small on-chain object/structure whose behavior is governed by covenant-like logic.

## Create

Operation that constructs and publishes the minimal covenant artefact with required parameters.

## Spend

Operation that consumes the artefact in a later transaction according to covenant rules.

## Inspect

Reading on-chain data (and any associated metadata/output) to understand what was created and how it was consumed.

## Explain

Produce a concise, reviewer-friendly walkthrough from creation inputs → on-chain output → observed spend behavior.

## Verifier

Any script, checklist, or process used to verify claims and reproduce inspections.

## Receiver / player model

A participant in future game demos whose action triggers a transaction path. Not implemented yet in this phase.

## Fairness evidence

Information that allows independent parties to confirm outcomes without relying on hidden randomness or centralized trust.

## Unknown / assumed / unverified

- **Unknown**: an item we have not observed or tested.
- **Assumed**: a claim accepted for planning, requiring later validation.
- **Unverified**: a claim still to be tested in a concrete spike.
