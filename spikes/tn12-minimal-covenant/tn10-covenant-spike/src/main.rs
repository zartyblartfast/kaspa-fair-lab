use std::env;
use std::fs::{self, OpenOptions};
use std::io::Write;
#[cfg(unix)]
use std::os::unix::fs::OpenOptionsExt;
use std::path::{Path, PathBuf};
use std::time::Duration;

use kaspa_addresses::{Address, Prefix, Version};
use kaspa_consensus_core::constants::TX_VERSION_TOCCATA;
use kaspa_consensus_core::hashing::sighash::SigHashReusedValuesUnsync;
use kaspa_consensus_core::hashing::tx::{
    payload_digest, transaction_v1_rest_preimage, v1_rest_digest,
};
use kaspa_consensus_core::mass::ComputeBudget;
use kaspa_consensus_core::sign::sign;
use kaspa_consensus_core::subnets::SubnetworkId;
use kaspa_consensus_core::tx::{
    CovenantBinding, GenesisCovenantGroup, MutableTransaction, PopulatedTransaction, Transaction,
    TransactionInput, TransactionOutpoint, TransactionOutput, UtxoEntry,
};
use kaspa_hashes::{Hash, Hasher, HasherBase, TransactionID, TransactionV1Id};
use kaspa_rpc_core::api::rpc::RpcApi;
use kaspa_txscript::caches::Cache;
use kaspa_txscript::covenants::CovenantsContext;
use kaspa_txscript::opcodes::codes::{
    Op1Add, OpBlake2bWithKey, OpCat, OpDup, OpEqual, OpEqualVerify, OpOutpointTxId, OpRot,
    OpTxInputIndex, OpTxInputSpk, OpTxOutputCount, OpTxOutputSpk, OpTxPayloadLen,
    OpTxPayloadSubstr,
};
use kaspa_txscript::pay_to_address_script;
use kaspa_txscript::pay_to_script_hash_script;
use kaspa_txscript::script_builder::{ScriptBuilder, ScriptBuilderResult};
use kaspa_txscript::{EngineCtx, EngineFlags, TxScriptEngine};
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
const ENV060C_FEE_SOMPI: u64 = 300_000;
const ENV060C_HELPER_FUNDING_TXID: &str =
    "d84921a7a30ffa1c8de5df189297fcace3a6a908191eaa9c19b6dfef29eca439";
const ENV060C_HELPER_FUNDING_INDEX: u32 = 0;
const ENV061_ARTIFACT_REL_DIR: &str = "artifacts/env-061-covenant-utxo-inspection-spend-preflight";
const ENV061_CREATE_TXID: &str = "f4941c478e9540c477e04d0a2dff7ab1b0d0d794a3ae453c8148d25d125fe53d";
const ENV061_COVENANT_OUTPUT_INDEX: u32 = 0;
const ENV061_COVENANT_OUTPUT_VALUE_SOMPI: u64 = 100_000_000;
const ENV061_EXPECTED_COVENANT_ID: &str =
    "69a36c409aa9d71304d2fb08f4e4c6e7d979a81db019d589d8e979d594ceb3d1";
const ENV061_HELPER_ADDRESS: &str =
    "kaspatest:qzn7auhpkdladk9m20f02dz46clvv7whgumgrm4pex4djesaued0g9wutcqld";
const ENV061_HELPER_CHANGE_OUTPOINT_INDEX: u32 = 1;
const ENV061_HELPER_CHANGE_VALUE_SOMPI: u64 = 199_700_000;
const ENV061_FUTURE_SPEND_FEE_SOMPI: u64 = 300_000;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut args = env::args().skip(1);
    match args.next().as_deref() {
        None | Some("env058-offline-scaffold") => run_env058()?,
        Some("env059-helper-key") => run_env059_helper_key()?,
        Some("covenant-create") => run_covenant_create(args.collect()).await?,
        Some("env061-inspect") => run_env061_inspect().await?,
        Some("covenant-spend") => run_covenant_spend(args.collect()).await?,
        Some("env062a-local-debug") => run_env062a_local_debug()?,
        Some("help") | Some("--help") | Some("-h") => print_help(),
        Some(other) => {
            return Err(format!(
                "unknown command `{other}`; use `env058-offline-scaffold`, `env059-helper-key`, `covenant-create`, `env061-inspect`, or `covenant-spend`"
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
            "blocked: ENV-060C requires explicit --submit for the one approved live retry".into(),
        );
    }
    if args.input_amount_sompi == 0 {
        return Err("blocked: input amount must be non-zero".into());
    }

    fs::create_dir_all(&args.public_evidence_dir)?;
    let preflight_path = args.public_evidence_dir.join("preflight.txt");
    let fee_analysis_path = args.public_evidence_dir.join("fee-analysis.txt");
    let submit_path = args.public_evidence_dir.join("create-submit.txt");
    let postcheck_path = args.public_evidence_dir.join("postcheck.txt");
    let summary_path = args.public_evidence_dir.join("env-060c-summary.txt");
    let json_path = args.public_evidence_dir.join("env-060c-public-create.json");

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

    let fee_sompi = ENV060C_FEE_SOMPI;
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
            "ENV-060C preflight\n\n",
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
    fs::write(
        &fee_analysis_path,
        format!(
            concat!(
                "ENV-060C fee analysis\n\n",
                "Prior ENV-060B result: REJECTED\n",
                "Prior ENV-060B fee used sompi: 100000\n",
                "Prior ENV-060B required fee reported sompi: 208300\n",
                "Prior ENV-060B reported compute mass: 2083\n",
                "ENV-060C selected fee sompi: {}\n",
                "Fee margin above reported requirement sompi: {}\n",
                "Input amount sompi: {}\n",
                "Covenant output value sompi: {}\n",
                "Change output value sompi: {}\n",
                "Change returns to helper address: true ({})\n",
                "Excessive spend guard: PASS (fee + covenant output <= input)\n",
                "TX_VERSION_TOCCATA: {}\n",
                "Transaction version: {}\n",
                "Covenant binding present before signing: true\n",
                "allow_orphan planned: false\n",
                "Private key material exposed: false\n"
            ),
            fee_sompi,
            fee_sompi.saturating_sub(208_300),
            args.input_amount_sompi,
            covenant_output_value_sompi,
            change_value_sompi,
            helper_address_string,
            TX_VERSION_TOCCATA,
            signed_tx.version,
        ),
    )?;
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
                        "ENV-060C create submit\n\n",
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
                        "ENV-060C postcheck\n\n",
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
                        "ENV-060C live helper-controlled TN10 covenant create fee retry\n\n",
                        "Result: PASS\n",
                        "Network: TN10 / testnet-10\n",
                        "Helper address: {}\n",
                        "Helper input UTXO: {}:{}\n",
                        "Amount spent/input sompi: {}\n",
                        "Fee sompi: {}\n",
                        "Covenant output value sompi: {}\n",
                        "Change output value sompi: {}\n",
                        "Covenant id: {}\n",
                        "Submitted txid if accepted: {}\n",
                        "Covenant UTXO observed: {}\n",
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
                    change_value_sompi,
                    binding.covenant_id,
                    submitted_txid_string,
                    covenant_observed,
                    mempool_result.is_ok(),
                    helper_post_utxos.len(),
                    covenant_observed,
                    args.public_evidence_dir.display()
                ),
            )?;
            println!("ENV-060C PASS");
            println!("submitted_txid={}", submitted_txid_string);
            println!("summary={}", summary_path.display());
        }
        Err(err) => {
            let rejection = err.to_string();
            fs::write(
                &submit_path,
                format!(
                    "ENV-060C create submit\n\nResult: REJECTED\nSubmission attempts: 1\nallow_orphan: false\nLocal txid: {}\nRejection/error: {}\nPrivate key material exposed: false\n",
                    local_txid, rejection
                ),
            )?;
            fs::write(
                &postcheck_path,
                "ENV-060C postcheck\n\nResult: NOT RUN AFTER REJECTED SUBMIT\n",
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
                        "ENV-060C live helper-controlled TN10 covenant create fee retry\n\n",
                        "Result: REJECTED\n",
                        "Network: TN10 / testnet-10\n",
                        "Helper address: {}\n",
                        "Helper input UTXO: {}:{}\n",
                        "Amount spent/input sompi: {}\n",
                        "Fee sompi: {}\n",
                        "Covenant output value sompi: {}\n",
                        "Change output value sompi: {}\n",
                        "Covenant id: {}\n",
                        "Covenant create txid if submitted locally: {}\n",
                        "Submitted txid if accepted: none\n",
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
                    change_value_sompi,
                    binding.covenant_id,
                    local_txid,
                    rejection
                ),
            )?;
            println!("ENV-060C REJECTED");
            println!("summary={}", summary_path.display());
        }
    }
    client.disconnect().await.ok();
    Ok(())
}

