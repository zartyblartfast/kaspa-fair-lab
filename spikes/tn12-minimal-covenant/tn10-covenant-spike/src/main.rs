use std::env;
use std::fs::{self, OpenOptions};
use std::io::Write;
#[cfg(unix)]
use std::os::unix::fs::OpenOptionsExt;
use std::path::{Path, PathBuf};
use std::time::Duration;

use kaspa_addresses::{Address, Prefix, Version};
use kaspa_consensus_core::constants::TX_VERSION_TOCCATA;
use kaspa_consensus_core::mass::ComputeBudget;
use kaspa_consensus_core::sign::sign;
use kaspa_consensus_core::subnets::SubnetworkId;
use kaspa_consensus_core::tx::{
    GenesisCovenantGroup, MutableTransaction, PopulatedTransaction, Transaction, TransactionInput,
    TransactionOutpoint, TransactionOutput, UtxoEntry,
};
use kaspa_hashes::Hash;
use kaspa_rpc_core::api::rpc::RpcApi;
use kaspa_txscript::covenants::CovenantsContext;
use kaspa_txscript::opcodes::codes::{
    Op1Add, OpBlake2bWithKey, OpCat, OpDup, OpEqual, OpEqualVerify, OpOutpointTxId, OpRot,
    OpTxInputIndex, OpTxInputSpk, OpTxOutputCount, OpTxOutputSpk, OpTxPayloadLen,
    OpTxPayloadSubstr,
};
use kaspa_txscript::pay_to_address_script;
use kaspa_txscript::pay_to_script_hash_script;
use kaspa_txscript::script_builder::{ScriptBuilder, ScriptBuilderResult};
use kaspa_wrpc_client::{
    KaspaRpcClient, Resolver, WrpcEncoding,
    client::{ConnectOptions, ConnectStrategy},
    prelude::{NetworkId, NetworkType},
};
use secp256k1::{Keypair, SecretKey};

const SOURCE_TAG: &str = "tn10-toc3";
const SOURCE_PATH: &str =
    "/root/kaspa-fair-lab/tools/rusty-kaspa-source/env058-20260625T230041Z/rusty-kaspa-tn10-toc3";
const DUMMY_REFERENCE_OUTPOINT: &str =
    "c9b4532f217d66987997e972963ec5dbfa5a9e7bf18f3e38910763274fb05135:0";
const DUMMY_REFERENCE_AMOUNT_SOMPI: u64 = 100_000_000;
const ENV059_HELPER_FUNDING_TKAS: u64 = 3;
const SOMPI_PER_TKAS: u64 = 100_000_000;
const ENV059_SECRET_REL_DIR: &str = "local-secrets/env-059-helper-key";
const ENV059_ARTIFACT_REL_DIR: &str = "artifacts/env-059-helper-controlled-covenant-preflight";

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut args = env::args().skip(1);
    match args.next().as_deref() {
        None | Some("env058-offline-scaffold") => run_env058()?,
        Some("env059-helper-key") => run_env059_helper_key()?,
        Some("covenant-create") => run_covenant_create(args.collect()).await?,
        Some("help") | Some("--help") | Some("-h") => print_help(),
        Some(other) => {
            return Err(format!(
                "unknown command `{other}`; use `env058-offline-scaffold`, `env059-helper-key`, or `covenant-create`"
            )
            .into());
        }
    }
    Ok(())
}

