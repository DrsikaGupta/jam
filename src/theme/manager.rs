use anyhow::Result;
use ratatui::style::Color;

use crate::theme::theme::Theme;

pub struct ThemeManager {
    current: Theme,
}

impl ThemeManager {
    pub fn initialize() -> Result<Self> {
        let theme = Theme {
            name: "Default".into(),

            primary: Color::White,
            secondary: Color::Gray,
            accent: Color::Cyan,

            success: Color::Green,
            warning: Color::Yellow,
            error: Color::Red,

            background: Color::Black,

            border: Color::Blue,

            selection: Color::LightCyan,

            visualizer: Color::Magenta,
        };

        Ok(Self { current: theme })
    }

    pub fn current(&self) -> &Theme {
        &self.current
    }
}
