# TN12 Node VPS Runbook

This note records the working VPS procedure for running a local Kaspa TN12 node for the `kaspa-fair-lab` Toccata feasibility spike.

## Purpose

The local TN12 node is used only for testnet feasibility work. It is needed for read-only RPC checks and later gated testnet-only transaction/covenant experiments.

Do not use this node for mainnet.

## Important constraints

* TN12/testnet only.
* RPC should listen on localhost only.
* Do not create a wallet unless explicitly approved.
* Do not request faucet funds unless explicitly approved.
* Do not sign or broadcast transactions unless explicitly approved.
* Keep evidence in `spikes/tn12-minimal-covenant/artifacts/`.

## Resource requirement

Earlier sync attempts failed because the Linux OOM killer killed `kaspad`.

The working VPS setup used:

* About 8 GiB RAM.
* 8 GiB swap file.
* Swap usage was observed during sync.

Check memory and swap:

```bash
free -h
swapon --show
```

Check for OOM kills:

```bash
dmesg -T | grep -Ei 'killed process|out of memory|oom|kaspad' | tail -30
```

## Required environment variables

```bash
export LIBCLANG_PATH=/usr/lib/llvm-18/lib
export BINDGEN_EXTRA_CLANG_ARGS="-isystem /usr/lib/gcc/x86_64-linux-gnu/13/include -isystem /usr/include"
export PROTOC=/usr/bin/protoc
```

## Working node command

Run from the pinned rusty-kaspa checkout:

```bash
cd /root/.cargo/git/checkouts/rusty-kaspa-410e06d1fde91a92/42b734f

cargo run --release --bin kaspad -- \
  --testnet \
  --netsuffix=12 \
  --disable-upnp \
  --listen=127.0.0.1:16311 \
  --rpclisten=127.0.0.1:16210 \
  --rpclisten-borsh=127.0.0.1:17210 \
  --connect=178.105.76.81:16311 \
  --connect=86.48.24.208:16311 \
  2>&1 | tee /root/kaspa-fair-lab/spikes/tn12-minimal-covenant/artifacts/manual-tn12-sync-connect.log
```

Notes:

* `--testnet --netsuffix=12` is required for TN12.
* `--connect` was more reliable than plain DNS seeding or `--addpeer`.
* RPC is intentionally localhost-only.
* `--disable-upnp` is used because this is a VPS/test setup.

## Check that the node is running

```bash
ps aux | grep '[k]aspad'
ss -ltnp | grep -E '16210|16311|17210'
```

Expected listeners include:

```text
127.0.0.1:16210
127.0.0.1:17210
```

## Confirm sync from logs

```bash
cd /root/kaspa-fair-lab/spikes/tn12-minimal-covenant/artifacts

grep 'IBD: Processed' manual-tn12-sync-connect.log | tail -40
tail -80 manual-tn12-sync-connect.log
```

Successful sync evidence observed:

```text
IBD: Processed 1098965 blocks (100%)
```

After IBD completed, the node continued processing live relayed blocks and headers.

## Confirm sync by read-only RPC

```bash
cd /root/kaspa-fair-lab/spikes/tn12-minimal-covenant/rpc-readonly-suite

cargo run --release -- grpc://127.0.0.1:16210 /root/kaspa-fair-lab/spikes/tn12-minimal-covenant/artifacts/env-046-rpc-readonly-suite
```

Successful observed values:

```text
networkId: testnet-12
serverVersion: 1.1.1-toc.1
hasUtxoIndex: false
isSynced: true
blockCount: 1235733
headerCount: 1235733
virtualDaaScore: 46858621
```

## Current caveat

The node can run on this VPS only if sufficient RAM/swap is available. A better long-term setup would be more RAM, but 8 GiB swap allowed the TN12 sync to complete.