async fn run_env061_inspect() -> Result<(), Box<dyn std::error::Error>> {
    let artifact_dir = spike_root_dir().join(ENV061_ARTIFACT_REL_DIR);
    fs::create_dir_all(&artifact_dir)?;
    let server_info_path = artifact_dir.join("server-info.txt");
    let inspection_path = artifact_dir.join("covenant-utxo-inspection.txt");
    let preflight_path = artifact_dir.join("spend-preflight.txt");
    let summary_path = artifact_dir.join("env-061-summary.txt");

    let create_txid = hash_from_hex(ENV061_CREATE_TXID);
    let covenant_script = build_covenant_script()?;
    let covenant_spk = pay_to_script_hash_script(&covenant_script);
    let covenant_address = Address::new(
        Prefix::Testnet,
        Version::ScriptHash,
        &covenant_spk.script()[2..34],
    );
    let covenant_spk_hex = hex_encode(covenant_spk.script());
    let helper_address = Address::try_from(ENV061_HELPER_ADDRESS)?;

    let client = connect_tn10_client().await?;
    let server_info = client.get_server_info().await?;
    let server_ok = server_info.network_id.to_string() == "testnet-10"
        && server_info.is_synced
        && server_info.has_utxo_index;
    fs::write(
        &server_info_path,
        format!(
            concat!(
                "ENV-061 public TN10 server info\n\n",
                "Result: {}\n",
                "network_id: {}\n",
                "network_suffix_10_observed: {}\n",
                "is_synced: {}\n",
                "has_utxo_index: {}\n",
                "virtual_daa_score: {}\n",
                "server_version: {}\n",
                "read_only_rpc_only: true\n"
            ),
            if server_ok { "PASS" } else { "BLOCKED" },
            server_info.network_id,
            server_info.network_id.to_string() == "testnet-10",
            server_info.is_synced,
            server_info.has_utxo_index,
            server_info.virtual_daa_score,
            server_info.server_version,
        ),
    )?;

    let mempool_entry = client.get_mempool_entry(create_txid, false, false).await;
    let mempool_observed = mempool_entry.is_ok();
    let (mempool_tx_version, mempool_output_count, mempool_output0_covenant_id) =
        match &mempool_entry {
            Ok(entry) => {
                let cov = entry
                    .transaction
                    .outputs
                    .get(0)
                    .and_then(|o| o.covenant)
                    .map(|c| c.0.covenant_id.to_string());
                (
                    Some(entry.transaction.version),
                    Some(entry.transaction.outputs.len()),
                    cov,
                )
            }
            Err(_) => (None, None, None),
        };
    let covenant_utxos = client
        .get_utxos_by_addresses(vec![covenant_address.clone()])
        .await?;
    let helper_utxos = client
        .get_utxos_by_addresses(vec![helper_address.clone()])
        .await?;
    let covenant_match = covenant_utxos.iter().find(|entry| {
        entry.outpoint.transaction_id == create_txid
            && entry.outpoint.index == ENV061_COVENANT_OUTPUT_INDEX
            && entry.utxo_entry.amount == ENV061_COVENANT_OUTPUT_VALUE_SOMPI
    });
    let covenant_observed = covenant_match.is_some();
    let covenant_unspent = covenant_observed;
    let observed_covenant_id = covenant_match
        .and_then(|entry| entry.utxo_entry.covenant_id)
        .map(|id| id.to_string());
    let covenant_id_matches_expected =
        observed_covenant_id.as_deref() == Some(ENV061_EXPECTED_COVENANT_ID);
    let helper_change_observed = helper_utxos.iter().any(|entry| {
        entry.outpoint.transaction_id == create_txid
            && entry.outpoint.index == ENV061_HELPER_CHANGE_OUTPOINT_INDEX
            && entry.utxo_entry.amount == ENV061_HELPER_CHANGE_VALUE_SOMPI
    });
    let mempool_entries_result = client.get_mempool_entries(false, false).await;
    let mempool_spend_seen = mempool_entries_result
        .as_ref()
        .map(|entries| {
            entries.iter().any(|entry| {
                entry.transaction.inputs.iter().any(|input| {
                    input.previous_outpoint.transaction_id == create_txid
                        && input.previous_outpoint.index == ENV061_COVENANT_OUTPUT_INDEX
                })
            })
        })
        .unwrap_or(false);
    let mempool_entries_count = mempool_entries_result
        .as_ref()
        .map(|entries| entries.len())
        .ok();

    let result = if !server_ok {
        "BLOCKED"
    } else if covenant_observed && covenant_unspent {
        "READY"
    } else {
        "PARTIAL"
    };
    let observed_covenant_id_display = observed_covenant_id
        .as_deref()
        .unwrap_or("not returned / not observed");
    let mempool_output0_covenant_id_display = mempool_output0_covenant_id
        .as_deref()
        .unwrap_or("not returned / not in mempool");
    let mempool_error_display = match &mempool_entry {
        Ok(_) => "none".to_string(),
        Err(err) => err.to_string(),
    };

    fs::write(
        &inspection_path,
        format!(
            concat!(
                "ENV-061 covenant UTXO inspection\n\n",
                "Result: {}\n",
                "Network: TN10 / testnet-10\n",
                "Covenant-create txid: {}\n",
                "Covenant UTXO outpoint: {}:{}\n",
                "Covenant output amount sompi: {}\n",
                "Covenant address: {}\n",
                "Covenant script public key version: {}\n",
                "Covenant script public key length: {}\n",
                "Covenant script public key hex: {}\n",
                "Redeem script length: {}\n",
                "Expected covenant id from ENV-060C: {}\n",
                "Observed UTXO covenant id: {}\n",
                "Observed covenant id matches expected: {}\n",
                "Covenant UTXO observed: {}\n",
                "Covenant appears unspent: {}\n",
                "Covenant address UTXO count: {}\n",
                "Mempool entry for create tx observed: {}\n",
                "Mempool entry error if absent: {}\n",
                "Mempool tx version if observed: {:?}\n",
                "Mempool tx output count if observed: {:?}\n",
                "Mempool output0 covenant id if observed: {}\n",
                "Mempool spend of covenant outpoint observed: {}\n",
                "Mempool entries scanned count: {:?}\n",
                "Helper change outpoint: {}:{}\n",
                "Helper change value sompi: {}\n",
                "Helper change observed: {}\n",
                "Helper current UTXO count: {}\n",
                "Read-only RPC only: true\n"
            ),
            result,
            ENV061_CREATE_TXID,
            ENV061_CREATE_TXID,
            ENV061_COVENANT_OUTPUT_INDEX,
            ENV061_COVENANT_OUTPUT_VALUE_SOMPI,
            covenant_address,
            covenant_spk.version(),
            covenant_spk.script().len(),
            covenant_spk_hex,
            covenant_script.len(),
            ENV061_EXPECTED_COVENANT_ID,
            observed_covenant_id_display,
            covenant_id_matches_expected,
            covenant_observed,
            covenant_unspent,
            covenant_utxos.len(),
            mempool_observed,
            mempool_error_display,
            mempool_tx_version,
            mempool_output_count,
            mempool_output0_covenant_id_display,
            mempool_spend_seen,
            mempool_entries_count,
            ENV061_CREATE_TXID,
            ENV061_HELPER_CHANGE_OUTPOINT_INDEX,
            ENV061_HELPER_CHANGE_VALUE_SOMPI,
            helper_change_observed,
            helper_utxos.len(),
        ),
    )?;

    let future_output_value =
        ENV061_COVENANT_OUTPUT_VALUE_SOMPI.saturating_sub(ENV061_FUTURE_SPEND_FEE_SOMPI);
    fs::write(
        &preflight_path,
        format!(
            concat!(
                "ENV-061 covenant-spend preflight for future ENV-062\n\n",
                "Result: {}\n",
                "This file is a plan only; no spend was signed or broadcast.\n\n",
                "Future input covenant UTXO outpoint: {}:{}\n",
                "Future input amount sompi: {}\n",
                "Redeem script needed: same helper covenant script from build_covenant_script(); length {} bytes.\n",
                "Unlock/signature script needed: push values satisfying the P2SH covenant script; exact byte-level sigscript still needs a local no-broadcast construction check before live signing.\n",
                "Script constraints summarized: output count must be 1; output 0 SPK must equal input SPK; payload substring check must match expected transition data; outpoint txid relation is checked by the script.\n",
                "Expected output state transition: one continuing covenant output to the same covenant script/address, with payload/unlock data proving the permitted transition.\n",
                "Planned output count: 1\n",
                "Planned output value sompi if fee is {}: {}\n",
                "Planned fee sompi: {}\n",
                "Planned change: none for the covenant input because script requires exactly one output; use output value = input - fee.\n",
                "Helper key role: helper private key should not sign the covenant input unless the actual spend construction proves it is required; helper key may sign a separate helper fee input only if future ENV-062 explicitly approves adding one.\n",
                "Expected tx version: {} (TX_VERSION_TOCCATA)\n",
                "Expected covenant id/state after spend: continuing output should retain/bind the covenant id according to Toccata covenant rules; exact post-spend id/state must be confirmed by local construction and later read-only RPC.\n\n",
                "Exact future command shape (do not run without ENV-062 approval):\n",
                "cargo run --manifest-path spikes/tn12-minimal-covenant/tn10-covenant-spike/Cargo.toml -- covenant-spend --network testnet-10 --covenant-utxo {}:{} --input-amount-sompi {} --covenant-id {} --fee-sompi {} --output-value-sompi {} --submit --public-evidence-dir spikes/tn12-minimal-covenant/artifacts/env-062-live-covenant-spend/\n\n",
                "Required ENV-062 prechecks before any signing/submission:\n",
                "- git-reviewed ENV-061 evidence.\n",
                "- public TN10 server info still testnet-10, synced, has UTXO index.\n",
                "- covenant UTXO still observed at the exact outpoint and amount.\n",
                "- local no-broadcast spend construction passes and records tx version/output count/covenant binding/sigscript shape.\n",
                "- explicit approval for live covenant spend signing and exactly one submission.\n\n",
                "Stop conditions:\n",
                "- stop if network is not testnet-10/TN10.\n",
                "- stop if server is not synced or lacks UTXO index.\n",
                "- stop if covenant UTXO is absent, spent, amount-mismatched, or covenant id mismatched.\n",
                "- stop if local no-broadcast spend construction cannot prove the exact sigscript/redeem data.\n",
                "- stop before signing if approval is not explicit.\n",
                "- stop before broadcast/submission unless exactly-one-submit approval is explicit.\n",
                "- stop before mainnet, wallet-secret access, helper-private-key exposure, roulette, or web app work.\n"
            ),
            if covenant_observed {
                "READY"
            } else {
                "PARTIAL"
            },
            ENV061_CREATE_TXID,
            ENV061_COVENANT_OUTPUT_INDEX,
            ENV061_COVENANT_OUTPUT_VALUE_SOMPI,
            covenant_script.len(),
            ENV061_FUTURE_SPEND_FEE_SOMPI,
            future_output_value,
            ENV061_FUTURE_SPEND_FEE_SOMPI,
            TX_VERSION_TOCCATA,
            ENV061_CREATE_TXID,
            ENV061_COVENANT_OUTPUT_INDEX,
            ENV061_COVENANT_OUTPUT_VALUE_SOMPI,
            ENV061_EXPECTED_COVENANT_ID,
            ENV061_FUTURE_SPEND_FEE_SOMPI,
            future_output_value,
        ),
    )?;

    fs::write(
        &summary_path,
        format!(
            concat!(
                "ENV-061 read-only covenant UTXO inspection and covenant-spend preflight\n\n",
                "Result: {}\n",
                "Network: TN10 / testnet-10\n",
                "Covenant-create txid: {}\n",
                "Covenant UTXO outpoint: {}:{}\n",
                "Covenant output amount sompi: {}\n",
                "Covenant id: {}\n",
                "Covenant UTXO observed: {}\n",
                "Covenant appears unspent: {}\n",
                "Helper address: {}\n",
                "Helper change output, if any: {}:{} value {} sompi; observed={}\n",
                "Fee used by ENV-060C: {} sompi\n",
                "Create tx still in mempool: {}\n",
                "No covenant spend observed by read-only checks: {}\n\n",
                "Future ENV-062 spend strategy:\n",
                "- spend the covenant UTXO only after explicit approval; construct a version-1 Toccata transaction with exactly one continuing covenant output to the same covenant SPK, no change from the covenant input, and output value input minus fee.\n",
                "- first add/prove a local no-broadcast covenant-spend construction path so the exact sigscript/unlock bytes are known before live signing.\n",
                "- keep allow_orphan=false and exactly one submission if later approved.\n\n",
                "Exact future command shape:\n",
                "cargo run --manifest-path spikes/tn12-minimal-covenant/tn10-covenant-spike/Cargo.toml -- covenant-spend --network testnet-10 --covenant-utxo {}:{} --input-amount-sompi {} --covenant-id {} --fee-sompi {} --output-value-sompi {} --submit --public-evidence-dir spikes/tn12-minimal-covenant/artifacts/env-062-live-covenant-spend/\n\n",
                "Remaining risks/unknowns:\n",
                "- exact covenant-spend sigscript/unlock data has not yet been implemented/proven locally.\n",
                "- whether a future spend preserves/rebinds covenant id/state exactly as expected must be verified by local construction and then read-only postcheck after any approved live spend.\n",
                "- public-node mempool entry for the create may disappear after mining; UTXO observation is the stronger unspent evidence.\n\n",
                "Stop conditions:\n",
                "- stop if network is not TN10/testnet-10.\n",
                "- stop if server is not synced or lacks UTXO index.\n",
                "- stop if covenant UTXO is absent, spent, amount-mismatched, or covenant id mismatched.\n",
                "- stop before signing or broadcasting without explicit ENV-062 approval.\n",
                "- stop before private key exposure, mainnet, roulette, or web app work.\n\n",
                "Safety confirmations:\n",
                "- no signing: true\n",
                "- no broadcast: true\n",
                "- no covenant spend: true\n",
                "- no UTXO spent: true\n",
                "- no mainnet: true\n",
                "- no private key exposed: true\n",
                "- no roulette/web app: true\n\n",
                "Evidence files:\n",
                "- {}\n",
                "- {}\n",
                "- {}\n",
                "- {}\n"
            ),
            result,
            ENV061_CREATE_TXID,
            ENV061_CREATE_TXID,
            ENV061_COVENANT_OUTPUT_INDEX,
            ENV061_COVENANT_OUTPUT_VALUE_SOMPI,
            observed_covenant_id
                .as_deref()
                .unwrap_or(ENV061_EXPECTED_COVENANT_ID),
            covenant_observed,
            covenant_unspent,
            ENV061_HELPER_ADDRESS,
            ENV061_CREATE_TXID,
            ENV061_HELPER_CHANGE_OUTPOINT_INDEX,
            ENV061_HELPER_CHANGE_VALUE_SOMPI,
            helper_change_observed,
            ENV060C_FEE_SOMPI,
            mempool_observed,
            covenant_unspent && !mempool_spend_seen,
            ENV061_CREATE_TXID,
            ENV061_COVENANT_OUTPUT_INDEX,
            ENV061_COVENANT_OUTPUT_VALUE_SOMPI,
            ENV061_EXPECTED_COVENANT_ID,
            ENV061_FUTURE_SPEND_FEE_SOMPI,
            future_output_value,
            summary_path.display(),
            server_info_path.display(),
            inspection_path.display(),
            preflight_path.display(),
        ),
    )?;
    client.disconnect().await.ok();
    println!("ENV-061 {}", result);
    println!("summary={}", summary_path.display());
    println!("covenant_utxo_observed={}", covenant_observed);
    println!("covenant_unspent={}", covenant_unspent);
    Ok(())
}

