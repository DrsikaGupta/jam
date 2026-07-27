use anyhow::Result;

use std::{fs, path::PathBuf};

pub fn library_root() -> Result<PathBuf> {
    let root = PathBuf::from("./music/library");

    fs::create_dir_all(&root)?;

    Ok(root)
}