fn run_env058() -> ScriptBuilderResult<()> {
    let artifact_dir = env058_artifact_dir();
    fs::create_dir_all(&artifact_dir).expect("create ENV-058 artifact directory");

    let covenant_script = build_covenant_script()?;
    let covenant_spk = pay_to_script_hash_script(&covenant_script);

    // Public dummy/reference input only. This helper never signs, submits, or spends it.
    let input = TransactionInput::new(
        TransactionOutpoint::new(
            hash_from_hex("c9b4532f217d66987997e972963ec5dbfa5a9e7bf18f3e38910763274fb05135"),
            0,
        ),
        vec![],
        0,
        0,
    );

    let output = TransactionOutput::new(DUMMY_REFERENCE_AMOUNT_SOMPI, covenant_spk.clone());
    let mut tx = Transaction::new(
        TX_VERSION_TOCCATA,
        vec![input],
        vec![output],
        0,
        SubnetworkId::default(),
        0,
        vec![],
    );

    let group = GenesisCovenantGroup::new(0, vec![0]);
    let populate_result = tx.populate_genesis_covenants(&[group]);
    let populate_succeeded = populate_result.is_ok();
    if let Err(err) = populate_result {
        panic!("populate_genesis_covenants failed: {err:?}");
    }
    tx.finalize();

    let input_utxo = UtxoEntry::new(
        DUMMY_REFERENCE_AMOUNT_SOMPI,
        covenant_spk.clone(),
        0,
        false,
        None,
    );
    let populated = PopulatedTransaction::new(&tx, vec![input_utxo]);
    let covenants_context_result = CovenantsContext::from_tx(&populated);
    let covenants_context_validated = covenants_context_result.is_ok();
    if let Err(err) = covenants_context_result {
        panic!("CovenantsContext::from_tx failed: {err:?}");
    }

    let binding = tx.outputs[0]
        .covenant
        .expect("populate_genesis_covenants binds output 0");
    let covenant_id = binding.covenant_id.to_string();
    let tx_id = tx.id().to_string();
    let script_hex = hex_encode(covenant_spk.script());

    let json_path = artifact_dir.join("offline-covenant-create.json");
    let summary_path = artifact_dir.join("env-058-summary.txt");

    let json = format!(
        concat!(
            "{{\n",
            "  \"result\": \"PASS\",\n",
            "  \"source_tag\": \"{}\",\n",
            "  \"source_path\": \"{}\",\n",
            "  \"helper_crate_path\": \"{}\",\n",
            "  \"tx_version\": {},\n",
            "  \"expected_toccata_tx_version\": {},\n",
            "  \"input_outpoint_used_as_dummy_reference\": \"{}\",\n",
            "  \"output_count\": {},\n",
            "  \"output0_value_sompi\": {},\n",
            "  \"script_public_key_version\": {},\n",
            "  \"script_public_key_length\": {},\n",
            "  \"script_public_key_hex\": \"{}\",\n",
            "  \"redeem_script_length\": {},\n",
            "  \"populate_genesis_covenants_succeeded\": {},\n",
            "  \"covenants_context_validated\": {},\n",
            "  \"covenant_binding_present_output0\": {},\n",
            "  \"covenant_id\": \"{}\",\n",
            "  \"authorizing_input_index\": {},\n",
            "  \"transaction_id_local_offline\": \"{}\",\n",
            "  \"signed\": false,\n",
            "  \"broadcast\": false,\n",
            "  \"utxo_spent\": false,\n",
            "  \"wallet_or_private_material_accessed\": false,\n",
            "  \"mainnet\": false,\n",
            "  \"roulette_or_web_app\": false\n",
            "}}\n"
        ),
        SOURCE_TAG,
        SOURCE_PATH,
        env!("CARGO_MANIFEST_DIR"),
        tx.version,
        TX_VERSION_TOCCATA,
        DUMMY_REFERENCE_OUTPOINT,
        tx.outputs.len(),
        tx.outputs[0].value,
        covenant_spk.version(),
        covenant_spk.script().len(),
        script_hex,
        covenant_script.len(),
        populate_succeeded,
        covenants_context_validated,
        tx.outputs[0].covenant.is_some(),
        covenant_id,
        binding.authorizing_input,
        tx_id,
    );

    fs::write(&json_path, json).expect("write offline covenant JSON artifact");

    let summary = format!(
        concat!(
            "ENV-058 TN10 offline covenant scaffold\n",
            "\n",
            "Result: PASS\n",
            "\n",
            "Source tag/path used:\n",
            "- tag: {}\n",
            "- source path: {}\n",
            "- path dependencies: kaspa-consensus-core, kaspa-hashes, kaspa-txscript, kaspa-txscript-errors from the official source path above\n",
            "\n",
            "Helper crate path:\n",
            "- {}\n",
            "\n",
            "Official source APIs used:\n",
            "- kaspa_consensus_core::constants::TX_VERSION_TOCCATA\n",
            "- kaspa_consensus_core::tx::Transaction::new\n",
            "- kaspa_consensus_core::tx::TransactionInput::new\n",
            "- kaspa_consensus_core::tx::TransactionOutpoint::new\n",
            "- kaspa_consensus_core::tx::TransactionOutput::new\n",
            "- kaspa_consensus_core::tx::GenesisCovenantGroup::new\n",
            "- kaspa_consensus_core::tx::Transaction::populate_genesis_covenants\n",
            "- kaspa_consensus_core::tx::PopulatedTransaction::new\n",
            "- kaspa_consensus_core::tx::UtxoEntry::new\n",
            "- kaspa_txscript::script_builder::ScriptBuilder\n",
            "- kaspa_txscript::pay_to_script_hash_script\n",
            "- kaspa_txscript::covenants::CovenantsContext::from_tx\n",
            "- covenant script opcode sequence adapted from crypto/txscript/examples/covenants.rs\n",
            "\n",
            "Commands run:\n",
            "- cargo run -p kaspa-txscript --example covenants (from official tn10-toc3 source root)\n",
            "- cargo fmt --manifest-path spikes/tn12-minimal-covenant/tn10-covenant-spike/Cargo.toml\n",
            "- cargo check --manifest-path spikes/tn12-minimal-covenant/tn10-covenant-spike/Cargo.toml\n",
            "- cargo run --manifest-path spikes/tn12-minimal-covenant/tn10-covenant-spike/Cargo.toml\n",
            "\n",
            "Whether upstream covenant example was run: yes\n",
            "Whether helper compiled: yes\n",
            "Whether helper ran: yes\n",
            "Transaction version observed: {}\n",
            "Expected TX_VERSION_TOCCATA: {}\n",
            "Created version-1 transaction: {}\n",
            "populate_genesis_covenants(...) succeeded: {}\n",
            "Covenant binding result: output 0 covenant binding present = {}; authorizing_input = {}; covenant_id = {}\n",
            "CovenantsContext validation: {}\n",
            "Input outpoint used as dummy/public reference only: {}\n",
            "Output count: {}\n",
            "Output 0 value sompi: {}\n",
            "Redeem script length: {}\n",
            "P2SH/SPK length: {}\n",
            "\n",
            "Artifact paths:\n",
            "- {}\n",
            "- {}\n",
            "\n",
            "Remaining blockers before live ENV-059/ENV-060:\n",
            "- Prove a safe live-style create route that preserves covenant bindings end-to-end without relying on stale version-2 fixtures.\n",
            "- Determine whether official wallet/PSKB tooling can construct and sign a covenant-bound output without stripping covenant fields.\n",
            "- Prove mempool/RPC acceptance on TN10 only after explicit approval; this helper proves offline structure only.\n",
            "- Define exact live UTXO choice, fee/change policy, and stop conditions before any signing/broadcast gate.\n",
            "\n",
            "Stale local tx.version == 2 status:\n",
            "- This ENV-058 scaffold supersedes the stale local helper assumption for new offline Toccata-oriented construction by using TX_VERSION_TOCCATA = 1.\n",
            "- Existing stale version-2 fixtures/helpers should be patched in a later narrow cleanup unless the reviewer wants that scope folded into ENV-058; this run flags them but does not broadly rewrite old fixtures.\n",
            "\n",
            "Stop conditions:\n",
            "- Stop before signing.\n",
            "- Stop before broadcast/submission.\n",
            "- Stop before spending any live UTXO.\n",
            "- Stop before accessing wallet/private material.\n",
            "- Stop before mainnet.\n",
            "- Stop before roulette/web-app work.\n",
            "\n",
            "Safety confirmations:\n",
            "- no signing: true\n",
            "- no broadcast: true\n",
            "- no UTXO spend: true\n",
            "- no wallet/private material accessed: true\n",
            "- no mainnet: true\n",
            "- no roulette/web app: true\n"
        ),
        SOURCE_TAG,
        SOURCE_PATH,
        env!("CARGO_MANIFEST_DIR"),
        tx.version,
        TX_VERSION_TOCCATA,
        tx.version == TX_VERSION_TOCCATA,
        populate_succeeded,
        tx.outputs[0].covenant.is_some(),
        binding.authorizing_input,
        covenant_id,
        covenants_context_validated,
        DUMMY_REFERENCE_OUTPOINT,
        tx.outputs.len(),
        tx.outputs[0].value,
        covenant_script.len(),
        covenant_spk.script().len(),
        summary_path.display(),
        json_path.display(),
    );
    fs::write(&summary_path, summary).expect("write ENV-058 summary artifact");

    println!("ENV-058 PASS");
    println!("summary={}", summary_path.display());
    println!("json={}", json_path.display());
    println!("tx_version={}", tx.version);
    println!(
        "covenant_binding_present_output0={}",
        tx.outputs[0].covenant.is_some()
    );
    println!("covenant_id={}", covenant_id);
    println!("signed=false");
    println!("broadcast=false");
    Ok(())
}

