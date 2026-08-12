use super::types::LoweredOutput;

pub fn unsupported_reason() -> Option<String> {
    Some("unsupported_platform".to_string())
}

pub fn lower(_percent: u8) -> Option<LoweredOutput> {
    None
}

pub fn restore(_state: &LoweredOutput) {}
