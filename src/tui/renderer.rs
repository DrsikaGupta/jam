use std::io::Stdout;

use anyhow::Result;

use ratatui::{
    Frame, Terminal,
    backend::CrosstermBackend,
    layout::{Alignment, Constraint, Direction, Layout},
    style::{Modifier, Style},
    widgets::{Block, Borders, List, ListItem, ListState, Paragraph},
};

use crate::audio::AudioPlayer;
use crate::audio::playback_manager::PlaybackManager;
use crate::audio::playback_manager::RepeatMode;
use crate::tui::home::HOME_MENU;
use crate::tui::local_music::LocalMusicScreen;
use crate::tui::now_playing::NowPlayingScreen;
use crate::tui::screen::Screen;
use crate::tui::widgets::visualizer::VisualizerWidget;
use crate::youtube::screen::YoutubeScreen;
pub struct Renderer;

impl Renderer {
    pub fn draw(
        terminal: &mut Terminal<CrosstermBackend<Stdout>>,
        current_screen: Screen,
        home_selected: usize,
        local_music: &LocalMusicScreen,
        youtube: &YoutubeScreen,
        now_playing: &mut NowPlayingScreen,
        audio: &AudioPlayer,
        playback: &PlaybackManager,
    ) -> Result<()> {
        terminal.draw(|frame| {
            frame.render_widget(ratatui::widgets::Clear, frame.area());
            match current_screen {
                Screen::Home => {
                    Self::draw_home(frame, home_selected);
                }

                Screen::LocalMusic => {
                    Self::draw_local_music(frame, local_music);
                }
                Screen::NowPlaying => {
                    Self::draw_now_playing(frame, now_playing, audio, playback);
                }
                Screen::YouTube => {
                    Self::draw_youtube(frame, youtube);
                }

                _ => {
                    Self::draw_placeholder(frame);
                }
            }
        })?;

        Ok(())
    }