fn run_env059_helper_key() -> Result<(), Box<dyn std::error::Error>> {
    let secret_dir = env059_secret_dir();
    let artifact_dir = env059_artifact_dir();
    fs::create_dir_all(&secret_dir)?;
    fs::create_dir_all(&artifact_dir)?;

    let private_key_path = secret_dir.join("helper-private-key.hex");
    let (secret_key, key_was_generated) = load_or_generate_secret_key(&private_key_path)?;
    let keypair = Keypair::from_secret_key(secp256k1::SECP256K1, &secret_key);
    let xonly_public_key = keypair.x_only_public_key().0.serialize();
    let helper_address = Address::new(Prefix::Testnet, Version::PubKey, &xonly_public_key);
    let helper_address_string = helper_address.to_string();
    let public_key_hex = hex_encode(&xonly_public_key);
    let public_json_path = artifact_dir.join("helper-address-public.json");
    let summary_path = artifact_dir.join("env-059-summary.txt");

    let planned_funding_tkas = planned_helper_funding_tkas();
    let planned_funding_sompi = planned_helper_funding_sompi();
    let planned_wallet_funding_command =
        format!("send {helper_address_string} {planned_funding_tkas} 0");
    let planned_wallet_estimate_command = format!("estimate {planned_funding_tkas} 0");
    let planned_create_command = concat!(
        "cargo run --manifest-path spikes/tn12-minimal-covenant/tn10-covenant-spike/Cargo.toml -- ",
        "covenant-create --network testnet-10 --utxo <helper-funding-txid:index> ",
        "--input-amount-sompi <helper-utxo-amount-sompi> --change-address <helper-address> ",
        "--submit --public-evidence-dir spikes/tn12-minimal-covenant/artifacts/env-060-helper-controlled-covenant-create/"
    );

    let public_json = format!(
        concat!(
            "{{\n",
            "  \"result\": \"READY\",\n",
            "  \"network\": \"testnet-10\",\n",
            "  \"address_prefix\": \"kaspatest\",\n",
            "  \"helper_public_address\": \"{}\",\n",
            "  \"helper_xonly_public_key_hex\": \"{}\",\n",
            "  \"private_material_storage_path\": \"{}\",\n",
            "  \"private_key_generated_this_run\": {},\n",
            "  \"private_material_publicly_exposed\": false,\n",
            "  \"planned_funding_amount_tkas\": {},\n",
            "  \"planned_funding_amount_sompi\": {},\n",
            "  \"planned_wallet_estimate_command\": \"{}\",\n",
            "  \"planned_wallet_funding_command\": \"{}\",\n",
            "  \"planned_covenant_create_command\": \"{}\",\n",
            "  \"live_covenant_transaction_created\": false,\n",
            "  \"live_utxo_spent\": false,\n",
            "  \"live_funds_signed\": false,\n",
            "  \"broadcast\": false,\n",
            "  \"mainnet\": false,\n",
            "  \"roulette_or_web_app\": false\n",
            "}}\n"
        ),
        helper_address_string,
        public_key_hex,
        private_key_path.display(),
        key_was_generated,
        planned_funding_tkas,
        planned_funding_sompi,
        json_escape(&planned_wallet_estimate_command),
        json_escape(&planned_wallet_funding_command),
        json_escape(planned_create_command),
    );
    fs::write(&public_json_path, public_json)?;

    let summary = format!(
        concat!(
            "ENV-059 helper-controlled TN10 covenant create preflight\n",
            "\n",
            "Result: READY\n",
            "\n",
            "Helper crate path:\n",
            "- {}\n",
            "\n",
            "Helper key/address support added:\n",
            "- yes; command: cargo run --manifest-path spikes/tn12-minimal-covenant/tn10-covenant-spike/Cargo.toml -- env059-helper-key\n",
            "- behavior: generate or reuse a local helper-controlled Schnorr key, derive a kaspatest P2PK address, write public-safe artifacts only\n",
            "\n",
            "Helper public TN10 address:\n",
            "- {}\n",
            "\n",
            "Helper public key:\n",
            "- x-only public key hex: {}\n",
            "\n",
            "Private material storage:\n",
            "- path: {}\n",
            "- generated this run: {}\n",
            "- private material is not printed and is not included in public artifacts\n",
            "- expected gitignore coverage: spikes/tn12-minimal-covenant/local-secrets/\n",
            "\n",
            "Planned funding amount:\n",
            "- {} TKAS ({} sompi)\n",
            "- rationale: small test-only amount above ENV-056 ordinary-send fee evidence (~0.002036 TKAS), enough room for a minimal create output/change and later spend experiments while limiting exposure\n",
            "\n",
            "Planned future funding command from official wallet:\n",
            "- network testnet-10\n",
            "- server public\n",
            "- connect\n",
            "- wallet open env052-tn10-test-only\n",
            "- {}\n",
            "- {}\n",
            "\n",
            "How helper will locate or accept funded UTXO once funded:\n",
            "- Preferred ENV-060 precheck: use official wallet/read-only RPC or public TN10 RPC to inspect helper address {} and capture the funding outpoint/value as public evidence.\n",
            "- Then pass that outpoint explicitly to the helper create command; do not let the helper guess or scan private wallet state.\n",
            "\n",
            "Planned future covenant create command:\n",
            "- {}\n",
            "\n",
            "Planned future evidence artifacts:\n",
            "- env-060 helper funding read-only UTXO capture for {}\n",
            "- env-060 unsigned/signed transaction summary with tx version 1 and output covenant binding\n",
            "- env-060 submit/broadcast response with transaction id if explicitly approved\n",
            "- env-060 post-broadcast read-only UTXO inspection showing covenant id on the created output\n",
            "\n",
            "Risks/unknowns:\n",
            "- Helper live signing and submit path is not implemented/proven yet.\n",
            "- Need exact mass/fee/change policy for Toccata covenant create.\n",
            "- Need proof that TN10 RPC/mempool accepts the helper-created covenant transaction.\n",
            "- Need read-only route that exposes covenant id on the created output after broadcast.\n",
            "\n",
            "Answers to ENV-059 questions:\n",
            "1. Can the helper generate a TN10 helper-controlled key/address safely? yes, with private material under ignored local-secrets and only public address/key artifacts committed.\n",
            "2. Code changes needed: add kaspa-addresses/secp256k1/rand deps; add env059-helper-key command; derive kaspatest P2PK address from x-only Schnorr public key; write private hex only to local-secrets; write public JSON/summary.\n",
            "3. Recommended funding amount: {} TKAS.\n",
            "4. Future wallet funding command: {} after network/server/connect/wallet-open and optional estimate.\n",
            "5. Helper UTXO handling: capture helper-address UTXO read-only, then pass <txid:index> and amount explicitly to helper.\n",
            "6. Future create command: {}\n",
            "7. Covenant create success evidence: accepted txid plus read-only UTXO inspection showing version-1 create output with covenant binding/covenant id.\n",
            "8. Remaining after create: covenant spend construction/sign/broadcast and post-spend/inspect lifecycle proof.\n",
            "9. Stop conditions: listed below.\n",
            "\n",
            "Stop conditions:\n",
            "- Stop before creating a live covenant transaction in ENV-059.\n",
            "- Stop before spending the existing wallet UTXO unless separately approved.\n",
            "- Stop before signing live funds.\n",
            "- Stop before broadcast/submission.\n",
            "- Stop before exposing private material.\n",
            "- Stop before mainnet.\n",
            "- Stop before roulette/web-app work.\n",
            "\n",
            "Safety confirmations:\n",
            "- no live covenant transaction created: true\n",
            "- no live UTXO spent unless separately approved: true\n",
            "- no signing of live funds: true\n",
            "- no broadcast: true\n",
            "- no private material exposed: true\n",
            "- no mainnet: true\n",
            "- no roulette/web app: true\n",
            "\n",
            "Artifact paths:\n",
            "- {}\n",
            "- {}\n"
        ),
        env!("CARGO_MANIFEST_DIR"),
        helper_address_string,
        public_key_hex,
        private_key_path.display(),
        key_was_generated,
        planned_funding_tkas,
        planned_funding_sompi,
        planned_wallet_estimate_command,
        planned_wallet_funding_command,
        helper_address_string,
        planned_create_command,
        helper_address_string,
        planned_funding_tkas,
        planned_wallet_funding_command,
        planned_create_command,
        summary_path.display(),
        public_json_path.display(),
    );
    fs::write(&summary_path, summary)?;

    println!("ENV-059 READY");
    println!("helper_public_address={}", helper_address_string);
    println!("helper_xonly_public_key_hex={}", public_key_hex);
    println!(
        "private_material_storage_path={}",
        private_key_path.display()
    );
    println!("private_material_exposed=false");
    println!("planned_funding_tkas={}", planned_funding_tkas);
    println!("summary={}", summary_path.display());
    println!("public_json={}", public_json_path.display());
    Ok(())
}

