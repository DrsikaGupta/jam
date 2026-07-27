use strsim::jaro_winkler;

/// Returns similarity score between 0.0 and 1.0
pub fn similarity(a: &str, b: &str) -> f64 {
    jaro_winkler(a, b)
}
