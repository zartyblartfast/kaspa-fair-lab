# ENV-048 key/address preflight

Status: documentation/preflight only

Purpose of ENV-048:
- define the safest future route for TN12-only Gate 1 key/address generation
- preserve a documentation-only decision record before any secret-bearing action is approved
- keep the next step constrained, reviewable, and explicitly reversible

This document does not authorize execution.

Hard scope limits for this preflight pass:
- do not create a wallet
- do not generate private keys
- do not generate an address
- do not request faucet funds
- do not inspect live UTXOs
- do not sign transactions
- do not broadcast transactions
- do not use mainnet
- do not build roulette
- do not create a web app

## 1. Purpose

ENV-048 exists to define the safest future route for TN12-only Gate 1 key/address generation before any command with wallet or key side effects is allowed to run.

The immediate goal is not to generate anything yet. The goal is to choose and document the safest later execution path, storage policy, evidence format, and stop conditions.

## 2. Current status

- ENV-046: complete; synced TN12 node and read-only RPC evidence preserved under `spikes/tn12-minimal-covenant/artifacts/env-046-rpc-readonly-suite/`.
- ENV-047: planning-only gate document complete at `spikes/tn12-minimal-covenant/env-047-gated-plan.md`.
- ENV-047 still records that the key/address generation route is not yet fixed.
- ENV-048: preflight/tooling discovery only.
- Relevant Rusty-Kaspa CLI wallet/address command surfaces were identified in prior read-only discovery:
  - `cli/src/modules/address.rs`
  - `cli/src/modules/wallet.rs`
  - `cli/src/wizards/wallet.rs`
- Prior read-only discovery also noted that CLI wallet creation appears interactive and may write local wallet files under default Kaspa wallet storage such as `~/.kaspa`.
- Signing helper code existence has been observed, but no signing helper may be executed under ENV-048.
- No wallet, key, address, faucet, UTXO inspection, signing, broadcast, or mainnet action has been executed in ENV-048.
- No repository edits were made during the earlier discovery pass; the edits in this step are documentation-only.

## 3. Candidate routes

### A. Kaspa CLI wallet/address path

Description:
- Use existing Rusty-Kaspa CLI wallet/address flows to create or access wallet state and derive a receive address for TN12.

Pros:
- uses existing Kaspa tooling
- likely closest to normal wallet workflow
- may produce valid TN12 receive addresses

Cons / risks:
- interactive
- may write wallet/private key material under default local wallet storage
- needs careful control of network selection
- needs explicit storage location policy before execution
- must avoid accidental mainnet configuration
- default-wallet behavior may be harder to constrain and review than a tiny purpose-built helper

Preflight view:
- viable as a fallback route
- not currently preferred because it appears more side-effect-prone and less storage-explicit than a tiny isolated helper

### B. Tiny isolated Rust helper path

Description:
- Build a very small repo-owned Rust helper dedicated to one approved TN12 test-only key/address generation action.

Pros:
- can be made small, reviewable, TN12-only, and non-interactive
- can write controlled evidence artifacts
- can avoid touching default wallet storage
- easier to wrap in explicit safety checks

Cons / risks:
- must be carefully checked against Rusty-Kaspa APIs
- could accidentally create misleading or non-wallet-compatible output if implemented incorrectly
- still involves key/address generation if later executed, so requires explicit approval
- must be reviewed carefully to ensure it cannot silently drift into node contact, mainnet selection, or unsafe output

Preflight view:
- strongest current candidate for a future Gate 1 execution path, if implemented conservatively and reviewed before use

## 4. Recommended future route

Current recommendation:
- prefer a tiny isolated Rust helper for Gate 1, with the Kaspa CLI path retained only as a fallback if the helper route proves unsuitable

This recommendation holds only if the helper is designed to satisfy all of the following:
- TN12-only by construction
- non-interactive
- does not contact the node
- does not write to `~/.kaspa`
- stores any private key material only in an explicitly ignored local secrets path, never in committed artifacts
- emits a public evidence summary only, such as network, address, derivation/test label, and safety checks
- is reviewed before execution

Reason for the recommendation:
- the main risk in Gate 1 is not whether key/address generation is theoretically possible, but whether it can be executed in a tightly bounded way without accidental wallet persistence, ambiguous network selection, or secret leakage into repo artifacts
- a tiny isolated helper is easier to audit, easier to bind to TN12-only behavior, and easier to couple to a strict public/private artifact split

Fallback rule:
- the CLI wallet/address route remains available only if the isolated helper path is later shown to be unsuitable or incorrect, and only after an equally explicit storage/location/network plan is documented and approved

## 5. Storage policy

Future Gate 1 storage policy must be strict:

Public artifact location:
- public artifacts may go under `spikes/tn12-minimal-covenant/artifacts/env-049-key-address/`

Private material restrictions:
- private key/wallet material must not be committed
- private key/wallet material must not be written into normal evidence artifacts
- private key/wallet material must not be printed into reviewable public summaries intended for commit

Local secrets path:
- if local secrets are ever needed, use a clearly named ignored path such as `spikes/tn12-minimal-covenant/local-secrets/`
- any future execution must treat this path as local-only and non-artifact storage

Git protection requirement:
- add or verify `.gitignore` coverage before any future key generation
- do not proceed if `.gitignore` does not protect local secret paths
- current preflight note: repository `.gitignore` has been inspected during ENV-048 and does not yet explicitly list `spikes/tn12-minimal-covenant/local-secrets/`; this must be corrected or otherwise verified before ENV-049 execution

Default-wallet isolation requirement:
- do not proceed if the selected tool would write secrets to default wallet storage unless that behavior is explicitly redirected, understood, approved, and kept outside committed artifacts
- avoid `~/.kaspa` writes where possible

## 6. Future ENV-049 approval gate

If ENV-049 is approved later, it should be limited to exactly this scope:
- generate one TN12 test-only key/address
- no faucet
- no UTXO inspection
- no signing
- no broadcast
- no covenant transaction
- no roulette/web app

ENV-049 should remain a single-gate execution step, not a bundled progression into later lifecycle work.

## 7. Expected future evidence artifact

For later ENV-049 only, the expected public artifact may be:
- `spikes/tn12-minimal-covenant/artifacts/env-049-key-address/env-049-summary.txt`

That public summary should contain only safe/public fields, for example:
- Result: PASS/FAIL
- Network: testnet-12
- Address prefix/type
- Tool path used
- Whether node contact was required
- Confirmation that no faucet/signing/broadcast occurred
- Confirmation that no private key material was committed

It should not include:
- private key material
- seed phrases
- encrypted wallet blobs
- raw secret-bearing debug output

## 8. Stop conditions

Stop immediately if any of the following occurs or cannot be ruled out:
- any command tries to use mainnet
- any tool wants to write to default wallet storage unexpectedly
- any command requests faucet funds
- any command signs or broadcasts
- any output includes private key material in a public artifact
- `.gitignore` does not protect local secret paths
- network selection is ambiguous
- address prefix/network cannot be verified as TN12/testnet
- the chosen route requires node contact even though Gate 1 should not need it
- the tool path cannot be reviewed clearly enough before execution

## Planning conclusion

ENV-048 is documentation/preflight only.

The current safest recommendation for a future Gate 1 approval is a tiny isolated Rust helper, provided it is TN12-only, non-interactive, storage-controlled, reviewable, and incapable of writing secrets into committed artifacts by default.

No wallet creation, no key generation, no address generation, no faucet use, no live UTXO inspection, no signing, no broadcast, no covenant execution, no mainnet use, no roulette work, and no web app work were performed while producing this document.