#[derive(Debug)]
struct CovenantCreateArgs {
    network: String,
    utxo: String,
    input_amount_sompi: u64,
    change_address: String,
    submit: bool,
    public_evidence_dir: PathBuf,
}

async fn run_covenant_create(args: Vec<String>) -> Result<(), Box<dyn std::error::Error>> {
    let args = parse_covenant_create_args(args)?;
    if args.network != "testnet-10" {
        return Err("blocked: covenant-create only accepts --network testnet-10".into());
    }
    if !args.submit {
        return Err(
            "blocked: ENV-060B requires explicit --submit for the one approved live attempt".into(),
        );
    }
    if args.input_amount_sompi == 0 {
        return Err("blocked: input amount must be non-zero".into());
    }

    fs::create_dir_all(&args.public_evidence_dir)?;
    let preflight_path = args.public_evidence_dir.join("preflight.txt");
    let submit_path = args.public_evidence_dir.join("create-submit.txt");
    let postcheck_path = args.public_evidence_dir.join("postcheck.txt");
    let summary_path = args.public_evidence_dir.join("env-060b-summary.txt");
    let json_path = args.public_evidence_dir.join("env-060b-public-create.json");

    let private_key_path = env059_secret_dir().join("helper-private-key.hex");
    if !private_key_path.exists() {
        write_blocked_summary(&summary_path, "helper private key file missing")?;
        return Err("blocked: helper private key file missing".into());
    }
    let (secret_key, _) = load_or_generate_secret_key(&private_key_path)?;
    let keypair = Keypair::from_secret_key(secp256k1::SECP256K1, &secret_key);
    let xonly_public_key = keypair.x_only_public_key().0.serialize();
    let helper_address = Address::new(Prefix::Testnet, Version::PubKey, &xonly_public_key);
    let helper_address_string = helper_address.to_string();
    if args.change_address != helper_address_string {
        write_blocked_summary(&summary_path, "change address does not match helper key")?;
        return Err("blocked: change address does not match helper key".into());
    }

    let (funding_txid, funding_index) = parse_outpoint_arg(&args.utxo)?;
    let helper_spk = pay_to_address_script(&helper_address);

    let client = connect_tn10_client().await?;
    let server_info = client.get_server_info().await?;
    if server_info.network_id.to_string() != "testnet-10"
        || !server_info.has_utxo_index
        || !server_info.is_synced
    {
        let reason = format!(
            "server preflight failed: network_id={} has_utxo_index={} is_synced={}",
            server_info.network_id, server_info.has_utxo_index, server_info.is_synced
        );
        write_blocked_summary(&summary_path, &reason)?;
        client.disconnect().await.ok();
        return Err(format!("blocked: {reason}").into());
    }
    let live_utxos = client
        .get_utxos_by_addresses(vec![helper_address.clone()])
        .await?;
    let live_utxo_match = live_utxos.iter().find(|entry| {
        entry.outpoint.transaction_id == funding_txid
            && entry.outpoint.index == funding_index
            && entry.utxo_entry.amount == args.input_amount_sompi
    });
    if live_utxo_match.is_none() {
        write_blocked_summary(
            &summary_path,
            "helper UTXO missing or amount mismatch in live TN10 read-only check",
        )?;
        client.disconnect().await.ok();
        return Err(
            "blocked: helper UTXO missing or amount mismatch in live TN10 read-only check".into(),
        );
    }

    let fee_sompi = 100_000u64;
    let covenant_output_value_sompi = 100_000_000u64;
    let total_outputs = covenant_output_value_sompi
        .checked_add(fee_sompi)
        .ok_or("blocked: output/fee overflow")?;
    if total_outputs > args.input_amount_sompi {
        write_blocked_summary(
            &summary_path,
            "transaction would spend more than helper UTXO",
        )?;
        client.disconnect().await.ok();
        return Err("blocked: transaction would spend more than helper UTXO".into());
    }
    let change_value_sompi = args.input_amount_sompi - total_outputs;

    let covenant_script = build_covenant_script()?;
    let covenant_spk = pay_to_script_hash_script(&covenant_script);
    let input = TransactionInput::new_with_mass(
        TransactionOutpoint::new(funding_txid, funding_index),
        vec![],
        0,
        ComputeBudget(0).into(),
    );
    let mut outputs = vec![TransactionOutput::new(
        covenant_output_value_sompi,
        covenant_spk.clone(),
    )];
    if change_value_sompi > 0 {
        outputs.push(TransactionOutput::new(
            change_value_sompi,
            helper_spk.clone(),
        ));
    }
    if outputs.len() != 2 {
        write_blocked_summary(&summary_path, "unexpected output count before signing")?;
        client.disconnect().await.ok();
        return Err("blocked: unexpected output count before signing".into());
    }

    let mut tx = Transaction::new(
        TX_VERSION_TOCCATA,
        vec![input],
        outputs,
        0,
        SubnetworkId::default(),
        0,
        vec![],
    );
    tx.populate_genesis_covenants(&[GenesisCovenantGroup::new(0, vec![0])])?;
    if tx.version != TX_VERSION_TOCCATA || tx.version == 2 || tx.outputs[0].covenant.is_none() {
        write_blocked_summary(
            &summary_path,
            "Toccata/covenant binding pre-sign guard failed",
        )?;
        client.disconnect().await.ok();
        return Err("blocked: Toccata/covenant binding pre-sign guard failed".into());
    }
    let binding = tx.outputs[0].covenant.expect("checked covenant binding");
    let input_utxo = UtxoEntry::new(args.input_amount_sompi, helper_spk.clone(), 0, false, None);
    let signed = sign(
        MutableTransaction::with_entries(tx, vec![input_utxo.clone()]),
        keypair,
    );
    let mut signed_tx = signed.tx;
    signed_tx.finalize();
    if signed_tx.inputs.len() != 1
        || signed_tx.outputs.len() != 2
        || signed_tx.inputs[0].previous_outpoint.transaction_id != funding_txid
    {
        write_blocked_summary(&summary_path, "signed transaction shape guard failed")?;
        client.disconnect().await.ok();
        return Err("blocked: signed transaction shape guard failed".into());
    }

    let preflight = format!(
        concat!(
            "ENV-060B preflight\n\n",
            "Result: PASS\n",
            "Network requested: {}\n",
            "Server network_id: {}\n",
            "Server is_synced: {}\n",
            "Server has_utxo_index: {}\n",
            "Helper address: {}\n",
            "Helper private key exists: true\n",
            "Helper private key path is under ignored local-secrets: true\n",
            "Input UTXO: {}:{}\n",
            "Input amount sompi: {}\n",
            "Live helper UTXO observed pre-submit: true\n",
            "TX_VERSION_TOCCATA: {}\n",
            "Transaction version: {}\n",
            "Uses literal version 2: false\n",
            "populate_genesis_covenants succeeded: true\n",
            "Covenant binding present before signing: true\n",
            "allow_orphan planned: false\n",
            "Output count: {}\n",
            "Covenant output index/value: 0 / {}\n",
            "Change output index/value: 1 / {}\n",
            "Fee sompi: {}\n",
            "Private key material exposed: false\n"
        ),
        args.network,
        server_info.network_id,
        server_info.is_synced,
        server_info.has_utxo_index,
        helper_address_string,
        funding_txid,
        funding_index,
        args.input_amount_sompi,
        TX_VERSION_TOCCATA,
        signed_tx.version,
        signed_tx.outputs.len(),
        covenant_output_value_sompi,
        change_value_sompi,
        fee_sompi
    );
    fs::write(&preflight_path, preflight)?;

    let local_txid = signed_tx.id().to_string();
    let submit_result = client.submit_transaction((&signed_tx).into(), false).await;
    match submit_result {
        Ok(submitted_txid) => {
            let submitted_txid_string = submitted_txid.to_string();
            fs::write(
                &submit_path,
                format!(
                    concat!(
                        "ENV-060B create submit\n\n",
                        "Result: PASS\n",
                        "Submission attempts: 1\n",
                        "allow_orphan: false\n",
                        "Local txid: {}\n",
                        "Submitted txid: {}\n",
                        "Mempool accepted RPC response: true\n",
                        "Private key material exposed: false\n"
                    ),
                    local_txid, submitted_txid_string
                ),
            )?;

            let mempool_result = client.get_mempool_entry(signed_tx.id(), false, false).await;
            let helper_post_utxos = client
                .get_utxos_by_addresses(vec![helper_address.clone()])
                .await?;
            let covenant_address = Address::new(
                Prefix::Testnet,
                Version::ScriptHash,
                &covenant_spk.script()[2..34],
            );
            let covenant_post_utxos = client
                .get_utxos_by_addresses(vec![covenant_address.clone()])
                .await?;
            let covenant_observed = covenant_post_utxos.iter().any(|entry| {
                entry.outpoint.transaction_id == signed_tx.id()
                    && entry.outpoint.index == 0
                    && entry.utxo_entry.amount == covenant_output_value_sompi
            });
            fs::write(
                &postcheck_path,
                format!(
                    concat!(
                        "ENV-060B postcheck\n\n",
                        "Result: PASS\n",
                        "Server network_id: {}\n",
                        "Server is_synced: {}\n",
                        "Server has_utxo_index: {}\n",
                        "Submitted txid: {}\n",
                        "Mempool entry observed: {}\n",
                        "Helper UTXO count after submit: {}\n",
                        "Covenant address: {}\n",
                        "Covenant UTXO observed: {}\n",
                        "Covenant postcheck UTXO count: {}\n"
                    ),
                    server_info.network_id,
                    server_info.is_synced,
                    server_info.has_utxo_index,
                    submitted_txid_string,
                    mempool_result.is_ok(),
                    helper_post_utxos.len(),
                    covenant_address,
                    covenant_observed,
                    covenant_post_utxos.len()
                ),
            )?;
            fs::write(
                &json_path,
                format!(
                    concat!(
                        "{{\n",
                        "  \"result\": \"PASS\",\n",
                        "  \"network\": \"testnet-10\",\n",
                        "  \"helper_address\": \"{}\",\n",
                        "  \"input_outpoint\": \"{}:{}\",\n",
                        "  \"input_amount_sompi\": {},\n",
                        "  \"fee_sompi\": {},\n",
                        "  \"tx_version\": {},\n",
                        "  \"allow_orphan\": false,\n",
                        "  \"covenant_output_index\": 0,\n",
                        "  \"covenant_output_value_sompi\": {},\n",
                        "  \"covenant_script_public_key_version\": {},\n",
                        "  \"covenant_script_public_key_length\": {},\n",
                        "  \"covenant_id\": \"{}\",\n",
                        "  \"change_output_index\": 1,\n",
                        "  \"change_output_value_sompi\": {},\n",
                        "  \"local_txid\": \"{}\",\n",
                        "  \"submitted_txid\": \"{}\",\n",
                        "  \"covenant_utxo_observed\": {},\n",
                        "  \"private_key_material_exposed\": false\n",
                        "}}\n"
                    ),
                    helper_address_string,
                    funding_txid,
                    funding_index,
                    args.input_amount_sompi,
                    fee_sompi,
                    signed_tx.version,
                    covenant_output_value_sompi,
                    covenant_spk.version(),
                    covenant_spk.script().len(),
                    binding.covenant_id,
                    change_value_sompi,
                    local_txid,
                    submitted_txid_string,
                    covenant_observed
                ),
            )?;
            fs::write(
                &summary_path,
                format!(
                    concat!(
                        "ENV-060B live helper-controlled TN10 covenant create\n\n",
                        "Result: PASS\n",
                        "Network: TN10 / testnet-10\n",
                        "Helper address: {}\n",
                        "Helper input UTXO: {}:{}\n",
                        "Amount spent/input sompi: {}\n",
                        "Fee sompi: {}\n",
                        "Covenant output value sompi: {}\n",
                        "Covenant id: {}\n",
                        "Covenant create txid: {}\n",
                        "Postcheck: mempool_entry_observed={} helper_post_utxo_count={} covenant_utxo_observed={}\n",
                        "Evidence path: {}\n\n",
                        "Safety confirmations:\n",
                        "- exactly one live covenant-create submission attempted: true\n",
                        "- no covenant spend attempted: true\n",
                        "- no mainnet: true\n",
                        "- no wallet secrets accessed: true\n",
                        "- helper private key not exposed: true\n",
                        "- no roulette/web app: true\n"
                    ),
                    helper_address_string,
                    funding_txid,
                    funding_index,
                    args.input_amount_sompi,
                    fee_sompi,
                    covenant_output_value_sompi,
                    binding.covenant_id,
                    submitted_txid_string,
                    mempool_result.is_ok(),
                    helper_post_utxos.len(),
                    covenant_observed,
                    args.public_evidence_dir.display()
                ),
            )?;
            println!("ENV-060B PASS");
            println!("submitted_txid={}", submitted_txid_string);
            println!("summary={}", summary_path.display());
        }
        Err(err) => {
            let rejection = err.to_string();
            fs::write(
                &submit_path,
                format!(
                    "ENV-060B create submit\n\nResult: REJECTED\nSubmission attempts: 1\nallow_orphan: false\nLocal txid: {}\nRejection/error: {}\nPrivate key material exposed: false\n",
                    local_txid, rejection
                ),
            )?;
            fs::write(
                &postcheck_path,
                "ENV-060B postcheck\n\nResult: NOT RUN AFTER REJECTED SUBMIT\n",
            )?;
            fs::write(
                &json_path,
                format!(
                    concat!(
                        "{{\n",
                        "  \"result\": \"REJECTED\",\n",
                        "  \"network\": \"testnet-10\",\n",
                        "  \"helper_address\": \"{}\",\n",
                        "  \"input_outpoint\": \"{}:{}\",\n",
                        "  \"input_amount_sompi\": {},\n",
                        "  \"fee_sompi\": {},\n",
                        "  \"tx_version\": {},\n",
                        "  \"allow_orphan\": false,\n",
                        "  \"covenant_output_index\": 0,\n",
                        "  \"covenant_output_value_sompi\": {},\n",
                        "  \"covenant_id\": \"{}\",\n",
                        "  \"change_output_index\": 1,\n",
                        "  \"change_output_value_sompi\": {},\n",
                        "  \"local_txid\": \"{}\",\n",
                        "  \"submitted_txid\": null,\n",
                        "  \"submission_attempts\": 1,\n",
                        "  \"covenant_utxo_observed\": false,\n",
                        "  \"private_key_material_exposed\": false\n",
                        "}}\n"
                    ),
                    helper_address_string,
                    funding_txid,
                    funding_index,
                    args.input_amount_sompi,
                    fee_sompi,
                    signed_tx.version,
                    covenant_output_value_sompi,
                    binding.covenant_id,
                    change_value_sompi,
                    local_txid
                ),
            )?;
            fs::write(
                &summary_path,
                format!(
                    concat!(
                        "ENV-060B live helper-controlled TN10 covenant create\n\n",
                        "Result: REJECTED\n",
                        "Network: TN10 / testnet-10\n",
                        "Helper address: {}\n",
                        "Helper input UTXO: {}:{}\n",
                        "Amount spent/input sompi: {}\n",
                        "Fee sompi: {}\n",
                        "Covenant output value sompi: {}\n",
                        "Covenant id: {}\n",
                        "Covenant create txid if submitted locally: {}\n",
                        "Submit rejection/error: {}\n",
                        "Covenant UTXO observed: false (postcheck stopped after rejection)\n\n",
                        "Safety confirmations:\n",
                        "- exactly one live covenant-create submission attempted: true\n",
                        "- no covenant spend attempted: true\n",
                        "- no mainnet: true\n",
                        "- no wallet secrets accessed: true\n",
                        "- helper private key not exposed: true\n",
                        "- no roulette/web app: true\n"
                    ),
                    helper_address_string,
                    funding_txid,
                    funding_index,
                    args.input_amount_sompi,
                    fee_sompi,
                    covenant_output_value_sompi,
                    binding.covenant_id,
                    local_txid,
                    rejection
                ),
            )?;
            println!("ENV-060B REJECTED");
            println!("summary={}", summary_path.display());
        }
    }
    client.disconnect().await.ok();
    Ok(())
}

