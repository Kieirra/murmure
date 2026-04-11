use anyhow::{Context, Result};
use qrcode::QrCode;

/// Generate a QR code as a base64-encoded SVG data URI from a full base URL.
/// The QR code encodes `{base_url}?token={token}`.
pub fn generate_qr_data_uri_from_base(base_url: &str, token: &str) -> Result<String> {
    let url = format!("{}?token={}", base_url, token);

    let code = QrCode::new(url.as_bytes()).context("Failed to generate QR code")?;

    let svg = code
        .render::<qrcode::render::svg::Color>()
        .quiet_zone(true)
        .build();

    use base64::Engine;
    let b64 = base64::engine::general_purpose::STANDARD.encode(svg.as_bytes());
    Ok(format!("data:image/svg+xml;base64,{}", b64))
}

/// Generate a QR code as a base64-encoded SVG data URI.
/// The QR code encodes `https://{ip}:{port}/?token={token}`.
pub fn generate_qr_data_uri(ip: &str, port: u16, token: &str) -> Result<String> {
    let base_url = format!("https://{}:{}/", ip, port);
    generate_qr_data_uri_from_base(&base_url, token)
}

/// Get the local IP address of this machine
pub fn get_local_ip() -> Result<String> {
    local_ip_address::local_ip()
        .map(|ip| ip.to_string())
        .context("Failed to detect local IP address")
}
