use crate::{
    audio::AudioBackend, cache::CacheManager, config::Config, keybindings::KeybindingManager,
    plugin::PluginManager, theme::ThemeManager,
};

pub struct AppState {
    pub config: Config,
    pub audio: AudioBackend,
    pub cache: CacheManager,
    pub plugins: PluginManager,
    pub themes: ThemeManager,
    pub keybindings: KeybindingManager,
}
