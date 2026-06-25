use kaspa_addresses::{Address, Prefix, Version};
use kaspa_consensus_core::network::{NetworkId, NetworkType};
use secp256k1::rand::thread_rng;
use std::error::Error;
use std::fs;
use std::path::{Path, PathBuf};

const NETWORK_ID: NetworkId = NetworkId::with_suffix(NetworkType::Testnet, 12);
const HELPER_PATH: &str = "spikes/tn12-minimal-covenant/env-049-key-address-helper/src/main.rs";
const SECRET_PATH_REL: &str = "spikes/tn12-minimal-covenant/local-secrets/env-049-key-address/tn12-test-only-key.private";
const SUMMARY_PATH_REL: &str = "spikes/tn12-minimal-covenant/artifacts/env-049-key-address/env-049-summary.txt";

fn hex_encode(bytes: &[u8]) -> String {
    let mut s = String::with_capacity(bytes.len() * 2);
    for byte in bytes {
        use std::fmt::Write as _;
        let _ = write!(&mut s, "{byte:02x}");
    }
    s
}

fn ensure_parent_dirs(path: &Path) -> Result<(), Box<dyn Error>> {
    let parent = path
        .parent()
        .ok_or_else(|| format!("path has no parent: {}", path.display()))?;
    fs::create_dir_all(parent)?;
    Ok(())
}

fn repo_root() -> Result<PathBuf, Box<dyn Error>> {
    let manifest_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let repo_root = manifest_dir
        .ancestors()
        .nth(3)
        .ok_or("failed to resolve repo root from CARGO_MANIFEST_DIR")?;
    Ok(repo_root.to_path_buf())
}

fn main() -> Result<(), Box<dyn Error>> {
    if NETWORK_ID.network_type() != NetworkType::Testnet || NETWORK_ID.suffix() != Some(12) {
        return Err("refusing to run: network id is not fixed to testnet-12".into());
    }

    let repo_root = repo_root()?;
    let secret_path = repo_root.join(SECRET_PATH_REL);
    let summary_path = repo_root.join(SUMMARY_PATH_REL);

    if secret_path.exists() || summary_path.exists() {
        return Err("refusing to run: output already exists, to avoid accidental second key/address generation".into());
    }

    ensure_parent_dirs(&secret_path)?;
    ensure_parent_dirs(&summary_path)?;

    let mut rng = thread_rng();
    let secret_key = secp256k1::SecretKey::new(&mut rng);
    let public_key = secp256k1::PublicKey::from_secret_key_global(&secret_key);
    let (x_only_public_key, _) = public_key.x_only_public_key();
    let payload = x_only_public_key.serialize();
    let address = Address::new(Prefix::from(NETWORK_ID), Version::PubKey, &payload);

    if address.prefix != Prefix::Testnet {
        return Err("refusing to continue: generated address prefix is not testnet".into());
    }

    let secret_body = format!(
        "network_id={network_id}\naddress={address}\nsecret_key_hex={secret_key_hex}\n",
        network_id = NETWORK_ID,
        address = address,
        secret_key_hex = hex_encode(&secret_key.secret_bytes())
    );
    fs::write(&secret_path, secret_body)?;

    let summary = format!(
        concat!(
            "Result: PASS\n",
            "Network: {network_id} / TN12\n",
            "Public address generated: {address}\n",
            "Address prefix/type: {prefix} / {version}\n",
            "Tool/helper path used: {helper_path}\n",
            "Node contact required: no\n",
            "Faucet funds requested: no\n",
            "UTXO inspection performed: no\n",
            "Signing performed: no\n",
            "Transaction broadcast: no\n",
            "Private key/seed/mnemonic/wallet material committed: no\n",
            "Private material storage path: {secret_path}\n"
        ),
        network_id = NETWORK_ID,
        address = address,
        prefix = address.prefix,
        version = "PubKey",
        helper_path = HELPER_PATH,
        secret_path = SECRET_PATH_REL,
    );
    fs::write(&summary_path, summary)?;

    println!("PASS");
    println!("network_id={NETWORK_ID}");
    println!("address={address}");
    println!("summary_path={SUMMARY_PATH_REL}");
    println!("secret_path={SECRET_PATH_REL}");

    Ok(())
}
