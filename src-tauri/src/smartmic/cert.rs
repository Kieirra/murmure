use anyhow::{Context, Result};
use log::info;
use rcgen::{CertificateParams, KeyPair, SanType};
use std::path::PathBuf;
use time::OffsetDateTime;

/// Ensure a self-signed TLS certificate exists for SmartMic HTTPS server.
/// Returns (cert_path, key_path) as PEM files for use with RustlsConfig::from_pem_file.
pub fn ensure_cert(app: &tauri::AppHandle) -> Result<(PathBuf, PathBuf)> {
    let dir = super::smartmic_data_dir(app)?;
    let cert_path = dir.join("cert.pem");
    let key_path = dir.join("key.pem");

    // Check if existing cert is still valid (< 10 years old)
    let needs_regen = !cert_path.exists()
        || !key_path.exists()
        || std::fs::metadata(&cert_path)
            .and_then(|m| m.modified())
            .map(|modified| {
                modified.elapsed().unwrap_or_default()
                    > std::time::Duration::from_secs(10 * 365 * 24 * 3600)
            })
            .unwrap_or(true);

    if !needs_regen {
        info!("Reusing existing SmartMic TLS certificate");
        return Ok((cert_path, key_path));
    }

    // Generate new certificate
    info!("Generating new SmartMic TLS certificate");
    let local_ip = local_ip_address::local_ip()
        .map(|ip| ip.to_string())
        .unwrap_or_else(|_| "127.0.0.1".to_string());

    let mut params = CertificateParams::new(vec![local_ip.clone()])
        .context("Failed to create certificate params")?;

    params
        .subject_alt_names
        .push(SanType::IpAddress(local_ip.parse().unwrap_or(
            std::net::IpAddr::V4(std::net::Ipv4Addr::LOCALHOST),
        )));
    params
        .subject_alt_names
        .push(SanType::IpAddress(std::net::IpAddr::V4(
            std::net::Ipv4Addr::LOCALHOST,
        )));
    params
        .subject_alt_names
        .push(SanType::DnsName("localhost".try_into().context("Invalid DNS name")?));

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
    std::fs::write(&key_path, key_pair.serialize_pem()).context("Failed to write key.pem")?;

    Ok((cert_path, key_path))
}
