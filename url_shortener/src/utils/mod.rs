use base62::encode;
use std::time::{SystemTime, UNIX_EPOCH};

pub fn generate_short_url(long_url: &str) -> String {
    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_nanos();
    
    // Combine URL with timestamp for uniqueness
    let input = format!("{}{}", long_url, timestamp);
    
    // Simple hash-based approach
    let hash = seahash::hash(input.as_bytes());
    
    // Base62 encode for URL-friendly output
    encode(hash as u64)[..8].to_string()
}