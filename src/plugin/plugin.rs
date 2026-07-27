use std::path::PathBuf;

#[derive(Debug, Clone)]
pub struct PluginInfo {
    pub name: String,
    pub path: PathBuf,
}
