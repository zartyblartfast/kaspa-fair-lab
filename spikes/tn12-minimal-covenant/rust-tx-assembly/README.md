# env-019 Rust tx assembly scaffold

Goal: local no-broadcast Rust transaction assembly only; no wallet, no faucet, no broadcast.

This spike keeps the experiment constrained to compiling a tiny local transaction-like structure with no network submission and no external signing/faucet steps.

Next step: prove a minimal signed payload artifact path in a follow-up after this scaffold compiles.

## Reproducibility note

`Cargo.toml` currently depends on `kaspa-consensus-core` using an absolute cargo-checkout path:

- `path = "/root/.cargo/git/checkouts/rusty-kaspa-410e06d1fde91a92/42b734f/consensus/core"`

This is not reproducible across environments. A repo-local alternative is not obvious from this spike alone because there is no `rusty-kaspa` checkout checked into this repository tree. Until a local clone of `rusty-kaspa` is intentionally introduced (e.g. via a tracked subdir), the safest documented state is to treat this as a known portability gap and verify it explicitly before rebase/share.
