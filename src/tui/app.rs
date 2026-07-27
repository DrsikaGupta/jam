use std::{io::Stdout, time::Duration};

use crossterm::event::{self, Event, KeyCode, KeyEventKind};

use crate::audio::AudioPlayer;
use crate::audio::playback_manager::PlaybackManager;
use crate::tui::home;
use crate::tui::local_music::LocalMusicScreen;
use crate::tui::now_playing::NowPlayingScreen;
use crate::tui::renderer::Renderer;
use crate::tui::screen::Screen;
use crate::youtube::downloader;
use crate::youtube::screen::YoutubeScreen;
use anyhow::Result;
use ratatui::{Terminal, backend::CrosstermBackend};
use ratatui_image::picker::Picker;

pub struct App {
    terminal: Terminal<CrosstermBackend<Stdout>>,

    pub current_screen: Screen,

    pub should_quit: bool,

    pub home_selected: usize,
    pub now_playing: NowPlayingScreen,
    pub local_music: LocalMusicScreen,
    pub audio: AudioPlayer,
    pub playback: PlaybackManager,
    pub youtube: YoutubeScreen,
    pub picker: Picker,
}

impl App {
    pub fn new(terminal: Terminal<CrosstermBackend<Stdout>>) -> Result<Self> {
        // Load library once
        let local_music = LocalMusicScreen::new()?;

        // Initialize playback queue
        let mut playback = PlaybackManager::new();
        playback.load(local_music.tracks.clone());

        Ok(Self {
            terminal,

            current_screen: Screen::Home,

            should_quit: false,

            home_selected: 0,
            now_playing: NowPlayingScreen::new(),
            local_music,
            audio: AudioPlayer::new()?,
            playback,
            youtube: YoutubeScreen::new(),
            picker: Picker::from_query_stdio().unwrap(),
        })
    }

    pub fn run(&mut self) -> Result<()> {
        while !self.should_quit {
            self.check_autoplay()?;

            self.draw()?;

            self.handle_events()?;
        }

        Ok(())
    }

    fn draw(&mut self) -> Result<()> {
        Renderer::draw(
            &mut self.terminal,
            self.current_screen,
            self.home_selected,
            &self.local_music,
            &self.youtube,
            &mut self.now_playing,
            &self.audio,
            &self.playback,
        )
    }