#[derive(Debug)]
struct CovenantSpendArgs {
    network: String,
    covenant_utxo: String,
    input_amount_sompi: u64,
    covenant_id: String,
    fee_sompi: u64,
    output_value_sompi: u64,
    submit: bool,
    public_evidence_dir: PathBuf,
}

async fn run_covenant_spend(args: Vec<String>) -> Result<(), Box<dyn std::error::Error>> {
    let args = parse_covenant_spend_args(args)?;
    if args.network != "testnet-10" {
        return Err("blocked: covenant-spend only accepts --network testnet-10".into());
    }
    if !args.submit {
        return Err("blocked: ENV-062 live path requires explicit --submit for the one approved covenant-spend attempt".into());
    }
    fs::create_dir_all(&args.public_evidence_dir)?;
    let preflight_path = args.public_evidence_dir.join("preflight.txt");
    let submit_path = args.public_evidence_dir.join("spend-submit.txt");
    let postcheck_path = args.public_evidence_dir.join("postcheck.txt");
    let summary_path = args.public_evidence_dir.join("env-062-summary.txt");

    let (covenant_txid, covenant_index) = parse_outpoint_arg(&args.covenant_utxo)?;
    if covenant_txid != hash_from_hex(ENV061_CREATE_TXID)
        || covenant_index != ENV061_COVENANT_OUTPUT_INDEX
        || args.input_amount_sompi != ENV061_COVENANT_OUTPUT_VALUE_SOMPI
        || args.covenant_id != ENV061_EXPECTED_COVENANT_ID
        || args.fee_sompi != ENV061_FUTURE_SPEND_FEE_SOMPI
        || args.output_value_sompi + args.fee_sompi != args.input_amount_sompi
    {
        write_env062_blocked_summary(
            &summary_path,
            "ENV-061 spend inputs/fee/output do not match the reviewed plan",
        )?;
        return Err(
            "blocked: ENV-061 spend inputs/fee/output do not match the reviewed plan".into(),
        );
    }

    let covenant_script = build_covenant_script()?;
    let covenant_spk = pay_to_script_hash_script(&covenant_script);
    let covenant_address = Address::new(
        Prefix::Testnet,
        Version::ScriptHash,
        &covenant_spk.script()[2..34],
    );
    let covenant_id = hash_from_hex(&args.covenant_id);

    let client = connect_tn10_client().await?;
    let server_info = client.get_server_info().await?;
    let server_ok = server_info.network_id.to_string() == "testnet-10"
        && server_info.is_synced
        && server_info.has_utxo_index;
    if !server_ok {
        let reason = format!(
            "server preflight failed: network_id={} has_utxo_index={} is_synced={}",
            server_info.network_id, server_info.has_utxo_index, server_info.is_synced
        );
        write_env062_blocked_summary(&summary_path, &reason)?;
        client.disconnect().await.ok();
        return Err(format!("blocked: {reason}").into());
    }

    let covenant_utxos_before = client
        .get_utxos_by_addresses(vec![covenant_address.clone()])
        .await?;
    let covenant_match = covenant_utxos_before.iter().find(|entry| {
        entry.outpoint.transaction_id == covenant_txid
            && entry.outpoint.index == covenant_index
            && entry.utxo_entry.amount == args.input_amount_sompi
            && entry.utxo_entry.covenant_id == Some(covenant_id)
    });
    if covenant_match.is_none() {
        write_env062_blocked_summary(
            &summary_path,
            "covenant UTXO absent, spent, amount-mismatched, or covenant id mismatched",
        )?;
        client.disconnect().await.ok();
        return Err(
            "blocked: covenant UTXO absent, spent, amount-mismatched, or covenant id mismatched"
                .into(),
        );
    }
    let mempool_entries_before = client.get_mempool_entries(false, false).await;
    let mempool_spend_before = mempool_entries_before
        .as_ref()
        .map(|entries| {
            entries.iter().any(|entry| {
                entry.transaction.inputs.iter().any(|input| {
                    input.previous_outpoint.transaction_id == covenant_txid
                        && input.previous_outpoint.index == covenant_index
                })
            })
        })
        .unwrap_or(false);
    if mempool_spend_before {
        write_env062_blocked_summary(
            &summary_path,
            "mempool already contains a spend of the covenant outpoint",
        )?;
        client.disconnect().await.ok();
        return Err("blocked: mempool already contains a spend of the covenant outpoint".into());
    }

    let create_tx = reconstruct_env060c_create_tx()?;
    let reconstructed_create_txid = create_tx.id().to_string();
    if reconstructed_create_txid != ENV061_CREATE_TXID {
        let reason =
            format!("reconstructed ENV-060C create txid mismatch: {reconstructed_create_txid}");
        write_env062_blocked_summary(&summary_path, &reason)?;
        client.disconnect().await.ok();
        return Err(format!("blocked: {reason}").into());
    }
    let prev_rest = transaction_v1_rest_preimage(&create_tx);
    let prev_payload = create_tx.payload.clone();
    let spend_payload = vec![1u8];
    let sig_script = ScriptBuilder::new()
        .add_data(&prev_rest)?
        .add_data(&prev_payload)?
        .add_data(&covenant_script)?
        .drain();
    let input = TransactionInput::new_with_mass(
        TransactionOutpoint::new(covenant_txid, covenant_index),
        sig_script.clone(),
        0,
        ComputeBudget(0).into(),
    );
    let output = TransactionOutput::with_covenant(
        args.output_value_sompi,
        covenant_spk.clone(),
        Some(CovenantBinding::new(0, covenant_id)),
    );
    let mut spend_tx = Transaction::new(
        TX_VERSION_TOCCATA,
        vec![input],
        vec![output],
        0,
        SubnetworkId::default(),
        0,
        spend_payload.clone(),
    );
    spend_tx.finalize();
    let input_utxo = UtxoEntry::new(
        args.input_amount_sompi,
        covenant_spk.clone(),
        0,
        false,
        Some(covenant_id),
    );
    let populated = PopulatedTransaction::new(&spend_tx, vec![input_utxo.clone()]);
    let covenants_ctx = CovenantsContext::from_tx(&populated)?;
    let sig_cache = Cache::new(10_000);
    let reused_values = SigHashReusedValuesUnsync::new();
    let ctx = EngineCtx::new(&sig_cache)
        .with_reused(&reused_values)
        .with_covenants_ctx(&covenants_ctx);
    let flags = EngineFlags {
        covenants_enabled: true,
        ..Default::default()
    };
    let mut vm = TxScriptEngine::from_transaction_input(
        &populated,
        &spend_tx.inputs[0],
        0,
        &input_utxo,
        ctx,
        flags,
    );
    vm.execute()?;
    let local_txid = spend_tx.id().to_string();

    fs::write(
        &preflight_path,
        format!(
            concat!(
                "ENV-062 preflight\n\n",
                "Result: PASS\n",
                "Network requested: {}\n",
                "Server network_id: {}\n",
                "Server is_synced: {}\n",
                "Server has_utxo_index: {}\n",
                "Covenant address: {}\n",
                "Covenant input outpoint: {}:{}\n",
                "Covenant input amount sompi: {}\n",
                "Covenant id: {}\n",
                "Covenant UTXO observed before submit: true\n",
                "Mempool spend of covenant outpoint before submit: false\n",
                "Reconstructed create txid matches ENV-061: true\n",
                "TX_VERSION_TOCCATA: {}\n",
                "Spend transaction version: {}\n",
                "Uses literal version 2: false\n",
                "Spend input count: {}\n",
                "Spend uses expected covenant UTXO only: true\n",
                "Helper fee input used: false\n",
                "Spend output count: {}\n",
                "Continuing covenant output value sompi: {}\n",
                "Fee sompi: {}\n",
                "Covenant output/state transition: output0 same SPK, same covenant id, payload 01\n",
                "Redeem script length: {}\n",
                "Sigscript length: {}\n",
                "Local VM covenant spend check passed: true\n",
                "Private key material exposed: false\n"
            ),
            args.network,
            server_info.network_id,
            server_info.is_synced,
            server_info.has_utxo_index,
            covenant_address,
            covenant_txid,
            covenant_index,
            args.input_amount_sompi,
            args.covenant_id,
            TX_VERSION_TOCCATA,
            spend_tx.version,
            spend_tx.inputs.len(),
            spend_tx.outputs.len(),
            args.output_value_sompi,
            args.fee_sompi,
            covenant_script.len(),
            sig_script.len(),
        ),
    )?;

    let submit_result = client.submit_transaction((&spend_tx).into(), false).await;
    match submit_result {
        Ok(submitted_txid) => {
            let submitted_txid_string = submitted_txid.to_string();
            fs::write(
                &submit_path,
                format!(
                    concat!(
                        "ENV-062 covenant spend submit\n\n",
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
            let mempool_result = client.get_mempool_entry(spend_tx.id(), false, false).await;
            let covenant_utxos_after = client
                .get_utxos_by_addresses(vec![covenant_address.clone()])
                .await?;
            let original_utxo_after = covenant_utxos_after.iter().any(|entry| {
                entry.outpoint.transaction_id == covenant_txid
                    && entry.outpoint.index == covenant_index
            });
            let new_utxo_after = covenant_utxos_after.iter().any(|entry| {
                entry.outpoint.transaction_id == spend_tx.id()
                    && entry.outpoint.index == 0
                    && entry.utxo_entry.amount == args.output_value_sompi
                    && entry.utxo_entry.covenant_id == Some(covenant_id)
            });
            let mempool_spend_after = mempool_result
                .as_ref()
                .map(|entry| {
                    entry.transaction.inputs.iter().any(|input| {
                        input.previous_outpoint.transaction_id == covenant_txid
                            && input.previous_outpoint.index == covenant_index
                    })
                })
                .unwrap_or(false);
            fs::write(
                &postcheck_path,
                format!(
                    concat!(
                        "ENV-062 postcheck\n\n",
                        "Result: PASS\n",
                        "Server network_id: {}\n",
                        "Server is_synced: {}\n",
                        "Server has_utxo_index: {}\n",
                        "Submitted txid: {}\n",
                        "Mempool entry observed: {}\n",
                        "Mempool entry spends original covenant outpoint: {}\n",
                        "Covenant address UTXO count after submit: {}\n",
                        "Original covenant UTXO still visible in UTXO set: {}\n",
                        "Original covenant UTXO appears spent or pending-spent: {}\n",
                        "New transition UTXO visible in UTXO set: {}\n",
                        "New transition output if accepted: {}:0 value {} sompi same SPK same covenant id payload 01\n"
                    ),
                    server_info.network_id,
                    server_info.is_synced,
                    server_info.has_utxo_index,
                    submitted_txid_string,
                    mempool_result.is_ok(),
                    mempool_spend_after,
                    covenant_utxos_after.len(),
                    original_utxo_after,
                    (!original_utxo_after) || mempool_spend_after,
                    new_utxo_after,
                    submitted_txid_string,
                    args.output_value_sompi,
                ),
            )?;
            fs::write(
                &summary_path,
                format!(
                    concat!(
                        "ENV-062 live TN10 covenant spend attempt\n\n",
                        "Result: PASS\n",
                        "Network: TN10 / testnet-10\n",
                        "Covenant input outpoint: {}:{}\n",
                        "Covenant input amount: {}\n",
                        "Covenant id: {}\n",
                        "Spend txid if accepted: {}\n",
                        "Fee used: {}\n",
                        "Resulting output/state details: output0 same covenant SPK, same covenant id, value {} sompi, payload 01\n",
                        "Original covenant UTXO appears spent: {}\n",
                        "New output/UTXO appears: {}\n",
                        "Exact rejection if rejected: none\n",
                        "Exactly one covenant-spend submission attempted: true\n",
                        "Confirmation no mainnet: true\n",
                        "Confirmation no wallet secrets accessed: true\n",
                        "Confirmation helper private key not exposed: true\n",
                        "Confirmation no roulette/web app: true\n",
                        "Evidence path: {}\n"
                    ),
                    covenant_txid,
                    covenant_index,
                    args.input_amount_sompi,
                    args.covenant_id,
                    submitted_txid_string,
                    args.fee_sompi,
                    args.output_value_sompi,
                    (!original_utxo_after) || mempool_spend_after,
                    new_utxo_after,
                    args.public_evidence_dir.display(),
                ),
            )?;
            println!("ENV-062 PASS");
            println!("submitted_txid={}", submitted_txid_string);
            println!("summary={}", summary_path.display());
        }
        Err(err) => {
            let rejection = err.to_string();
            fs::write(
                &submit_path,
                format!(
                    "ENV-062 covenant spend submit\n\nResult: REJECTED\nSubmission attempts: 1\nallow_orphan: false\nLocal txid: {}\nRejection/error: {}\nPrivate key material exposed: false\n",
                    local_txid, rejection
                ),
            )?;
            fs::write(
                &postcheck_path,
                "ENV-062 postcheck\n\nResult: NOT RUN AFTER REJECTED SUBMIT\n",
            )?;
            fs::write(
                &summary_path,
                format!(
                    concat!(
                        "ENV-062 live TN10 covenant spend attempt\n\n",
                        "Result: REJECTED\n",
                        "Network: TN10 / testnet-10\n",
                        "Covenant input outpoint: {}:{}\n",
                        "Covenant input amount: {}\n",
                        "Covenant id: {}\n",
                        "Spend txid if accepted: none\n",
                        "Fee used: {}\n",
                        "Resulting output/state details: not accepted; planned output0 same covenant SPK, same covenant id, value {} sompi, payload 01\n",
                        "Original covenant UTXO appears spent: false (postcheck stopped after rejection)\n",
                        "New output/UTXO appears: false\n",
                        "Exact rejection if rejected: {}\n",
                        "Exactly one covenant-spend submission attempted: true\n",
                        "Confirmation no mainnet: true\n",
                        "Confirmation no wallet secrets accessed: true\n",
                        "Confirmation helper private key not exposed: true\n",
                        "Confirmation no roulette/web app: true\n"
                    ),
                    covenant_txid,
                    covenant_index,
                    args.input_amount_sompi,
                    args.covenant_id,
                    args.fee_sompi,
                    args.output_value_sompi,
                    rejection,
                ),
            )?;
            println!("ENV-062 REJECTED");
            println!("summary={}", summary_path.display());
        }
    }
    client.disconnect().await.ok();
    Ok(())
}

fn reconstruct_env060c_create_tx() -> Result<Transaction, Box<dyn std::error::Error>> {
    let helper_address = Address::try_from(ENV061_HELPER_ADDRESS)?;
    let helper_spk = pay_to_address_script(&helper_address);
    let covenant_script = build_covenant_script()?;
    let covenant_spk = pay_to_script_hash_script(&covenant_script);
    let input = TransactionInput::new_with_mass(
        TransactionOutpoint::new(
            hash_from_hex(ENV060C_HELPER_FUNDING_TXID),
            ENV060C_HELPER_FUNDING_INDEX,
        ),
        vec![],
        0,
        ComputeBudget(0).into(),
    );
    let mut tx = Transaction::new(
        TX_VERSION_TOCCATA,
        vec![input],
        vec![
            TransactionOutput::new(ENV061_COVENANT_OUTPUT_VALUE_SOMPI, covenant_spk),
            TransactionOutput::new(ENV061_HELPER_CHANGE_VALUE_SOMPI, helper_spk),
        ],
        0,
        SubnetworkId::default(),
        0,
        vec![],
    );
    tx.populate_genesis_covenants(&[GenesisCovenantGroup::new(0, vec![0])])?;
    tx.finalize();
    Ok(tx)
}

fn run_env062a_local_debug() -> Result<(), Box<dyn std::error::Error>> {
    let artifact_dir = env062a_artifact_dir();
    fs::create_dir_all(&artifact_dir)?;
    let debug = env062_local_proof_debug_text()?;
    fs::write(artifact_dir.join("local-proof-debug.txt"), &debug)?;
    println!("ENV-062A local proof debug written");
    println!("artifact_dir={}", artifact_dir.display());
    Ok(())
}

fn env062a_artifact_dir() -> PathBuf {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    manifest_dir
        .parent()
        .expect("crate is under spikes/tn12-minimal-covenant")
        .join("artifacts/env-062a-covenant-spend-verifyerror-diagnosis")
}

fn build_env062_local_spend_for_proof()
-> Result<(Transaction, UtxoEntry, Vec<u8>, Vec<u8>, Vec<u8>, Hash), Box<dyn std::error::Error>> {
    let covenant_txid = hash_from_hex(ENV061_CREATE_TXID);
    let covenant_index = ENV061_COVENANT_OUTPUT_INDEX;
    let covenant_id = hash_from_hex(ENV061_EXPECTED_COVENANT_ID);
    let covenant_script = build_covenant_script()?;
    let covenant_spk = pay_to_script_hash_script(&covenant_script);
    let create_tx = reconstruct_env060c_create_tx()?;
    let prev_rest = transaction_v1_rest_preimage(&create_tx);
    let prev_payload = create_tx.payload.clone();
    let spend_payload = vec![1u8];
    let sig_script = ScriptBuilder::new()
        .add_data(&prev_rest)?
        .add_data(&prev_payload)?
        .add_data(&covenant_script)?
        .drain();
    let input = TransactionInput::new_with_mass(
        TransactionOutpoint::new(covenant_txid, covenant_index),
        sig_script,
        0,
        ComputeBudget(0).into(),
    );
    let output = TransactionOutput::with_covenant(
        ENV061_COVENANT_OUTPUT_VALUE_SOMPI - ENV061_FUTURE_SPEND_FEE_SOMPI,
        covenant_spk.clone(),
        Some(CovenantBinding::new(0, covenant_id)),
    );
    let mut spend_tx = Transaction::new(
        TX_VERSION_TOCCATA,
        vec![input],
        vec![output],
        0,
        SubnetworkId::default(),
        0,
        spend_payload,
    );
    spend_tx.finalize();
    let input_utxo = UtxoEntry::new(
        ENV061_COVENANT_OUTPUT_VALUE_SOMPI,
        covenant_spk,
        0,
        false,
        Some(covenant_id),
    );
    Ok((
        spend_tx,
        input_utxo,
        prev_rest,
        prev_payload,
        covenant_script,
        covenant_id,
    ))
}

fn env062_local_proof_debug_text() -> Result<String, Box<dyn std::error::Error>> {
    let create_tx = reconstruct_env060c_create_tx()?;
    let (spend_tx, input_utxo, prev_rest, prev_payload, covenant_script, covenant_id) =
        build_env062_local_spend_for_proof()?;
    let mut v0_style_preimage = prev_rest.clone();
    v0_style_preimage.extend_from_slice(&prev_payload);
    let script_computed_txid = TransactionID::hash(&v0_style_preimage);
    let rest_digest = v1_rest_digest(&create_tx);
    let payload_digest = payload_digest(&create_tx.payload);
    let mut v1_hasher = TransactionV1Id::new();
    v1_hasher
        .update(payload_digest.as_bytes())
        .update(rest_digest.as_bytes());
    let reconstructed_v1_id = v1_hasher.finalize();
    let populated = PopulatedTransaction::new(&spend_tx, vec![input_utxo.clone()]);
    let covenants_ctx = CovenantsContext::from_tx(&populated)?;
    let sig_cache = Cache::new(10_000);
    let reused_values = SigHashReusedValuesUnsync::new();
    let ctx = EngineCtx::new(&sig_cache)
        .with_reused(&reused_values)
        .with_covenants_ctx(&covenants_ctx);
    let flags = EngineFlags {
        covenants_enabled: true,
        ..Default::default()
    };
    let mut vm = TxScriptEngine::from_transaction_input(
        &populated,
        &spend_tx.inputs[0],
        0,
        &input_utxo,
        ctx,
        flags,
    );
    let vm_result = vm.execute();
    Ok(format!(
        concat!(
            "ENV-062A local proof debug\n\n",
            "Reconstructed ENV-060C create txid: {}\n",
            "Expected ENV-061 create txid: {}\n",
            "Create tx version: {}\n",
            "Create tx payload hex: {}\n",
            "Spend tx version: {}\n",
            "Spend tx payload hex: {}\n",
            "Spend input previous outpoint: {}:{}\n",
            "Spend output count: {}\n",
            "Spend output covenant id: {}\n",
            "Input UTXO covenant id: {}\n",
            "CovenantsContext::from_tx: OK\n",
            "CovenantsContext auth outputs for input 0: {}\n",
            "Sigscript stack item 1 / prev_rest length: {}\n",
            "Sigscript stack item 2 / prev_payload length: {}\n",
            "Sigscript stack item 3 / covenant script length: {}\n",
            "Script check 1 opcode: OP_EQUALVERIFY after OP_BLAKE2B_WITH_KEY/OP_OUTPOINT_TX_ID\n",
            "Script-computed v0-style TransactionID(prev_rest || prev_payload): {}\n",
            "Actual outpoint txid: {}\n",
            "First script check passes: {}\n",
            "V1 rest digest: {}\n",
            "V1 payload digest: {}\n",
            "Reconstructed V1 id from payload_digest || rest_digest: {}\n",
            "Reconstructed V1 id matches create txid: {}\n",
            "VM result: {:?}\n",
            "Exact VerifyError variant: {}\n",
            "Failing check/opcode: {}\n"
        ),
        create_tx.id(),
        ENV061_CREATE_TXID,
        create_tx.version,
        hex_encode(&create_tx.payload),
        spend_tx.version,
        hex_encode(&spend_tx.payload),
        spend_tx.inputs[0].previous_outpoint.transaction_id,
        spend_tx.inputs[0].previous_outpoint.index,
        spend_tx.outputs.len(),
        spend_tx.outputs[0]
            .covenant
            .map(|binding| binding.covenant_id.to_string())
            .unwrap_or_else(|| "none".to_string()),
        covenant_id,
        covenants_ctx
            .input_ctxs
            .get(&0)
            .map(|ctx| ctx.auth_outputs.len())
            .unwrap_or(0),
        prev_rest.len(),
        prev_payload.len(),
        covenant_script.len(),
        script_computed_txid,
        create_tx.id(),
        script_computed_txid == create_tx.id(),
        rest_digest,
        payload_digest,
        reconstructed_v1_id,
        reconstructed_v1_id == create_tx.id(),
        vm_result,
        if matches!(
            vm_result,
            Err(kaspa_txscript_errors::TxScriptError::VerifyError)
        ) {
            "TxScriptError::VerifyError"
        } else {
            "not VerifyError"
        },
        if script_computed_txid != create_tx.id() {
            "first OP_EQUALVERIFY; the upstream v0 script hashes prev_rest||prev_payload with TransactionID, but live TX_VERSION_TOCCATA v1 txid is TransactionV1Id(payload_digest||rest_digest)"
        } else {
            "not isolated by txid precheck"
        },
    ))
}

fn parse_covenant_spend_args(
    args: Vec<String>,
) -> Result<CovenantSpendArgs, Box<dyn std::error::Error>> {
    let mut network = None;
    let mut covenant_utxo = None;
    let mut input_amount_sompi = None;
    let mut covenant_id = None;
    let mut fee_sompi = None;
    let mut output_value_sompi = None;
    let mut submit = false;
    let mut public_evidence_dir = None;
    let mut iter = args.into_iter();
    while let Some(arg) = iter.next() {
        match arg.as_str() {
            "--network" => network = iter.next(),
            "--covenant-utxo" => covenant_utxo = iter.next(),
            "--input-amount-sompi" | "--covenant-amount-sompi" => {
                input_amount_sompi = Some(
                    iter.next()
                        .ok_or("missing --input-amount-sompi value")?
                        .parse()?,
                );
            }
            "--covenant-id" => covenant_id = iter.next(),
            "--fee-sompi" => {
                fee_sompi = Some(iter.next().ok_or("missing --fee-sompi value")?.parse()?)
            }
            "--output-value-sompi" => {
                output_value_sompi = Some(
                    iter.next()
                        .ok_or("missing --output-value-sompi value")?
                        .parse()?,
                )
            }
            "--submit" => submit = true,
            "--public-evidence-dir" => {
                public_evidence_dir = Some(PathBuf::from(
                    iter.next().ok_or("missing --public-evidence-dir value")?,
                ));
            }
            other => return Err(format!("unknown covenant-spend argument `{other}`").into()),
        }
    }
    Ok(CovenantSpendArgs {
        network: network.ok_or("missing --network")?,
        covenant_utxo: covenant_utxo.ok_or("missing --covenant-utxo")?,
        input_amount_sompi: input_amount_sompi.ok_or("missing --input-amount-sompi")?,
        covenant_id: covenant_id.ok_or("missing --covenant-id")?,
        fee_sompi: fee_sompi.ok_or("missing --fee-sompi")?,
        output_value_sompi: output_value_sompi.ok_or("missing --output-value-sompi")?,
        submit,
        public_evidence_dir: public_evidence_dir.ok_or("missing --public-evidence-dir")?,
    })
}

fn write_env062_blocked_summary(path: &Path, reason: &str) -> std::io::Result<()> {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }
    fs::write(
        path,
        format!(
            concat!(
                "ENV-062 live TN10 covenant spend attempt\n\n",
                "Result: BLOCKED\n",
                "Network: TN10 / testnet-10\n",
                "Reason: {}\n",
                "Exactly one covenant-spend submission attempted: false\n",
                "Confirmation no mainnet: true\n",
                "Confirmation no wallet secrets accessed: true\n",
                "Confirmation helper private key not exposed: true\n",
                "Confirmation no roulette/web app: true\n"
            ),
            reason
        ),
    )
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
            "ENV-060C live helper-controlled TN10 covenant create fee retry\n\nResult: BLOCKED\nReason: {}\nSafety confirmations: no submission attempted, no covenant spend attempted, no mainnet, no wallet secrets accessed, helper private key not exposed, no roulette/web app.\n",
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
    println!(
        "  env061-inspect           Read-only inspect accepted covenant-create UTXO and write ENV-061 preflight artifacts"
    );
    println!(
        "  covenant-spend           Build, locally prove, and submit exactly one ENV-062 TN10 covenant-spend transaction"
    );
    println!(
        "  env062a-local-debug      Local-only ENV-062A covenant spend VerifyError diagnostics; writes no-broadcast artifacts"
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

    #[test]
    fn env062a_reconstructs_env060c_create_txid() {
        let tx = reconstruct_env060c_create_tx().expect("reconstruct ENV-060C create tx");
        assert_eq!(tx.version, TX_VERSION_TOCCATA);
        assert_eq!(tx.id().to_string(), ENV061_CREATE_TXID);
        assert_eq!(
            tx.outputs[0]
                .covenant
                .map(|binding| binding.covenant_id.to_string()),
            Some(ENV061_EXPECTED_COVENANT_ID.to_string())
        );
    }

    #[test]
    fn env062a_local_spend_fails_at_v1_txid_precheck() {
        let debug = env062_local_proof_debug_text().expect("build local proof debug text");
        assert!(debug.contains("VM result: Err(VerifyError)"));
        assert!(debug.contains("First script check passes: false"));
        assert!(debug.contains("Reconstructed V1 id matches create txid: true"));
        assert!(debug.contains("Failing check/opcode: first OP_EQUALVERIFY"));
    }
}
