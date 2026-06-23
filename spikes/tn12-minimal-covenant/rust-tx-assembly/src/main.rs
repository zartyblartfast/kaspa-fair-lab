use kaspa_consensus_core::tx::{Transaction, TransactionInput, TransactionOutpoint, TransactionOutput, ScriptPublicKey, TransactionId};

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

    let tx = Transaction::new(2, vec![input], vec![output], 0, Default::default(), 0, vec![]);

    println!("planned fields: version={} inputs={} outputs={}", tx.version, tx.inputs.len(), tx.outputs.len());
    println!("Planned tx debug: {:#?}", tx);
}
