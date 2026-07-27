use anyhow::Result;
use std::{collections::HashSet, fs, path::PathBuf};

pub struct HashDatabase {
    hashes: HashSet<String>,
}

impl HashDatabase {
    pub fn load() -> Result<Self> {
        let path = PathBuf::from("./music/hashes.txt");

        let mut hashes = HashSet::new();

        if path.exists() {
            let content = fs::read_to_string(&path)?;

            for line in content.lines() {
                hashes.insert(line.to_string());
            }
        }

        Ok(Self { hashes })
    }

    pub fn add(&mut self, hash: String) -> bool {
        self.hashes.insert(hash)
    }

    pub fn contains(&self, hash: &str) -> bool {
        self.hashes.contains(hash)
    }

    pub fn insert(&mut self, hash: String) {
        self.hashes.insert(hash);
    }

    pub fn save(&self) -> Result<()> {
        let path = PathBuf::from("./music/hashes.txt");

        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)?;
        }

        let mut hashes: Vec<_> = self.hashes.iter().cloned().collect();
        hashes.sort();

        fs::write(path, hashes.join("\n"))?;

        Ok(())
    }
}
