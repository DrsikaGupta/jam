use crate::tui::screen::Screen;

pub const HOME_MENU: &[&str] = &["🎵 Local Music", "🌐 YouTube", "⚙ Settings", "Help", "Exit"];

pub fn selected_screen(index: usize) -> Screen {
    match index {
        0 => Screen::LocalMusic,
        1 => Screen::YouTube,
        2 => Screen::Settings,
        3 => Screen::Help,
        _ => Screen::Exit,
    }
}
