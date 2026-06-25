use std::fs;
use std::path::{Path, PathBuf};

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

const SOURCE_TAG: &str = "tn10-toc3";
const SOURCE_PATH: &str =
    "/root/kaspa-fair-lab/tools/rusty-kaspa-source/env058-20260625T230041Z/rusty-kaspa-tn10-toc3";
const DUMMY_REFERENCE_OUTPOINT: &str =
    "c9b4532f217d66987997e972963ec5dbfa5a9e7bf18f3e38910763274fb05135:0";
const DUMMY_REFERENCE_AMOUNT_SOMPI: u64 = 100_000_000;

fn main() -> ScriptBuilderResult<()> {
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
