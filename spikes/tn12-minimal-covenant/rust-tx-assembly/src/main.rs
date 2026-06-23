use std::fs;
use std::path::PathBuf;

use borsh::to_vec;
use kaspa_consensus_core::tx::{
    ScriptPublicKey, Transaction, TransactionId, TransactionInput, TransactionOutpoint,
    TransactionOutput,
};
use kaspa_rpc_core::{RpcTransaction, SubmitTransactionRequest};
use workflow_serializer::serializer::{Deserializer, Serializer};

fn hex_encode(bytes: &[u8]) -> String {
    const HEX: &[u8; 16] = b"0123456789abcdef";
    let mut encoded = String::with_capacity(bytes.len() * 2);
    for byte in bytes {
        encoded.push(HEX[(byte >> 4) as usize] as char);
        encoded.push(HEX[(byte & 0x0f) as usize] as char);
    }
    encoded
}

fn hex_decode(hex: &str) -> Result<Vec<u8>, String> {
    let trimmed = hex.trim();
    if trimmed.len() % 2 != 0 {
        return Err("hex length must be even".to_string());
    }

    let mut bytes = Vec::with_capacity(trimmed.len() / 2);
    let chars: Vec<char> = trimmed.chars().collect();
    for i in (0..chars.len()).step_by(2) {
        let hi = chars[i]
            .to_digit(16)
            .ok_or_else(|| format!("invalid hex at position {}", i))?;
        let lo = chars[i + 1]
            .to_digit(16)
            .ok_or_else(|| format!("invalid hex at position {}", i + 1))?;
        bytes.push(((hi << 4) | lo) as u8);
    }
    Ok(bytes)
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
    let rpc_tx: RpcTransaction = (&tx).into();
    let rpc_output0 = &rpc_tx.outputs[0];
    let rpc_script_bytes = rpc_output0.script_public_key.script();
    let rpc_script_bytes_present = !rpc_script_bytes.is_empty();
    let allow_orphan = false;
    let submit_transaction_request = SubmitTransactionRequest {
        transaction: rpc_tx.clone(),
        allow_orphan,
    };
    let mut rpc_serialized_bytes = Vec::new();
    rpc_tx
        .serialize(&mut rpc_serialized_bytes)
        .expect("serialize rpc transaction with rpc serializer");
    let rpc_serialized_hex = hex_encode(&rpc_serialized_bytes);
    let mut submit_request_serialized_bytes = Vec::new();
    submit_transaction_request
        .serialize(&mut submit_request_serialized_bytes)
        .expect("serialize submit transaction request with rpc serializer");
    let submit_request_serialized_hex = hex_encode(&submit_request_serialized_bytes);

    let artifact_dir = PathBuf::from("artifacts");
    fs::create_dir_all(&artifact_dir).expect("create artifacts directory");
    let summary_artifact_path = artifact_dir.join("local-no-broadcast-transaction-summary.txt");
    let serialization_artifact_path = artifact_dir.join("local-no-broadcast-transaction.hex");
    let rpc_summary_artifact_path =
        artifact_dir.join("local-no-broadcast-rpc-transaction-summary.txt");
    let submit_request_summary_artifact_path =
        artifact_dir.join("local-no-broadcast-submit-transaction-request-summary.txt");
    let rpc_serializer_artifact_path =
        artifact_dir.join("local-no-broadcast-rpc-transaction-rpc-serializer.hex");
    let submit_request_serializer_artifact_path =
        artifact_dir.join("local-no-broadcast-submit-transaction-request-rpc-serializer.hex");
    let rpc_roundtrip_summary_artifact_path =
        artifact_dir.join("local-no-broadcast-rpc-roundtrip-summary.txt");
    let serialized_bytes = to_vec(&tx).expect("serialize transaction with borsh");
    let serialized_hex = hex_encode(&serialized_bytes);
    let serialization_type =
        "borsh binary hex (deterministic local artifact; consensus-wire equivalence unverified)";
    let consensus_serialization_conclusion = "unresolved: targeted rusty-kaspa source audit found Borsh + serde/RPC object serializers, but no explicit consensus/raw wire transaction serialization API";

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
            "consensus_serialization_conclusion: {}\n",
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
        consensus_serialization_conclusion,
        serialized_bytes.len(),
        serialization_artifact_path.display(),
        tx,
    );

    let rpc_summary = format!(
        concat!(
            "rpc transaction summary\n",
            "conversion_path: official From<&Transaction> for RpcTransaction\n",
            "source_transaction_id: {}\n",
            "rpc_version: {}\n",
            "rpc_inputs: {}\n",
            "rpc_outputs: {}\n",
            "rpc_output0_value: {}\n",
            "rpc_output0_script_bytes_len: {}\n",
            "rpc_output0_script_bytes_present: {}\n",
            "source_output0_covenant_binding_present: {}\n",
            "rpc_verbose_data_present: {}\n",
            "rpc_debug: {:#?}\n"
        ),
        tx_id,
        rpc_tx.version,
        rpc_tx.inputs.len(),
        rpc_tx.outputs.len(),
        rpc_output0.value,
        rpc_script_bytes.len(),
        rpc_script_bytes_present,
        covenant_binding_present,
        rpc_tx.verbose_data.is_some(),
        rpc_tx,
    );

    let submit_request_summary = format!(
        concat!(
            "submit transaction request summary\n",
            "request_type: SubmitTransactionRequest\n",
            "allow_orphan: {}\n",
            "rpc_transaction_version: {}\n",
            "rpc_input_count: {}\n",
            "rpc_output_count: {}\n",
            "rpc_output0_value: {}\n",
            "rpc_source_output0_covenant_binding_present: {}\n",
            "rpc_call_made: false\n",
            "broadcast_attempted: false\n",
            "request_debug: {:#?}\n"
        ),
        submit_transaction_request.allow_orphan,
        submit_transaction_request.transaction.version,
        submit_transaction_request.transaction.inputs.len(),
        submit_transaction_request.transaction.outputs.len(),
        submit_transaction_request.transaction.outputs[0].value,
        covenant_binding_present,
        submit_transaction_request,
    );

    let rpc_serializer_type = "rusty-kaspa rpc Serializer trait binary encoded as lowercase hex";

    fs::write(&summary_artifact_path, &summary).expect("write artifact summary");
    fs::write(
        &serialization_artifact_path,
        format!("{}\n", serialized_hex),
    )
    .expect("write serialization artifact");
    fs::write(&rpc_summary_artifact_path, &rpc_summary).expect("write rpc artifact summary");
    fs::write(
        &submit_request_summary_artifact_path,
        &submit_request_summary,
    )
    .expect("write submit request artifact summary");
    fs::write(
        &rpc_serializer_artifact_path,
        format!("{}\n", rpc_serialized_hex),
    )
    .expect("write rpc serializer artifact");
    fs::write(
        &submit_request_serializer_artifact_path,
        format!("{}\n", submit_request_serialized_hex),
    )
    .expect("write submit request serializer artifact");

    let rpc_serializer_artifact_hex =
        fs::read_to_string(&rpc_serializer_artifact_path).expect("read rpc serializer artifact");
    let rpc_serializer_artifact_bytes =
        hex_decode(&rpc_serializer_artifact_hex).expect("decode rpc serializer artifact hex");
    let mut rpc_reader = std::io::Cursor::new(rpc_serializer_artifact_bytes);
    let roundtrip_rpc_tx = RpcTransaction::deserialize(&mut rpc_reader)
        .expect("deserialize rpc transaction artifact with rpc serializer");

    let submit_request_serializer_artifact_hex =
        fs::read_to_string(&submit_request_serializer_artifact_path)
            .expect("read submit request serializer artifact");
    let submit_request_serializer_artifact_bytes =
        hex_decode(&submit_request_serializer_artifact_hex)
            .expect("decode submit request serializer artifact hex");
    let mut submit_request_reader = std::io::Cursor::new(submit_request_serializer_artifact_bytes);
    let roundtrip_submit_transaction_request =
        SubmitTransactionRequest::deserialize(&mut submit_request_reader)
            .expect("deserialize submit request artifact with rpc serializer");

    let roundtrip_rpc_output0 = &roundtrip_rpc_tx.outputs[0];
    let roundtrip_rpc_covenant_binding_present = roundtrip_rpc_output0.covenant.is_some();
    let rpc_transaction_roundtrip_pass = roundtrip_rpc_tx.version == rpc_tx.version
        && roundtrip_rpc_tx.inputs.len() == rpc_tx.inputs.len()
        && roundtrip_rpc_tx.outputs.len() == rpc_tx.outputs.len()
        && roundtrip_rpc_output0.value == rpc_output0.value
        && roundtrip_rpc_covenant_binding_present == covenant_binding_present
        && roundtrip_rpc_tx.verbose_data.is_some() == rpc_tx.verbose_data.is_some();

    let roundtrip_submit_request_output0 =
        &roundtrip_submit_transaction_request.transaction.outputs[0];
    let roundtrip_submit_request_covenant_binding_present =
        roundtrip_submit_request_output0.covenant.is_some();
    let submit_request_roundtrip_pass = roundtrip_submit_transaction_request.allow_orphan
        == allow_orphan
        && roundtrip_submit_transaction_request.transaction.version == rpc_tx.version
        && roundtrip_submit_transaction_request
            .transaction
            .inputs
            .len()
            == rpc_tx.inputs.len()
        && roundtrip_submit_transaction_request
            .transaction
            .outputs
            .len()
            == rpc_tx.outputs.len()
        && roundtrip_submit_request_output0.value == rpc_output0.value
        && roundtrip_submit_request_covenant_binding_present == covenant_binding_present
        && roundtrip_submit_transaction_request
            .transaction
            .verbose_data
            .is_some()
            == rpc_tx.verbose_data.is_some();

    let roundtrip_summary = format!(
        concat!(
            "rpc serializer roundtrip summary\n",
            "rpc_transaction_roundtrip: {}\n",
            "submit_request_roundtrip: {}\n",
            "rpc_transaction_version_original: {}\n",
            "rpc_transaction_version_roundtrip: {}\n",
            "rpc_input_count_original: {}\n",
            "rpc_input_count_roundtrip: {}\n",
            "rpc_output_count_original: {}\n",
            "rpc_output_count_roundtrip: {}\n",
            "rpc_output0_value_original: {}\n",
            "rpc_output0_value_roundtrip: {}\n",
            "allow_orphan_original: {}\n",
            "allow_orphan_roundtrip: {}\n",
            "covenant_binding_present_original: {}\n",
            "covenant_binding_present_roundtrip_rpc: {}\n",
            "covenant_binding_present_roundtrip_submit_request: {}\n",
            "rpc_verbose_data_present_original: {}\n",
            "rpc_verbose_data_present_roundtrip: {}\n",
            "submit_request_verbose_data_present_roundtrip: {}\n",
            "no_rpc_client_called: true\n",
            "signed: false\n",
            "broadcast: false\n"
        ),
        if rpc_transaction_roundtrip_pass {
            "pass"
        } else {
            "fail"
        },
        if submit_request_roundtrip_pass {
            "pass"
        } else {
            "fail"
        },
        rpc_tx.version,
        roundtrip_rpc_tx.version,
        rpc_tx.inputs.len(),
        roundtrip_rpc_tx.inputs.len(),
        rpc_tx.outputs.len(),
        roundtrip_rpc_tx.outputs.len(),
        rpc_output0.value,
        roundtrip_rpc_output0.value,
        allow_orphan,
        roundtrip_submit_transaction_request.allow_orphan,
        covenant_binding_present,
        roundtrip_rpc_covenant_binding_present,
        roundtrip_submit_request_covenant_binding_present,
        rpc_tx.verbose_data.is_some(),
        roundtrip_rpc_tx.verbose_data.is_some(),
        roundtrip_submit_transaction_request
            .transaction
            .verbose_data
            .is_some(),
    );
    fs::write(&rpc_roundtrip_summary_artifact_path, &roundtrip_summary)
        .expect("write rpc roundtrip summary artifact");

    println!("summary_artifact_path={}", summary_artifact_path.display());
    println!(
        "serialization_artifact_path={}",
        serialization_artifact_path.display()
    );
    println!(
        "rpc_summary_artifact_path={}",
        rpc_summary_artifact_path.display()
    );
    println!(
        "submit_request_summary_artifact_path={}",
        submit_request_summary_artifact_path.display()
    );
    println!(
        "rpc_serializer_artifact_path={}",
        rpc_serializer_artifact_path.display()
    );
    println!(
        "submit_request_serializer_artifact_path={}",
        submit_request_serializer_artifact_path.display()
    );
    println!(
        "rpc_roundtrip_summary_artifact_path={}",
        rpc_roundtrip_summary_artifact_path.display()
    );
    println!("serialization_type={}", serialization_type);
    println!("rpc_serializer_type={}", rpc_serializer_type);
    println!(
        "consensus_serialization_conclusion={}",
        consensus_serialization_conclusion
    );
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
    println!("rpc_transaction_conversion=success");
    println!("submit_transaction_request_construction=success");
    println!(
        "rpc_transaction_roundtrip={}",
        if rpc_transaction_roundtrip_pass {
            "pass"
        } else {
            "fail"
        }
    );
    println!(
        "submit_request_roundtrip={}",
        if submit_request_roundtrip_pass {
            "pass"
        } else {
            "fail"
        }
    );
    println!("submit_transaction_request_allow_orphan={}", allow_orphan);
    println!("submit_transaction_request_rpc_call_made=false");
    println!("submit_transaction_request_broadcast_attempted=false");
    println!("no_rpc_client_called=true");
    println!("signed=false");
    println!("broadcast=false");
    println!("rpc_serializer_bytes_len={}", rpc_serialized_bytes.len());
    println!(
        "submit_request_serializer_bytes_len={}",
        submit_request_serialized_bytes.len()
    );
    println!("rpc_transaction_version={}", rpc_tx.version);
    println!("rpc_input_count={}", rpc_tx.inputs.len());
    println!("rpc_output_count={}", rpc_tx.outputs.len());
    println!("rpc_output0_value={}", rpc_output0.value);
    println!(
        "rpc_output0_script_bytes_present={}",
        rpc_script_bytes_present
    );
    println!(
        "rpc_source_output0_covenant_binding_present={}",
        covenant_binding_present
    );
    println!("rpc_verbose_data_present={}", rpc_tx.verbose_data.is_some());
    println!("transaction_debug={:#?}", tx);
    println!("rpc_transaction_debug={:#?}", rpc_tx);
}
