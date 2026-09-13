use anyhow::{Context, Result};
use log::info;
use rcgen::{CertificateParams, KeyPair, SanType};
use std::fs::OpenOptions;
use std::io::Write;
use std::path::{Path, PathBuf};
use time::OffsetDateTime;

fn detected_ip() -> String {
    local_ip_address::local_ip()
        .map(|ip| ip.to_string())
        .unwrap_or_else(|_| "127.0.0.1".to_string())
}

fn configured_bind(app: &tauri::AppHandle) -> Option<String> {
    crate::settings::load_settings(app)
        .smartmic_bind_address
        .as_deref()
        .map(str::trim)
        .filter(|addr| !addr.is_empty())
        .map(str::to_string)
}

fn cert_identity(app: &tauri::AppHandle) -> String {
    format!(
        "{}\n{}",
        detected_ip(),
        configured_bind(app).unwrap_or_default()
    )
}

fn cert_is_expired(path: &Path) -> bool {
    std::fs::metadata(path)
        .and_then(|m| m.modified())
        .map(|modified| {
            modified.elapsed().unwrap_or_default()
                > std::time::Duration::from_secs(10 * 365 * 24 * 3600)
        })
        .unwrap_or(true)
}

fn write_secret_file(path: &Path, contents: &str) -> Result<()> {
    let mut opts = OpenOptions::new();
    opts.write(true).create(true).truncate(true);
    #[cfg(unix)]
    {
        use std::os::unix::fs::OpenOptionsExt;
        opts.mode(0o600);
    }
    let mut file = opts
        .open(path)
        .with_context(|| format!("Failed to open {}", path.display()))?;
    file.write_all(contents.as_bytes())
        .with_context(|| format!("Failed to write {}", path.display()))?;
    Ok(())
}

fn restrict_key_permissions(path: &Path) -> Result<()> {
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        let meta = std::fs::metadata(path)
            .with_context(|| format!("Failed to read {}", path.display()))?;
        let mut perms = meta.permissions();
        if perms.mode() & 0o777 != 0o600 {
            perms.set_mode(0o600);
            std::fs::set_permissions(path, perms)
                .context("Failed to set key.pem permissions")?;
        }
    }
    let _ = path;
    Ok(())
}

/// Ensure a self-signed TLS certificate exists for SmartMic HTTPS server.
/// Returns (cert_path, key_path) as PEM files for use with RustlsConfig::from_pem_file.
pub fn ensure_cert(app: &tauri::AppHandle) -> Result<(PathBuf, PathBuf)> {
    let dir = super::smartmic_data_dir(app)?;
    let cert_path = dir.join("cert.pem");
    let key_path = dir.join("key.pem");
    let identity_path = dir.join("cert.identity");
    let identity = cert_identity(app);

    let identity_matches = std::fs::read_to_string(&identity_path)
        .ok()
        .as_deref()
        == Some(identity.as_str());

    let needs_regen = !cert_path.exists()
        || !key_path.exists()
        || !identity_matches
        || cert_is_expired(&cert_path);

    if !needs_regen {
        info!("Reusing existing SmartMic TLS certificate");
        restrict_key_permissions(&key_path)?;
        return Ok((cert_path, key_path));
    }

    info!("Generating new SmartMic TLS certificate");

    let mut san_ips = vec!["127.0.0.1".to_string(), detected_ip()];
    if let Some(bind) = configured_bind(app) {
        if !san_ips.contains(&bind) {
            san_ips.push(bind);
        }
    }
    san_ips.sort();
    san_ips.dedup();

    let mut params = CertificateParams::new(san_ips.clone())
        .context("Failed to create certificate params")?;

    for ip in &san_ips {
        if let Ok(parsed) = ip.parse() {
            params.subject_alt_names.push(SanType::IpAddress(parsed));
        }
    }
    params.subject_alt_names.push(SanType::DnsName(
        "localhost".try_into().context("Invalid DNS name")?,
    ));

    let now = OffsetDateTime::now_utc();
    params.not_before = now;
    params.not_after = now
        .replace_year(now.year() + 10)
        .unwrap_or(now + time::Duration::days(3650));

    let key_pair = KeyPair::generate().context("Failed to generate key pair")?;
    let cert = params
        .self_signed(&key_pair)
        .context("Failed to generate self-signed cert")?;

    std::fs::write(&cert_path, cert.pem()).context("Failed to write cert.pem")?;
    write_secret_file(&key_path, &key_pair.serialize_pem())?;
    std::fs::write(&identity_path, identity).context("Failed to write cert.identity")?;

    Ok((cert_path, key_path))
}
