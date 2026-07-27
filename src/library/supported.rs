use std::path::Path;

const SUPPORTED: &[&str] = &["mp3", "flac", "wav", "ogg", "opus", "m4a", "aac"];

pub fn is_supported(path: &Path) -> bool {
    path.extension()
        .and_then(|e| e.to_str())
        .map(|e| SUPPORTED.iter().any(|ext| ext.eq_ignore_ascii_case(e)))
        .unwrap_or(false)
}