    fn draw_home(frame: &mut Frame, selected_index: usize) {
        let area = frame.area();

        let layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(3),
                Constraint::Min(0),
                Constraint::Length(3),
            ])
            .split(area);

        let header = Paragraph::new(" JAM ")
            .alignment(Alignment::Center)
            .style(Style::default().add_modifier(Modifier::BOLD))
            .block(Block::default().borders(Borders::ALL));

        frame.render_widget(header, layout[0]);

        let items: Vec<ListItem> = HOME_MENU.iter().map(|item| ListItem::new(*item)).collect();

        let list = List::new(items)
            .highlight_symbol("▶ ")
            .highlight_style(Style::default().add_modifier(Modifier::REVERSED))
            .block(Block::default().borders(Borders::ALL));

        let mut state = ListState::default();
        state.select(Some(selected_index));

        frame.render_stateful_widget(list, layout[1], &mut state);

        let footer = Paragraph::new("↑↓ Navigate   Enter Select   Q Quit")
            .alignment(Alignment::Center)
            .block(Block::default().borders(Borders::ALL));

        frame.render_widget(footer, layout[2]);
    }

    fn draw_placeholder(frame: &mut Frame) {
        let area = frame.area();

        let paragraph = Paragraph::new("Screen not implemented.")
            .alignment(Alignment::Center)
            .block(Block::default().borders(Borders::ALL));

        frame.render_widget(paragraph, area);
    }
    fn draw_local_music(frame: &mut Frame, screen: &crate::tui::local_music::LocalMusicScreen) {
        let area = frame.area();

        let layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(3),
                Constraint::Min(0),
                Constraint::Length(3),
            ])
            .split(area);

        let header_text = if screen.search_mode {
            format!(" 🔍 Search: {}█ ", screen.query)
        } else {
            " Local Music (/ to search) ".to_string()
        };

        let header = Paragraph::new(header_text)
            .alignment(Alignment::Left)
            .style(Style::default().add_modifier(Modifier::BOLD))
            .block(Block::default().borders(Borders::ALL));

        frame.render_widget(header, layout[0]);

        let items: Vec<ListItem> = screen
            .filtered
            .iter()
            .map(|&index| {
                let track = &screen.tracks[index];

                ListItem::new(format!(
                    "{} - {}",
                    track.title,
                    track.artist.as_deref().unwrap_or("Unknown Artist"),
                ))
            })
            .collect();

        let list = List::new(items)
            .highlight_symbol("▶ ")
            .highlight_style(Style::default().add_modifier(Modifier::REVERSED))
            .block(Block::default().borders(Borders::ALL));

        let mut state = ListState::default();
        state.select(Some(screen.selected));

        frame.render_stateful_widget(list, layout[1], &mut state);

        let footer = Paragraph::new("/Search   ↑↓ Navigate   Enter Play   Esc Home")
            .alignment(Alignment::Center)
            .block(Block::default().borders(Borders::ALL));

        frame.render_widget(footer, layout[2]);
    }
    fn draw_now_playing(
        frame: &mut Frame,
        screen: &mut NowPlayingScreen,
        audio: &AudioPlayer,
        playback: &PlaybackManager,
    ) {
        let area = frame.area();

        //--------------------------------------------------
        // Main Layout
        //--------------------------------------------------

        let layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(3), // Header
                Constraint::Min(0),    // Body
                Constraint::Length(5), // Visualizer
                Constraint::Length(3), // Footer
            ])
            .split(area);

        //--------------------------------------------------
        // Header
        //--------------------------------------------------

        let header = Paragraph::new(" NOW PLAYING ")
            .alignment(Alignment::Center)
            .style(Style::default().add_modifier(Modifier::BOLD))
            .block(Block::default().borders(Borders::ALL));

        frame.render_widget(header, layout[0]);

        //--------------------------------------------------
        // Body
        //--------------------------------------------------

        let body = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Length(22), Constraint::Min(0)])
            .split(layout[1]);

        //--------------------------------------------------
        // Square Artwork
        //--------------------------------------------------

        screen.artwork.draw(frame, body[0]);

        //--------------------------------------------------
        // Metadata
        //--------------------------------------------------

        let info = if let Some(track) = &audio.state().current_track {
            format!(
                "{}\n\n{}\n\n{}\n\nDuration : {}\n\nCodec    : MP3\nBitrate  : --- kbps\nSample   : --- Hz",
                track.title,
                track.artist.as_deref().unwrap_or("Unknown Artist"),
                track.album.as_deref().unwrap_or("Unknown Album"),
                track
                    .duration
                    .map(|d| format!("{:02}:{:02}", d.as_secs() / 60, d.as_secs() % 60))
                    .unwrap_or("--:--".to_string()),
            )
        } else {
            "Nothing Playing".to_string()
        };

        let metadata =
            Paragraph::new(info).block(Block::default().title(" Track ").borders(Borders::ALL));

        frame.render_widget(metadata, body[1]);

        VisualizerWidget::draw(frame, layout[2], &audio.visualizer().bars());

        //--------------------------------------------------
        // Footer
        //--------------------------------------------------

        let shuffle = if playback.shuffle() {
            "🟢 Shuffle"
        } else {
            "⚪ Shuffle"
        };

        let repeat = match playback.repeat() {
            RepeatMode::Off => "Repeat Off",
            RepeatMode::All => "Repeat All",
            RepeatMode::One => "Repeat One",
        };

        let footer = Paragraph::new(format!(
            "⏯ Space   ← Prev   → Next   (S){}   (R){}   (Esc)Library",
            shuffle, repeat,
        ))
        .alignment(Alignment::Center)
        .block(Block::default().borders(Borders::ALL));
        frame.render_widget(footer, layout[3]);
    }
    fn draw_youtube(frame: &mut Frame, screen: &YoutubeScreen) {
        let layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(3),
                Constraint::Min(0),
                Constraint::Length(3),
            ])
            .split(frame.area());

        //------------------------------------------------
        // Header
        //------------------------------------------------

        let title = if screen.search_mode {
            format!(" Search: {}", screen.query)
        } else {
            " YouTube ".to_string()
        };

        let header = Paragraph::new(title).block(Block::default().borders(Borders::ALL));

        frame.render_widget(header, layout[0]);

        //------------------------------------------------
        // Results
        //------------------------------------------------

        let items: Vec<ListItem> = screen
            .results
            .iter()
            .map(|video| {
                let line = format!(
                    "{} • {}",
                    video.title,
                    video.uploader.clone().unwrap_or_default(),
                );

                ListItem::new(line)
            })
            .collect();

        let list = List::new(items)
            .highlight_symbol("▶ ")
            .highlight_style(Style::default().add_modifier(Modifier::REVERSED))
            .block(Block::default().title(" Results ").borders(Borders::ALL));

        let mut state = ListState::default();

        state.select(Some(screen.selected));

        frame.render_stateful_widget(list, layout[1], &mut state);

        //------------------------------------------------
        // Footer
        //------------------------------------------------

        let footer = Paragraph::new("/ Search   ↑↓ Navigate   Enter Play   Esc Home")
            .alignment(Alignment::Center)
            .block(Block::default().borders(Borders::ALL));

        frame.render_widget(footer, layout[2]);
    }
}
