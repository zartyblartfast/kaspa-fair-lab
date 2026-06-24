use std::{env, fs, path::PathBuf};

use kaspa_grpc_client::GrpcClient;
use kaspa_rpc_core::{
    api::rpc::RpcApi,
    notify::mode::NotificationMode,
    GetBlockDagInfoRequest, GetServerInfoRequest, GetSyncStatusRequest,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut args = env::args().skip(1);
    let target_url = args
        .next()
        .unwrap_or_else(|| "grpc://127.0.0.1:16210".to_string());
    let output_dir = PathBuf::from(args.next().unwrap_or_else(|| ".".to_string()));

    fs::create_dir_all(&output_dir)?;

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

    let server_info = client
        .get_server_info_call(None, GetServerInfoRequest {})
        .await?;
    fs::write(
        output_dir.join("env-042-get-server-info-after-30min.txt"),
        format!(
            "target_url={}\nresponse_json={}\n",
            target_url,
            serde_json::to_string_pretty(&server_info)?
        ),
    )?;

    let blockdag_info = client
        .get_block_dag_info_call(None, GetBlockDagInfoRequest {})
        .await?;
    fs::write(
        output_dir.join("env-042-get-blockdag-info-after-30min.txt"),
        format!(
            "target_url={}\nresponse_json={}\n",
            target_url,
            serde_json::to_string_pretty(&blockdag_info)?
        ),
    )?;

    let sync_status = client
        .get_sync_status_call(None, GetSyncStatusRequest {})
        .await?;
    fs::write(
        output_dir.join("env-042-get-sync-status-after-30min.txt"),
        format!(
            "target_url={}\nresponse_json={}\n",
            target_url,
            serde_json::to_string_pretty(&sync_status)?
        ),
    )?;

    client.disconnect().await?;
    Ok(())
}
