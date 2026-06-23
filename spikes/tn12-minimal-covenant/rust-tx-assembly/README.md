# env-019 Rust tx assembly scaffold

Goal: local no-broadcast Rust transaction assembly only; no wallet, no faucet, no broadcast.

This spike keeps the experiment constrained to compiling a tiny local transaction-like structure with no network submission and no external signing/faucet steps.

Next step: prove a minimal signed payload artifact path in a follow-up after this scaffold compiles.

## Reproducibility note

`Cargo.toml` now uses a git-pinned `kaspa-consensus-core` dependency for reproducibility:

- `git = "https://github.com/kaspanet/rusty-kaspa"`
- `rev = "42b734f16e2e09078175028ab33158a9f75e91cf"`

The revision was chosen to match the version already locked by `external/silverscript` (`source = "git+https://github.com/kaspanet/rusty-kaspa?branch=tn12#42b734f16e2e09078175028ab33158a9f75e91cf"` in `external/silverscript/Cargo.lock`).

This avoids the previous absolute `/root/.cargo/git/checkouts/...` path and makes the scaffold dependency resolution deterministic for other environments that can fetch dependencies.

`Cargo.lock` is intentionally committed for reproducibility for this executable spike.
