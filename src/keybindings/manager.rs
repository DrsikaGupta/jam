use anyhow::Result;
use crossterm::event::KeyCode;

use crate::keybindings::bindings::Bindings;

pub struct KeybindingManager {
    bindings: Bindings,
}

impl KeybindingManager {
    pub fn initialize() -> Result<Self> {
        let mut bindings = Bindings::new();

        bindings.insert("quit".into(), KeyCode::Char('q'));
        bindings.insert("play_pause".into(), KeyCode::Char(' '));
        bindings.insert("next".into(), KeyCode::Char('n'));
        bindings.insert("previous".into(), KeyCode::Char('p'));

        bindings.insert("volume_up".into(), KeyCode::Up);
        bindings.insert("volume_down".into(), KeyCode::Down);

        bindings.insert("search".into(), KeyCode::Char('/'));

        bindings.insert("help".into(), KeyCode::Char('?'));

        Ok(Self { bindings })
    }

    pub fn key_for(&self, action: &str) -> Option<&KeyCode> {
        self.bindings.get(action)
    }

    pub fn bindings(&self) -> &Bindings {
        &self.bindings
    }
}
