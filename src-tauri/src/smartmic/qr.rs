use anyhow::{Context, Result};
use qrcode::QrCode;

/// Append an optional `lang` query parameter to the SmartMic URL.
fn append_lang(url: &mut String, lang: Option<&str>) {
    if let Some(l) = lang {
        if url.contains('?') {
            url.push_str("&lang=");
        } else {
            url.push_str("?lang=");
        }
        url.push_str(l);
    }
}

/// Pairing URL encoded in the QR. The token is in the fragment so HTTPS
/// proxies do not log it (`https://host/?lang=en#token=...`).
pub fn pairing_url(base_url: &str, token: &str, lang: Option<&str>) -> String {
    let mut url = base_url.to_string();
    append_lang(&mut url, lang);
    url.push_str("#token=");
    url.push_str(token);
    url
}

fn encode_svg_data_uri(url: &str) -> Result<String> {
    let code = QrCode::new(url.as_bytes()).context("Failed to generate QR code")?;

    let svg = code
        .render::<qrcode::render::svg::Color>()
        .quiet_zone(true)
        .build();

    use base64::Engine;
    let b64 = base64::engine::general_purpose::STANDARD.encode(svg.as_bytes());
    Ok(format!("data:image/svg+xml;base64,{}", b64))
}

/// Generate a QR code as a base64-encoded SVG data URI from a full base URL.
/// The QR code encodes `{base_url}[?lang={lang}]#token={token}`.
pub fn generate_qr_data_uri_from_base(
    base_url: &str,
    token: &str,
    lang: Option<&str>,
) -> Result<String> {
    encode_svg_data_uri(&pairing_url(base_url, token, lang))
}

/// Generate a QR code as a base64-encoded SVG data URI.
/// The QR code encodes `https://{ip}:{port}/[?lang={lang}]#token={token}`.
pub fn generate_qr_data_uri(
    ip: &str,
    port: u16,
    token: &str,
    lang: Option<&str>,
) -> Result<String> {
    let base_url = format!("https://{}:{}/", ip, port);
    generate_qr_data_uri_from_base(&base_url, token, lang)
}

/// Get the local IP address of this machine
pub fn get_local_ip() -> Result<String> {
    local_ip_address::local_ip()
        .map(|ip| ip.to_string())
        .context("Failed to detect local IP address")
}

#[cfg(test)]
mod tests {
    use super::pairing_url;

    #[test]
    fn pairing_url_puts_the_token_in_the_fragment() {
        let url = pairing_url("https://10.0.0.4:4801/", "aaaaaaaa-bbbb-cccc-dddd-eeeeeeeeeeee", None);
        assert_eq!(
            url,
            "https://10.0.0.4:4801/#token=aaaaaaaa-bbbb-cccc-dddd-eeeeeeeeeeee"
        );
        assert!(!url.contains("?token="));
    }

    #[test]
    fn pairing_url_keeps_lang_in_the_query() {
        let url = pairing_url("https://relay.example/", "aaaaaaaa-bbbb-cccc-dddd-eeeeeeeeeeee", Some("fr"));
        assert_eq!(
            url,
            "https://relay.example/?lang=fr#token=aaaaaaaa-bbbb-cccc-dddd-eeeeeeeeeeee"
        );
    }
}
