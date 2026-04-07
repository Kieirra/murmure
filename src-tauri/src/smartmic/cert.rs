use anyhow::{Context, Result};
use log::info;
use rcgen::{CertificateParams, KeyPair, SanType};
use time::OffsetDateTime;

/// Get or create a self-signed TLS certificate for SmartMic HTTPS server.
/// The certificate is persisted to disk and reused across restarts.
/// Returns (cert_der, key_der) for use with rustls.
pub fn get_or_create_cert(app: &tauri::AppHandle) -> Result<(Vec<u8>, Vec<u8>)> {
    let dir = super::smartmic_data_dir(app)?;
    let cert_path = dir.join("cert.der");
    let key_path = dir.join("key.der");

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
            let cert_der = std::fs::read(&cert_path).context("Failed to read cert.der")?;
            let key_der = std::fs::read(&key_path).context("Failed to read key.der")?;
            info!("Reusing existing SmartMic TLS certificate");
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

    let cert_der = cert.der().to_vec();
    let key_der = key_pair.serialize_der();

    // Persist to disk as DER
    std::fs::write(&cert_path, &cert_der).context("Failed to write cert.der")?;
    std::fs::write(&key_path, &key_der).context("Failed to write key.der")?;

    // Restrict permissions on sensitive files
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        let _ = std::fs::set_permissions(&cert_path, std::fs::Permissions::from_mode(0o600));
        let _ = std::fs::set_permissions(&key_path, std::fs::Permissions::from_mode(0o600));
    }

    Ok((cert_der, key_der))
}
