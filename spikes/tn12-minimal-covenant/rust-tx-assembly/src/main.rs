use std::fs;
use std::path::PathBuf;

use borsh::to_vec;
use kaspa_consensus_core::tx::{
    ScriptPublicKey, Transaction, TransactionId, TransactionInput, TransactionOutpoint,
    TransactionOutput,
};

fn hex_encode(bytes: &[u8]) -> String {
    const HEX: &[u8; 16] = b"0123456789abcdef";
    let mut encoded = String::with_capacity(bytes.len() * 2);
    for byte in bytes {
        encoded.push(HEX[(byte >> 4) as usize] as char);
        encoded.push(HEX[(byte & 0x0f) as usize] as char);
    }
    encoded
}

fn main() {
    let input = TransactionInput::new(
        TransactionOutpoint {
            transaction_id: TransactionId::from_bytes([0x11; 32]),
            index: 0,
        },
        vec![0xaa, 0xbb],
        0,
        0,
    );

    let output = TransactionOutput {
        value: 1_500,
        script_public_key: ScriptPublicKey::new(0, vec![0x51].into()),
        covenant: None,
    };

    let tx = Transaction::new(
        2,
        vec![input],
        vec![output],
        0,
        Default::default(),
        0,
        vec![],
    );
    let tx_id = tx.id();
    let output0 = &tx.outputs[0];
    let script_bytes = output0.script_public_key.script();
    let script_bytes_present = !script_bytes.is_empty();
    let covenant_binding_present = output0.covenant.is_some();

    let artifact_dir = PathBuf::from("artifacts");
    fs::create_dir_all(&artifact_dir).expect("create artifacts directory");
    let summary_artifact_path = artifact_dir.join("local-no-broadcast-transaction-summary.txt");
    let serialization_artifact_path = artifact_dir.join("local-no-broadcast-transaction.hex");
    let serialized_bytes = to_vec(&tx).expect("serialize transaction with borsh");
    let serialized_hex = hex_encode(&serialized_bytes);
    let serialization_type =
        "borsh binary hex (deterministic local artifact; consensus-wire equivalence unverified)";

    let summary = format!(
        concat!(
            "transaction summary\n",
            "version: {}\n",
            "inputs: {}\n",
            "outputs: {}\n",
            "output0_value: {}\n",
            "output0_script_bytes_len: {}\n",
            "output0_script_bytes_present: {}\n",
            "output0_covenant_binding_present: {}\n",
            "transaction_id: {}\n",
            "serialization_type: {}\n",
            "serialization_bytes_len: {}\n",
            "serialization_hex_artifact: {}\n",
            "debug: {:#?}\n"
        ),
        tx.version,
        tx.inputs.len(),
        tx.outputs.len(),
        output0.value,
        script_bytes.len(),
        script_bytes_present,
        covenant_binding_present,
        tx_id,
        serialization_type,
        serialized_bytes.len(),
        serialization_artifact_path.display(),
        tx,
    );

    fs::write(&summary_artifact_path, &summary).expect("write artifact summary");
    fs::write(
        &serialization_artifact_path,
        format!("{}\n", serialized_hex),
    )
    .expect("write serialization artifact");

    println!("summary_artifact_path={}", summary_artifact_path.display());
    println!(
        "serialization_artifact_path={}",
        serialization_artifact_path.display()
    );
    println!("serialization_type={}", serialization_type);
    println!("serialization_bytes_len={}", serialized_bytes.len());
    println!("transaction_version={}", tx.version);
    println!("input_count={}", tx.inputs.len());
    println!("output_count={}", tx.outputs.len());
    println!("output0_value={}", output0.value);
    println!("output0_script_bytes_present={}", script_bytes_present);
    println!(
        "output0_covenant_binding_present={}",
        covenant_binding_present
    );
    println!("transaction_id={}", tx_id);
    println!("transaction_debug={:#?}", tx);
}