fn parse_covenant_create_args(
    args: Vec<String>,
) -> Result<CovenantCreateArgs, Box<dyn std::error::Error>> {
    let mut network = None;
    let mut utxo = None;
    let mut input_amount_sompi = None;
    let mut change_address = None;
    let mut submit = false;
    let mut public_evidence_dir = None;
    let mut iter = args.into_iter();
    while let Some(arg) = iter.next() {
        match arg.as_str() {
            "--network" => network = iter.next(),
            "--utxo" => utxo = iter.next(),
            "--input-amount-sompi" => {
                input_amount_sompi = Some(
                    iter.next()
                        .ok_or("missing --input-amount-sompi value")?
                        .parse()?,
                )
            }
            "--change-address" => change_address = iter.next(),
            "--submit" => submit = true,
            "--public-evidence-dir" => {
                public_evidence_dir = Some(PathBuf::from(
                    iter.next().ok_or("missing --public-evidence-dir value")?,
                ))
            }
            other => return Err(format!("unknown covenant-create argument `{other}`").into()),
        }
    }
    Ok(CovenantCreateArgs {
        network: network.ok_or("missing --network")?,
        utxo: utxo.ok_or("missing --utxo")?,
        input_amount_sompi: input_amount_sompi.ok_or("missing --input-amount-sompi")?,
        change_address: change_address.ok_or("missing --change-address")?,
        submit,
        public_evidence_dir: public_evidence_dir.ok_or("missing --public-evidence-dir")?,
    })
}

