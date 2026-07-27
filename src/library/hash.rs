use anyhow::Result;
use sha2::{Digest, Sha256};
use std::{
    fs::File,
    io::{BufReader, Read},
    path::Path,
};

pub fn sha256(path: &Path) -> Result<String> {
    let file = File::open(path)?;
    let mut reader = BufReader::new(file);

    let mut hasher = Sha256::new();

    let mut buffer = [0u8; 8192];

    loop {
        let bytes = reader.read(&mut buffer)?;

        if bytes == 0 {
            break;
        }

        hasher.update(&buffer[..bytes]);
    }

    Ok(hex::encode(hasher.finalize()))
}
