use sha2::{Digest, Sha256};

pub fn compute_hash(data: &[u8]) -> String {
    let mut hasher = Sha256::new();
    hasher.update(data);
    format!("{:x}", hasher.finalize())
}

pub fn truncate_preview(text: &str, max_len: usize) -> String {
    if text.len() <= max_len {
        text.to_string()
    } else {
        let truncated: String = text.chars().take(max_len).collect();
        format!("{}...", truncated)
    }
}

pub fn detect_url(text: &str) -> Option<String> {
    let trimmed = text.trim();
    if trimmed.starts_with("http://") || trimmed.starts_with("https://") {
        // Check if it's a single URL (no spaces)
        if !trimmed.contains(' ') && !trimmed.contains('\n') {
            return Some(trimmed.to_string());
        }
    }
    None
}
