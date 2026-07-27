use crate::app::constants::{MIN_TERMINAL_HEIGHT, MIN_TERMINAL_WIDTH};
use crate::app::{
    constants::STARTUP_STEPS,
    startup::splash::{SplashScreen, sleep_frame},
};
use crate::audio::AudioBackend;
use crate::cache::CacheManager;
use crate::config::Config;
use crate::keybindings::KeybindingManager;
use crate::plugin::PluginManager;
use crate::theme::ThemeManager;
use anyhow::Result;
use crossterm::terminal;
use ratatui::{Terminal, backend::CrosstermBackend};

type StartupTask = fn() -> Result<()>;

pub struct Initializer {
    tasks: Vec<StartupTask>,
}

impl Initializer {
    pub fn new() -> Self {
        Self {
            tasks: vec![
                detect_terminal,
                load_configuration,
                initialize_audio,
                initialize_cache,
                detect_plugins,
                load_themes,
                load_keybindings,
            ],
        }
    }

    pub fn run(
        &self,
        splash: &mut SplashScreen,
        terminal: &mut Terminal<CrosstermBackend<std::io::Stdout>>,
    ) -> Result<()> {
        for (task, step) in self.tasks.iter().zip(STARTUP_STEPS.iter()) {
            splash.set_step(*step);

            // Give the spinner a few frames so the
            // user actually sees the animation.
            for _ in 0..3 {
                splash.tick();
                splash.draw(terminal)?;
                sleep_frame();
            }
            std::thread::sleep(std::time::Duration::from_secs(1));
            task()?;
        }

        Ok(())
    }
}

/// -------------------------------
/// Startup Tasks
/// (Stub implementations for now)
/// -------------------------------

fn detect_terminal() -> Result<()> {
    let (width, height) = terminal::size()?;

    if width < MIN_TERMINAL_WIDTH {
        anyhow::bail!(
            "Terminal width too small ({} < {}).",
            width,
            MIN_TERMINAL_WIDTH
        );
    }

    if height < MIN_TERMINAL_HEIGHT {
        anyhow::bail!(
            "Terminal height too small ({} < {}).",
            height,
            MIN_TERMINAL_HEIGHT
        );
    }

    Ok(())
}

fn load_configuration() -> Result<()> {
    let _config = Config::load()?;

    Ok(())
}

fn initialize_audio() -> Result<()> {
    AudioBackend::initialize()?;

    Ok(())
}

fn initialize_cache() -> Result<()> {
    let _ = CacheManager::initialize()?;

    Ok(())
}

fn detect_plugins() -> Result<()> {
    let manager = PluginManager::initialize()?;

    Ok(())
}

fn load_themes() -> Result<()> {
    let manager = ThemeManager::initialize()?;

    Ok(())
}

fn load_keybindings() -> Result<()> {
    let manager = KeybindingManager::initialize()?;

    Ok(())
}
