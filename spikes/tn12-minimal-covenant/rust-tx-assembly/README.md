# env-019/env-020 Rust tx assembly scaffold

Goal: local no-broadcast Rust transaction assembly only; no wallet, no faucet, no broadcast.

This spike keeps the experiment constrained to compiling and running a tiny local transaction-like structure with no network submission and no external signing/faucet steps.

Current behavior:

- constructs a minimal local `kaspa-consensus-core::tx::Transaction`,
- converts that local `Transaction` into `kaspa_rpc_core::RpcTransaction` via the official `From<&Transaction> for RpcTransaction` path,
- constructs a local `kaspa_rpc_core::SubmitTransactionRequest` around that `RpcTransaction` with `allow_orphan = false`,
- serializes both `RpcTransaction` and `SubmitTransactionRequest` through the Rusty Kaspa RPC `Serializer` trait path and writes lowercase hex artifacts,
- deserializes those RPC serializer artifacts back into local objects and verifies a local round-trip on key fields,
- prints a deterministic summary (`version`, input/output counts, output value, script/covenant presence, transaction id),
- writes a repo-owned artifact file at:
  - `/root/kaspa-fair-lab/spikes/tn12-minimal-covenant/rust-tx-assembly/artifacts/local-no-broadcast-transaction-summary.txt`
- writes a deterministic local serialization artifact at:
  - `/root/kaspa-fair-lab/spikes/tn12-minimal-covenant/rust-tx-assembly/artifacts/local-no-broadcast-transaction.hex`
- writes a repo-owned local RPC transaction artifact at:
  - `/root/kaspa-fair-lab/spikes/tn12-minimal-covenant/rust-tx-assembly/artifacts/local-no-broadcast-rpc-transaction-summary.txt`
- writes a repo-owned local submit-request artifact at:
  - `/root/kaspa-fair-lab/spikes/tn12-minimal-covenant/rust-tx-assembly/artifacts/local-no-broadcast-submit-transaction-request-summary.txt`
- writes repo-owned RPC serializer artifacts at:
  - `/root/kaspa-fair-lab/spikes/tn12-minimal-covenant/rust-tx-assembly/artifacts/local-no-broadcast-rpc-transaction-rpc-serializer.hex`
  - `/root/kaspa-fair-lab/spikes/tn12-minimal-covenant/rust-tx-assembly/artifacts/local-no-broadcast-submit-transaction-request-rpc-serializer.hex`
- writes a repo-owned local round-trip verification artifact at:
  - `/root/kaspa-fair-lab/spikes/tn12-minimal-covenant/rust-tx-assembly/artifacts/local-no-broadcast-rpc-roundtrip-summary.txt`

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

env-023 note:

- Source-only inspection indicates submission boundary is `RpcTransaction`, not raw transaction hex.
- env-024 emits a local no-broadcast `RpcTransaction` summary artifact via the official `From<&Transaction> for RpcTransaction` conversion path.
- env-025 emits a local no-broadcast `SubmitTransactionRequest` summary artifact without calling any RPC client.

## Reproducibility note

`Cargo.toml` now uses git-pinned `kaspa-consensus-core` and `kaspa-rpc-core` dependencies plus direct `workflow-serializer` access for reproducibility:

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
- `rpc_summary_artifact_path=artifacts/local-no-broadcast-rpc-transaction-summary.txt`
- `submit_request_summary_artifact_path=artifacts/local-no-broadcast-submit-transaction-request-summary.txt`
- `rpc_serializer_artifact_path=artifacts/local-no-broadcast-rpc-transaction-rpc-serializer.hex`
- `submit_request_serializer_artifact_path=artifacts/local-no-broadcast-submit-transaction-request-rpc-serializer.hex`
- `rpc_roundtrip_summary_artifact_path=artifacts/local-no-broadcast-rpc-roundtrip-summary.txt`
- `serialization_type=borsh binary hex (deterministic local artifact; consensus-wire equivalence unverified)`
- `rpc_serializer_type=rusty-kaspa rpc Serializer trait binary encoded as lowercase hex`
- `transaction_version=2`
- `rpc_serializer_bytes_len=171`
- `submit_request_serializer_bytes_len=178`
- `rpc_transaction_roundtrip=pass`
- `submit_request_roundtrip=pass`
- `no_rpc_client_called=true`
- `signed=false`
- `broadcast=false`

No signing, submission, broadcast, faucet, wallet seed, or live TN12 network access is performed by this spike.
