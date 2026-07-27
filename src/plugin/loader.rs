use std::fs;

use anyhow::{Context, Result};

use crate::plugin::plugin::PluginInfo;

pub struct PluginManager {
    plugins: Vec<PluginInfo>,
}

impl PluginManager {
    pub fn initialize() -> Result<Self> {
        let plugin_dir = std::path::PathBuf::from("plugins");

        if !plugin_dir.exists() {
            fs::create_dir_all(&plugin_dir).context("Failed to create plugins directory")?;
        }

        let mut plugins = Vec::new();

        for entry in fs::read_dir(&plugin_dir)? {
            let entry = entry?;
            let path = entry.path();

            if !path.is_file() {
                continue;
            }

            let extension = path.extension().and_then(|e| e.to_str()).unwrap_or("");

            #[cfg(target_os = "windows")]
            let valid = extension.eq_ignore_ascii_case("dll");

            #[cfg(target_os = "linux")]
            let valid = extension == "so";

            #[cfg(target_os = "macos")]
            let valid = extension == "dylib";

            if !valid {
                continue;
            }

            let name = path.file_stem().unwrap().to_string_lossy().to_string();

            plugins.push(PluginInfo { name, path });
        }

        Ok(Self { plugins })
    }

    pub fn plugins(&self) -> &[PluginInfo] {
        &self.plugins
    }
}
