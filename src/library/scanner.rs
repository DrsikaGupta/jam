use anyhow::Result;

use std::{fs, path::Path};

use crate::library::{supported::is_supported, track::Track};

pub struct LibraryScanner;

impl LibraryScanner {
    pub fn scan(root: &Path) -> Result<Vec<Track>> {
        let mut tracks = Vec::new();

        Self::scan_recursive(root, &mut tracks)?;

        Ok(tracks)
    }

    fn scan_recursive(directory: &Path, tracks: &mut Vec<Track>) -> Result<()> {
        for entry in fs::read_dir(directory)? {
            let entry = entry?;

            let path = entry.path();

            if path.is_dir() {
                Self::scan_recursive(&path, tracks)?;
                continue;
            }

            if !is_supported(&path) {
                continue;
            }

            match crate::library::metadata::read_track(&path) {
                Ok(track) => tracks.push(track),

                Err(err) => {
                    eprintln!("Failed to read metadata for {:?}: {}", path, err);
                }
            }
        }

        Ok(())
    }
}
