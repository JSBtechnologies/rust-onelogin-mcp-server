pub mod serde_helpers;

use base64::{engine::general_purpose, Engine as _};

pub fn base64_encode(data: &str) -> String {
    general_purpose::STANDARD.encode(data.as_bytes())
}

pub fn base64_decode(encoded: &str) -> Result<String, String> {
    general_purpose::STANDARD
        .decode(encoded)
        .map_err(|e| format!("Base64 decode error: {}", e))
        .and_then(|bytes| {
            String::from_utf8(bytes).map_err(|e| format!("UTF-8 decode error: {}", e))
        })
}
