use std::process::Command;

use anyhow::{Result, anyhow};
use serde::Deserialize;

use super::models::YoutubeTrack;

#[derive(Debug, Deserialize)]
struct SearchResult {
    entries: Option<Vec<Option<YoutubeTrack>>>,
}

pub fn search(query: &str) -> Result<Vec<YoutubeTrack>> {
    let output = Command::new("yt-dlp")
        .args([
            "--ignore-errors",
            "--js-runtimes",
            "node",
            "--dump-single-json",
            &format!("ytsearch10:{query}"),
        ])
        .output()?;

    // Print stderr if yt-dlp emitted warnings/errors
    if !output.stderr.is_empty() {
        eprintln!("{}", String::from_utf8_lossy(&output.stderr));
    }

    // No JSON returned
    if output.stdout.is_empty() {
        return Err(anyhow!(
            "yt-dlp returned no JSON:\n{}",
            String::from_utf8_lossy(&output.stderr)
        ));
    }

    let result: SearchResult = serde_json::from_slice(&output.stdout)?;

    let tracks = result
        .entries
        .unwrap_or_default()
        .into_iter()
        .flatten()
        .collect::<Vec<_>>();

    Ok(tracks)
}
