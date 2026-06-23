use std::{env, fs, path::PathBuf};

use kaspa_grpc_client::GrpcClient;
use kaspa_rpc_core::{
    api::rpc::RpcApi,
    notify::mode::NotificationMode,
    GetCurrentNetworkRequest,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut args = env::args().skip(1);
    let target_url = args
        .next()
        .unwrap_or_else(|| "grpc://127.0.0.1:16210".to_string());
    let artifact_path = PathBuf::from(
        args.next().unwrap_or_else(|| {
            "spikes/tn12-minimal-covenant/artifacts/env-040-get-current-network.txt".to_string()
        }),
    );

    let client = GrpcClient::connect_with_args(
        NotificationMode::Direct,
        target_url.clone(),
        None,
        false,
        None,
        false,
        Some(500_000),
        Default::default(),
    )
    .await?;

    let response = client
        .get_current_network_call(None, GetCurrentNetworkRequest {})
        .await?;

    let response_json = serde_json::to_string_pretty(&response)?;
    fs::write(
        &artifact_path,
        format!("target_url={}\nresponse_json={}\n", target_url, response_json),
    )?;

    client.disconnect().await?;
    Ok(())
}
