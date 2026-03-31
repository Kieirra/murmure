use anyhow::{Context, Result};
use image::Luma;
use qrcode::QrCode;

/// Generate a QR code as a base64-encoded PNG data URI.
/// The QR code encodes `https://{ip}:{port}?token={token}`.
pub fn generate_qr_data_uri(ip: &str, port: u16, token: &str) -> Result<String> {
    let url = format!("https://{}:{}?token={}", ip, port, token);

    let code = QrCode::new(url.as_bytes()).context("Failed to generate QR code")?;

    let img = code.render::<Luma<u8>>().quiet_zone(true).build();

    let mut png_bytes: Vec<u8> = Vec::new();
    let encoder = image::codecs::png::PngEncoder::new(&mut png_bytes);
    image::ImageEncoder::write_image(
        encoder,
        img.as_raw(),
        img.width(),
        img.height(),
        image::ExtendedColorType::L8,
    )
    .context("Failed to encode QR code as PNG")?;

    use base64::Engine;
    let b64 = base64::engine::general_purpose::STANDARD.encode(&png_bytes);
    Ok(format!("data:image/png;base64,{}", b64))
}

/// Get the local IP address of this machine
pub fn get_local_ip() -> Result<String> {
    local_ip_address::local_ip()
        .map(|ip| ip.to_string())
        .context("Failed to detect local IP address")
}