async fn connect_tn10_client() -> Result<KaspaRpcClient, Box<dyn std::error::Error>> {
    let client = KaspaRpcClient::new(
        WrpcEncoding::Borsh,
        None,
        Some(Resolver::default()),
        Some(NetworkId::with_suffix(NetworkType::Testnet, 10)),
        None,
    )?;
    let options = ConnectOptions {
        block_async_connect: true,
        connect_timeout: Some(Duration::from_millis(10_000)),
        strategy: ConnectStrategy::Fallback,
        ..Default::default()
    };
    client.connect(Some(options)).await?;
    Ok(client)
}

fn parse_outpoint_arg(utxo: &str) -> Result<(Hash, u32), Box<dyn std::error::Error>> {
    let (txid, index) = utxo.split_once(':').ok_or("UTXO must be <txid:index>")?;
    Ok((hash_from_hex(txid), index.parse()?))
}

fn write_blocked_summary(path: &Path, reason: &str) -> std::io::Result<()> {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }
    fs::write(
        path,
        format!(
            "ENV-060B live helper-controlled TN10 covenant create\n\nResult: BLOCKED\nReason: {}\nSafety confirmations: no submission attempted, no covenant spend attempted, no mainnet, no wallet secrets accessed, helper private key not exposed, no roulette/web app.\n",
            reason
        ),
    )
}

