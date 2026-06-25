# TN10 Official Rusty Kaspa Wallet Workflow

Purpose:
Use the official Rusty Kaspa tn10-toc3 wallet tooling safely for TN10/Toccata testnet work in kaspa-fair-lab.

Tool path:

/root/kaspa-fair-lab/tools/rusty-kaspa-releases/tn10-toc3/bin/kaspa-wallet

Known version:

Kaspa Cli Wallet v1.2.1-toc.3

Standard TN10 connection sequence inside kaspa-wallet:

network testnet-10
server public
connect

Read-only verification commands:

rpc get-server-info
rpc get-current-network
rpc get-sync-status
rpc get-block-dag-info

Important:
get-current-network only proves Testnet class. Use get-server-info or get-block-dag-info to confirm suffix 10.

Expected good server properties:

network_id: Testnet suffix 10
is_synced: true
has_utxo_index: true

Project wallet:

env052-tn10-test-only

Funded source address:

kaspatest:qrhszwr4r2ejukpxyjp7jvn40tth5s8zy0538zvkkrvtkxvvyhlmjhe275slx

TN10 faucet used successfully:

https://faucet-tn10.kaspanet.io/

Faucet txid:

29d76273819d519bea146e881554c633bac4d30989bfc8e1862fed965d8f5116

Ordinary send command pattern:

estimate <amount> <priority fee>
send <address> <amount> <priority fee>

Proven ordinary send:

amount: 1 TKAS
priority fee: 0
txid: c9b4532f217d66987997e972963ec5dbfa5a9e7bf18f3e38910763274fb05135

Recipient used in ENV-056:

kaspatest:qpf0pc97d4vxtd99gppqtzhrjtna4t396lvu2t249p9f0rkh05pxxc5mj9yf2

Observed post-send state:

wallet/account total: 999.997964 TKAS
recipient address balance: 1.0 TKAS
recipient UTXO: c9b4532f217d66987997e972963ec5dbfa5a9e7bf18f3e38910763274fb05135:0
change UTXO: c9b4532f217d66987997e972963ec5dbfa5a9e7bf18f3e38910763274fb05135:1

Safety rules:

- Never use mainnet.
- Never expose mnemonic, seed, private key, wallet password, or wallet file contents.
- Never tee or paste wallet creation transcripts if they may include secret material.
- Never commit files from /root/.kaspa/.
- Never commit wallet files or password files.
- Public artifacts may include public addresses, txids, balances, UTXO outpoints, command names, and non-secret server/network info.
- Public artifacts must not include wallet password, mnemonic, seed phrase, private key, or wallet file contents.

Known local-only wallet files:

/root/.kaspa/env052-tn10-test-only.wallet
/root/.kaspa/env052-tn10-test-only.password

Current project boundary:

- TN10 ordinary wallet/faucet/UTXO/sign/broadcast is proven.
- Live covenant create/spend/inspect is not yet proven.
- Roulette remains paused.
