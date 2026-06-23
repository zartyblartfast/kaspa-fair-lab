use std::fs;
use std::path::PathBuf;

use kaspa_consensus_core::tx::{
    ScriptPublicKey, Transaction, TransactionId, TransactionInput, TransactionOutpoint,
    TransactionOutput,
};

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
    let artifact_path = artifact_dir.join("local-no-broadcast-transaction-summary.txt");

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
        tx,
    );

    fs::write(&artifact_path, &summary).expect("write artifact summary");

    println!("artifact_path={}", artifact_path.display());
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
