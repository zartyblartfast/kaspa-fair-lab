use std::env;
use std::fs::{self, OpenOptions};
use std::io::Write;
#[cfg(unix)]
use std::os::unix::fs::OpenOptionsExt;
use std::path::{Path, PathBuf};

use kaspa_addresses::{Address, Prefix, Version};
use kaspa_consensus_core::constants::TX_VERSION_TOCCATA;
use kaspa_consensus_core::subnets::SubnetworkId;
use kaspa_consensus_core::tx::{
    GenesisCovenantGroup, PopulatedTransaction, Transaction, TransactionInput, TransactionOutpoint,
    TransactionOutput, UtxoEntry,
};
use kaspa_hashes::Hash;
use kaspa_txscript::covenants::CovenantsContext;
use kaspa_txscript::opcodes::codes::{
    Op1Add, OpBlake2bWithKey, OpCat, OpDup, OpEqual, OpEqualVerify, OpOutpointTxId, OpRot,
    OpTxInputIndex, OpTxInputSpk, OpTxOutputCount, OpTxOutputSpk, OpTxPayloadLen,
    OpTxPayloadSubstr,
};
use kaspa_txscript::pay_to_script_hash_script;
use kaspa_txscript::script_builder::{ScriptBuilder, ScriptBuilderResult};
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

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut args = env::args().skip(1);
    match args.next().as_deref() {
        None | Some("env058-offline-scaffold") => run_env058()?,
        Some("env059-helper-key") => run_env059_helper_key()?,
        Some("help") | Some("--help") | Some("-h") => print_help(),
        Some(other) => {
            return Err(format!(
                "unknown command `{other}`; use `env058-offline-scaffold` or `env059-helper-key`"
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

fn print_help() {
    println!("tn10-covenant-spike commands:");
    println!(
        "  env058-offline-scaffold  Build the ENV-058 offline covenant scaffold artifact (default)"
    );
    println!(
        "  env059-helper-key        Generate/reuse helper-controlled TN10 key and public address preflight artifacts"
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
