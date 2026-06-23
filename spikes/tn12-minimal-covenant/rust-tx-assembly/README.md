# env-019/env-020 Rust tx assembly scaffold

Goal: local no-broadcast Rust transaction assembly only; no wallet, no faucet, no broadcast.

This spike keeps the experiment constrained to compiling and running a tiny local transaction-like structure with no network submission and no external signing/faucet steps.

Current behavior:

- constructs a minimal local `kaspa-consensus-core::tx::Transaction`,
- prints a deterministic summary (`version`, input/output counts, output value, script/covenant presence, transaction id),
- writes a repo-owned artifact file at:
  - `/root/kaspa-fair-lab/spikes/tn12-minimal-covenant/rust-tx-assembly/artifacts/local-no-broadcast-transaction-summary.txt`
- writes a deterministic local serialization artifact at:
  - `/root/kaspa-fair-lab/spikes/tn12-minimal-covenant/rust-tx-assembly/artifacts/local-no-broadcast-transaction.hex`

The constructed object is unsigned and local-only.

Serialization note:

- the `.hex` artifact is Borsh binary serialization encoded as lowercase hex,
- this is a real deterministic binary artifact from the local `Transaction` type,
- targeted source audit evidence for the pinned `rusty-kaspa` revision:
  - `consensus/core/src/tx.rs` derives `BorshSerialize`/`BorshDeserialize` for `Transaction`,
  - `consensus/core/src/tx/serde_impl.rs` documents version-aware serde for JSON/bincode-style object serialization,
  - `rpc/core/src/model/tx.rs` defines separate `RpcTransaction*` serializers,
  - no explicit consensus/raw wire transaction serialization API was identified from the allowed targeted searches (`consensus_encode`, `Transaction::serialize`, `serialize_to_vec`, `TransactionHex`).
- conclusion for env-022: Borsh output is not confirmed as Kaspa consensus/raw wire serialization in this spike.

## Reproducibility note

`Cargo.toml` now uses a git-pinned `kaspa-consensus-core` dependency for reproducibility:

- `git = "https://github.com/kaspanet/rusty-kaspa"`
- `rev = "42b734f16e2e09078175028ab33158a9f75e91cf"`

The revision was chosen to match the version already locked by `external/silverscript` (`source = "git+https://github.com/kaspanet/rusty-kaspa?branch=tn12#42b734f16e2e09078175028ab33158a9f75e91cf"` in `external/silverscript/Cargo.lock`).

This avoids the previous absolute `/root/.cargo/git/checkouts/...` path and makes the scaffold dependency resolution deterministic for other environments that can fetch dependencies.

`Cargo.lock` is intentionally committed for reproducibility for this executable spike.

## How to rerun

From repo root:

```bash
cd /root/kaspa-fair-lab/spikes/tn12-minimal-covenant/rust-tx-assembly
cargo check
cargo run
```

Expected `cargo run` outputs include:

- `summary_artifact_path=artifacts/local-no-broadcast-transaction-summary.txt`
- `serialization_artifact_path=artifacts/local-no-broadcast-transaction.hex`
- `serialization_type=borsh binary hex (deterministic local artifact; consensus-wire equivalence unverified)`
- `consensus_serialization_conclusion=unresolved: targeted rusty-kaspa source audit found Borsh + serde/RPC object serializers, but no explicit consensus/raw wire transaction serialization API`
- `transaction_version=2`
- `input_count=1`
- `output_count=1`
- `output0_value=1500`
- `output0_script_bytes_present=true`
- `output0_covenant_binding_present=false`

No signing, submission, broadcast, faucet, wallet seed, or live TN12 network access is performed by this spike.