    fn handle_events(&mut self) -> Result<()> {
        if !event::poll(Duration::from_millis(16))? {
            return Ok(());
        }

        let Event::Key(key) = event::read()? else {
            return Ok(());
        };

        if key.kind != KeyEventKind::Press {
            return Ok(());
        }

        match self.current_screen {
            Screen::Home => match key.code {
                KeyCode::Char('q') => {
                    self.should_quit = true;
                }

                KeyCode::Up => {
                    if self.home_selected > 0 {
                        self.home_selected -= 1;
                    }
                }

                KeyCode::Down => {
                    if self.home_selected + 1 < home::HOME_MENU.len() {
                        self.home_selected += 1;
                    }
                }

                KeyCode::Enter => {
                    self.current_screen = home::selected_screen(self.home_selected);

                    if self.current_screen == Screen::Exit {
                        self.should_quit = true;
                    }
                }

                _ => {}
            },

            Screen::LocalMusic => {
                //--------------------------------------------------
                // Search Mode
                //--------------------------------------------------

                if self.local_music.search_mode {
                    match key.code {
                        KeyCode::Esc => {
                            self.local_music.search_mode = false;
                            self.local_music.query.clear();
                            self.local_music.apply_filter();
                        }

                        KeyCode::Enter => {
                            self.local_music.search_mode = false;
                        }

                        KeyCode::Backspace => {
                            self.local_music.query.pop();
                            self.local_music.apply_filter();
                        }

                        KeyCode::Char(c) => {
                            self.local_music.query.push(c);
                            self.local_music.apply_filter();
                        }

                        _ => {}
                    }

                    return Ok(());
                }

                //--------------------------------------------------
                // Normal Library Mode
                //--------------------------------------------------

                match key.code {
                    KeyCode::Char('q') => {
                        self.should_quit = true;
                    }

                    //--------------------------------------------------
                    // Open Search
                    //--------------------------------------------------
                    KeyCode::Char('/') => {
                        self.local_music.search_mode = true;
                    }

                    KeyCode::Esc => {
                        self.current_screen = Screen::Home;
                    }

                    KeyCode::Up => {
                        self.local_music.previous();
                    }

                    KeyCode::Down => {
                        self.local_music.next();
                    }

                    KeyCode::Enter => {
                        if let Some(track) = self.local_music.current() {
                            let index = self.local_music.filtered[self.local_music.selected];

                            self.playback.select(index);

                            self.audio.play(track.clone())?;

                            self.now_playing
                                .artwork
                                .set_image(&mut self.picker, track.artwork.as_ref());

                            self.current_screen = Screen::NowPlaying;
                        }
                    }

                    _ => {}
                }
            }
            Screen::YouTube => {
                if self.youtube.search_mode {
                    match key.code {
                        KeyCode::Esc => {
                            self.youtube.search_mode = false;
                        }
                        KeyCode::Char(c) => {
                            self.youtube.query.push(c);
                        }
                        KeyCode::Backspace => {
                            self.youtube.query.pop();
                        }

                        KeyCode::Enter => {
                            self.youtube.search();
                            self.youtube.search_mode = false;
                        }

                        _ => {}
                    }

                    return Ok(());
                }

                match key.code {
                    KeyCode::Char('/') => {
                        self.youtube.query.clear();
                        self.youtube.search_mode = true;
                    }
                    KeyCode::Esc => {
                        self.current_screen = Screen::Home;
                    }

                    KeyCode::Up => {
                        self.youtube.previous();
                    }

                    KeyCode::Down => {
                        self.youtube.next();
                    }

                    KeyCode::Enter => {
                        if let Some(video) = self.youtube.current().cloned() {
                            let path = downloader::download(&video.id)?;

                            let track = video.into_track(path);

                            self.playback.load(vec![track.clone()]);

                            self.audio.play(track.clone())?;

                            self.now_playing
                                .artwork
                                .set_image(&mut self.picker, track.artwork.as_ref());

                            self.current_screen = Screen::NowPlaying;
                        }
                    }

                    _ => {}
                }
            }
            Screen::NowPlaying => match key.code {
                KeyCode::Char('q') => {
                    self.should_quit = true;
                }

                KeyCode::Esc => {
                    self.current_screen = Screen::LocalMusic;
                }

                KeyCode::Char(' ') => {
                    if self.audio.is_paused() {
                        self.audio.resume();
                    } else {
                        self.audio.pause();
                    }
                }
                KeyCode::Right | KeyCode::Char('n') => {
                    if let Some(track) = self.playback.next() {
                        self.audio.play(track.clone())?;

                        self.now_playing
                            .artwork
                            .set_image(&mut self.picker, track.artwork.as_ref());
                    }
                }
                KeyCode::Left | KeyCode::Char('b') => {
                    if let Some(track) = self.playback.previous() {
                        self.audio.play(track.clone())?;

                        self.now_playing
                            .artwork
                            .set_image(&mut self.picker, track.artwork.as_ref());
                    }
                }
                KeyCode::Char('s') => {
                    self.playback.toggle_shuffle();
                }
                KeyCode::Char('r') => {
                    self.playback.cycle_repeat();
                }

                _ => {}
            },

            _ => match key.code {
                KeyCode::Char('q') => {
                    self.should_quit = true;
                }

                KeyCode::Esc => {
                    self.current_screen = Screen::Home;
                }

                _ => {}
            },
        }

        Ok(())
    }

    fn check_autoplay(&mut self) -> Result<()> {
        if !self.audio.is_empty() {
            return Ok(());
        }

        if self.audio.state().current_track.is_none() {
            return Ok(());
        }

        if let Some(track) = self.playback.next() {
            if let Some(index) = self
                .local_music
                .tracks
                .iter()
                .position(|t| t.path == track.path)
            {
                if let Some(pos) = self.local_music.filtered.iter().position(|&i| i == index) {
                    self.local_music.selected = pos;
                }
            }

            self.audio.play(track.clone())?;

            self.now_playing
                .artwork
                .set_image(&mut self.picker, track.artwork.as_ref());
        }

        Ok(())
    }

    pub fn into_terminal(self) -> Terminal<CrosstermBackend<Stdout>> {
        self.terminal
    }
}
