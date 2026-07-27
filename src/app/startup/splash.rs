use std::{
    io::{self, Stdout},
    time::Duration,
};

use crossterm::{
    cursor, execute,
    terminal::{self, Clear, ClearType},
};

use ratatui::{
    Terminal,
    backend::CrosstermBackend,
    layout::{Alignment, Constraint, Direction, Layout},
    style::{Modifier, Style},
    text::{Line, Span},
    widgets::Paragraph,
};

use crate::app::constants::{APP_NAME, ASCII_BANNER, SPINNER_FRAMES, VERSION};

pub struct SplashScreen {
    frame: usize,
    current_step: String,
}

impl SplashScreen {
    pub fn new() -> Self {
        Self {
            frame: 0,
            current_step: String::new(),
        }
    }

    pub fn set_step<S: Into<String>>(&mut self, step: S) {
        self.current_step = step.into();
    }

    pub fn tick(&mut self) {
        self.frame = (self.frame + 1) % SPINNER_FRAMES.len();
    }

    pub fn draw(&self, terminal: &mut Terminal<CrosstermBackend<Stdout>>) -> io::Result<()> {
        terminal.draw(|f| {
            let size = f.area();

            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .constraints([
                    Constraint::Percentage(25),
                    Constraint::Length(12),
                    Constraint::Length(2),
                    Constraint::Length(2),
                    Constraint::Percentage(61),
                ])
                .split(size);

            let banner = Paragraph::new(ASCII_BANNER).alignment(Alignment::Center);

            let version = Paragraph::new(format!("{}   v{}", APP_NAME, VERSION))
                .alignment(Alignment::Center)
                .style(Style::default().add_modifier(Modifier::BOLD));

            let loading = Paragraph::new(Line::from(vec![
                Span::raw(format!("{} ", SPINNER_FRAMES[self.frame])),
                Span::raw(&self.current_step),
            ]))
            .alignment(Alignment::Center);

            f.render_widget(banner, chunks[1]);
            f.render_widget(version, chunks[2]);
            f.render_widget(loading, chunks[3]);
        })?;

        Ok(())
    }
}

pub fn create_terminal() -> io::Result<Terminal<CrosstermBackend<Stdout>>> {
    terminal::enable_raw_mode()?;

    let mut stdout = io::stdout();

    execute!(
        stdout,
        terminal::EnterAlternateScreen,
        cursor::Hide,
        Clear(ClearType::All)
    )?;

    let backend = CrosstermBackend::new(stdout);

    Terminal::new(backend)
}

pub fn restore_terminal(mut terminal: Terminal<CrosstermBackend<Stdout>>) -> io::Result<()> {
    terminal::disable_raw_mode()?;

    execute!(
        terminal.backend_mut(),
        terminal::LeaveAlternateScreen,
        cursor::Show
    )?;

    terminal.show_cursor()
}

pub fn sleep_frame() {
    std::thread::sleep(Duration::from_millis(80));
}
