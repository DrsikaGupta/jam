use anyhow::{Context, Result};
use directories::ProjectDirs;
use std::path::PathBuf;

const QUALIFIER: &str = "com";
const ORGANIZATION: &str = "jam";
const APPLICATION: &str = "jam";

pub fn config_dir() -> Result<PathBuf> {
    let dirs = ProjectDirs::from(QUALIFIER, ORGANIZATION, APPLICATION)
        .context("Unable to determine configuration directory")?;

    Ok(dirs.config_dir().to_path_buf())
}

pub fn config_file() -> Result<PathBuf> {
    Ok(config_dir()?.join("config.toml"))
}
