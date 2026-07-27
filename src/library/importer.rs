use anyhow::Result;

use std::{fs, path::Path};

use crate::library::{
    database::HashDatabase, filesystem::library_root, hash::sha256, supported::is_supported,
};

pub struct Importer;

impl Importer {
    pub fn import<P: AsRef<Path>>(path: P) -> Result<()> {
        let path = path.as_ref();

        let mut database = HashDatabase::load()?;

        if path.is_file() {
            let root = path.parent().unwrap_or(Path::new(""));
            Self::copy_file_preserve(root, path, &mut database)?;
        } else if path.is_dir() {
            Self::copy_directory_recursive(path, path, &mut database)?;
        }

        database.save()?;

        Ok(())
    }

    pub fn import_files(files: Vec<std::path::PathBuf>) -> Result<()> {
        let mut database = HashDatabase::load()?;

        for file in files {
            let root = file.parent().unwrap_or(Path::new(""));

            Self::copy_file_preserve(root, &file, &mut database)?;
        }

        database.save()?;

        Ok(())
    }

    pub fn import_folder(folder: std::path::PathBuf) -> Result<()> {
        let mut database = HashDatabase::load()?;

        Self::copy_directory_recursive(&folder, &folder, &mut database)?;

        database.save()?;

        Ok(())
    }

    fn copy_directory_recursive(
        root: &Path,
        current: &Path,
        database: &mut HashDatabase,
    ) -> Result<()> {
        for entry in fs::read_dir(current)? {
            let entry = entry?;
            let path = entry.path();

            if path.is_dir() {
                Self::copy_directory_recursive(root, &path, database)?;
            } else {
                Self::copy_file_preserve(root, &path, database)?;
            }
        }

        Ok(())
    }
    fn unique_destination(path: &Path) -> std::path::PathBuf {
        if !path.exists() {
            return path.to_path_buf();
        }

        let parent = path.parent().unwrap();

        let stem = path.file_stem().unwrap().to_string_lossy();

        let extension = path.extension().and_then(|e| e.to_str());

        let mut counter = 1;

        loop {
            let filename = match extension {
                Some(ext) => format!("{} ({}).{}", stem, counter, ext),
                None => format!("{} ({})", stem, counter),
            };

            let candidate = parent.join(filename);

            if !candidate.exists() {
                return candidate;
            }

            counter += 1;
        }
    }
    fn copy_file_preserve(root: &Path, file: &Path, database: &mut HashDatabase) -> Result<()> {
        if !is_supported(file) {
            return Ok(());
        }
        let hash = sha256(file)?;

        if database.contains(&hash) {
            println!("Duplicate skipped: {}", file.display());
            return Ok(());
        }
        let relative = file.strip_prefix(root)?;

        let destination = Self::unique_destination(&library_root()?.join(relative));
        if let Some(parent) = destination.parent() {
            fs::create_dir_all(parent)?;
        }

        fs::copy(file, destination)?;
        database.add(hash);

        Ok(())
    }
}
