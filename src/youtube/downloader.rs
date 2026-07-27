use std::{fs, path::PathBuf, process::Command};

use anyhow::{Result, anyhow};

fn cache_dir() -> Result<PathBuf> {
    let dir = std::env::current_dir()?.join("cache").join("youtube");

    fs::create_dir_all(&dir)?;

    Ok(dir)
}

pub fn download(video_id: &str) -> Result<PathBuf> {
    let cache = cache_dir()?;

    let output = cache.join(format!("{video_id}.mp3"));

    if output.exists() {
        return Ok(output);
    }

    let url = format!("https://www.youtube.com/watch?v={}", video_id);

    let status = Command::new("yt-dlp")
        .args([
            "--js-runtimes",
            "node",
            "-x",
            "--audio-format",
            "mp3",
            "--audio-quality",
            "0",
            "-o",
            output.to_str().unwrap(),
            &url,
        ])
        .status()?;

    if !status.success() {
        return Err(anyhow!("yt-dlp download failed"));
    }

    Ok(output)
}
