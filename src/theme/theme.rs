use ratatui::style::Color;

#[derive(Debug, Clone)]
pub struct Theme {
    pub name: String,

    pub primary: Color,
    pub secondary: Color,
    pub accent: Color,

    pub success: Color,
    pub warning: Color,
    pub error: Color,

    pub background: Color,

    pub border: Color,

    pub selection: Color,

    pub visualizer: Color,
}