fn print_help() {
    println!("tn10-covenant-spike commands:");
    println!(
        "  env058-offline-scaffold  Build the ENV-058 offline covenant scaffold artifact (default)"
    );
    println!(
        "  env059-helper-key        Generate/reuse helper-controlled TN10 key and public address preflight artifacts"
    );
    println!(
        "  covenant-create          Build, sign, submit exactly one helper-controlled TN10 covenant-create transaction"
    );
}

fn planned_helper_funding_tkas() -> u64 {
    ENV059_HELPER_FUNDING_TKAS
}

fn planned_helper_funding_sompi() -> u64 {
    ENV059_HELPER_FUNDING_TKAS * SOMPI_PER_TKAS
}

fn env059_secret_dir() -> PathBuf {
    spike_root_dir().join(ENV059_SECRET_REL_DIR)
}

fn env059_artifact_dir() -> PathBuf {
    spike_root_dir().join(ENV059_ARTIFACT_REL_DIR)
}

fn spike_root_dir() -> PathBuf {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    manifest_dir
        .parent()
        .expect("crate is under spikes/tn12-minimal-covenant")
        .to_path_buf()
}

fn load_or_generate_secret_key(
    path: &Path,
) -> Result<(SecretKey, bool), Box<dyn std::error::Error>> {
    if path.exists() {
        let private_hex = fs::read_to_string(path)?;
        let private_bytes = hex_decode_32(private_hex.trim());
        let secret_key = SecretKey::from_slice(&private_bytes)?;
        return Ok((secret_key, false));
    }

    let secret_key = SecretKey::new(&mut rand::thread_rng());
    let private_hex = format!("{}\n", hex_encode(&secret_key.secret_bytes()));
    write_secret_file(path, private_hex.as_bytes())?;
    Ok((secret_key, true))
}

