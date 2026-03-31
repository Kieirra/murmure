use anyhow::{Context, Result};
use log::info;
use rcgen::{CertificateParams, KeyPair, SanType};
use std::path::PathBuf;
use tauri::Manager;
use time::OffsetDateTime;

/// Get the directory where SmartMic data is stored
fn smartmic_data_dir(app: &tauri::AppHandle) -> Result<PathBuf> {
    let dir = app
        .path()
        .app_data_dir()
        .context("Failed to resolve app data dir")?
        .join("smartmic");

    if !dir.exists() {
        std::fs::create_dir_all(&dir).context("Failed to create smartmic data dir")?;
    }

    Ok(dir)
}

/// Get or create a self-signed TLS certificate for SmartMic HTTPS server.
/// The certificate is persisted to disk and reused across restarts.
/// Returns (cert_der, key_der) for use with rustls.
pub fn get_or_create_cert(app: &tauri::AppHandle) -> Result<(Vec<u8>, Vec<u8>)> {
    let dir = smartmic_data_dir(app)?;
    let cert_path = dir.join("cert.pem");
    let key_path = dir.join("key.pem");

    // Try to load existing cert and key
    if cert_path.exists() && key_path.exists() {
        // Check if cert is still valid via file age (< 10 years)
        let is_expired = std::fs::metadata(&cert_path)
            .and_then(|m| m.modified())
            .map(|modified| {
                modified.elapsed().unwrap_or_default() > std::time::Duration::from_secs(10 * 365 * 24 * 3600)
            })
            .unwrap_or(true);

        if !is_expired {
            let cert_pem =
                std::fs::read_to_string(&cert_path).context("Failed to read cert.pem")?;
            let key_pem =
                std::fs::read_to_string(&key_path).context("Failed to read key.pem")?;

            info!("Reusing existing SmartMic TLS certificate");
            let cert_der = pem_to_der(&cert_pem, "CERTIFICATE")?;
            let key_der = pem_to_der(&key_pem, "PRIVATE KEY")?;
            return Ok((cert_der, key_der));
        }

        info!("SmartMic TLS certificate expired, regenerating");
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
            std::net::IpAddr::V4(std::net::Ipv4Addr::new(127, 0, 0, 1)),
        )));
    params
        .subject_alt_names
        .push(SanType::IpAddress(std::net::IpAddr::V4(
            std::net::Ipv4Addr::new(127, 0, 0, 1),
        )));
    params
        .subject_alt_names
        .push(SanType::DnsName("localhost".try_into().context("Invalid DNS name")?));

    // Dynamic validity: now to now + 10 years
    let now = OffsetDateTime::now_utc();
    params.not_before = now;
    params.not_after = now
        .replace_year(now.year() + 10)
        .unwrap_or(now + time::Duration::days(3650));

    let key_pair = KeyPair::generate().context("Failed to generate key pair")?;
    let cert = params
        .self_signed(&key_pair)
        .context("Failed to generate self-signed cert")?;

    let cert_pem = cert.pem();
    let key_pem = key_pair.serialize_pem();

    // Persist to disk
    std::fs::write(&cert_path, &cert_pem).context("Failed to write cert.pem")?;
    std::fs::write(&key_path, &key_pem).context("Failed to write key.pem")?;

    let cert_der = cert.der().to_vec();
    let key_der = key_pair.serialize_der();

    Ok((cert_der, key_der))
}

/// Extract DER bytes from a PEM string
fn pem_to_der(pem: &str, label: &str) -> Result<Vec<u8>> {
    let b64: String = pem
        .lines()
        .filter(|line| !line.starts_with("-----"))
        .collect::<Vec<&str>>()
        .join("");

    if b64.is_empty() {
        return Err(anyhow::anyhow!(
            "No {} found between -----BEGIN {}----- and -----END {}-----",
            label,
            label,
            label
        ));
    }

    use base64::Engine;
    base64::engine::general_purpose::STANDARD
        .decode(&b64)
        .context("Failed to decode PEM base64")
}
