// From https://docs.rs/uri_encode/latest/src/uri_encode/lib.rs.html#41-64

/// Encode an URL component, such as a path segment.
pub fn encode_uri_component(s: &str) -> String {
    let mut result = String::new();

    for c in s.chars() {
        if c.is_ascii_alphanumeric() || c == '-' || c == '_' || c == '.' || c == '~' {
            result.push(c);
        } else {
            let bytes = c.to_string().into_bytes();
            for byte in bytes {
                result.push_str(&format!("%{:02X}", byte));
            }
        }
    }

    result
}