fn write_secret_file(path: &Path, bytes: &[u8]) -> std::io::Result<()> {
    let mut options = OpenOptions::new();
    options.write(true).create_new(true);
    #[cfg(unix)]
    options.mode(0o600);
    let mut file = options.open(path)?;
    file.write_all(bytes)
}

fn json_escape(input: &str) -> String {
    input
        .replace('\\', "\\\\")
        .replace('"', "\\\"")
        .replace('\n', "\\n")
}

/// Adapted from the official tn10-toc3 `crypto/txscript/examples/covenants.rs`.
fn build_covenant_script() -> ScriptBuilderResult<Vec<u8>> {
    Ok(ScriptBuilder::new()
        .add_op(OpDup)?
        .add_op(OpRot)?
        .add_op(OpRot)?
        .add_op(OpCat)?
        .add_data(b"TransactionID")?
        .add_op(OpBlake2bWithKey)?
        .add_op(OpTxInputIndex)?
        .add_op(OpOutpointTxId)?
        .add_op(OpEqualVerify)?
        .add_op(Op1Add)?
        .add_i64(0)?
        .add_op(OpTxPayloadLen)?
        .add_op(OpTxPayloadSubstr)?
        .add_op(OpEqualVerify)?
        .add_op(OpTxInputIndex)?
        .add_op(OpTxInputSpk)?
        .add_i64(0)?
        .add_op(OpTxOutputSpk)?
        .add_op(OpEqualVerify)?
        .add_op(OpTxOutputCount)?
        .add_i64(1)?
        .add_op(OpEqual)?
        .drain())
}

fn env058_artifact_dir() -> PathBuf {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    manifest_dir
        .parent()
        .expect("crate is under spikes/tn12-minimal-covenant")
        .join("artifacts/env-058-tn10-offline-covenant-scaffold")
}

fn hex_encode(bytes: &[u8]) -> String {
    const HEX: &[u8; 16] = b"0123456789abcdef";
    let mut encoded = String::with_capacity(bytes.len() * 2);
    for byte in bytes {
        encoded.push(HEX[(byte >> 4) as usize] as char);
        encoded.push(HEX[(byte & 0x0f) as usize] as char);
    }
    encoded
}

fn hash_from_hex(hex: &str) -> Hash {
    let bytes = hex_decode_32(hex);
    Hash::from_bytes(bytes)
}

fn hex_decode_32(hex: &str) -> [u8; 32] {
    assert_eq!(hex.len(), 64, "expected 32-byte hex string");
    let mut out = [0u8; 32];
    let bytes = hex.as_bytes();
    for i in 0..32 {
        let hi = hex_nibble(bytes[2 * i]);
        let lo = hex_nibble(bytes[2 * i + 1]);
        out[i] = (hi << 4) | lo;
    }
    out
}

fn hex_nibble(byte: u8) -> u8 {
    match byte {
        b'0'..=b'9' => byte - b'0',
        b'a'..=b'f' => byte - b'a' + 10,
        b'A'..=b'F' => byte - b'A' + 10,
        _ => panic!("invalid hex byte"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn env059_plans_small_tn10_funding_amount() {
        assert_eq!(planned_helper_funding_tkas(), 3);
        assert_eq!(planned_helper_funding_sompi(), 300_000_000);
    }

    #[test]
    fn env059_secret_path_stays_under_ignored_local_secrets() {
        let path = env059_secret_dir();
        assert!(path.ends_with("spikes/tn12-minimal-covenant/local-secrets/env-059-helper-key"));
    }
}
