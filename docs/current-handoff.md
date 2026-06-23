# TN12 Spike Handoff

## Current project goal
Continue TN12 minimal covenant spike route discovery with documentation-first evidence, while staying constrained to:

- no roulette build,
- no web app,
- no own covenant implementation,
- no transaction submission,
- no mainnet usage.

## Concise status update

1) env-016 added the canonical helper script: `spikes/tn12-minimal-covenant/run_no_broadcast_checks.sh`.

2) env-017 re-ran the helper script successfully:
- command: `cd /root/kaspa-fair-lab && ./spikes/tn12-minimal-covenant/run_no_broadcast_checks.sh`
- result: PASS
- output included: `PASS version_2_pass`, `PASS tn12_demo_transition_ok`, `PASS version_2_with_tx_context`, `[PASS] All local no-broadcast checks passed.`

3) Repo-owned log paths:
- `spikes/tn12-minimal-covenant/artifacts/simple-covenant-version2.log`
- `spikes/tn12-minimal-covenant/artifacts/transition-demo.log`
- `spikes/tn12-minimal-covenant/artifacts/simple-covenant-tx-structured.log`
- these are generated artifacts and are not modified in this run; continue using them as the canonical local trail.

4) Current evidence proves:
- SilverScript builds locally.
- repo-owned local fixtures reproduce no-broadcast SilverScript checks.
- local cli-debugger transaction-like simulation is working.

5) Current evidence does not prove:
- live TN12 create/spend/inspect,
- real transaction serialization/signing,
- wallet/faucet usage,
- broadcast,
- mainnet behaviour.

6) Recommended next task after `/new`:
- plan a Rust-based local transaction-assembly spike using mock/test keys only;
- no real wallet seed,
- no faucet funds,
- no network broadcast,
- inspect local Rusty Kaspa / SilverScript dependencies for `Transaction::new`, `PopulatedTransaction`, covenant outputs, serialization, and signing APIs.

## Branch / repo status

- Repo: `/root/kaspa-fair-lab`
- Branch: `main` (`origin/main`)
- Modified files:
  - `docs/current-handoff.md`
  - `spikes/tn12-minimal-covenant/README.md`
  - `spikes/tn12-minimal-covenant/findings.md`

## Suggested first prompt after /new

`Plan env-019 as a no-broadcast Rust tx-assembly artifact step: generate a signed local tx payload from `spikes/tn12-minimal-covenant/fixtures`, save deterministic artifacts under repo-owned paths, and update `findings.md` with proof-only evidence.`
