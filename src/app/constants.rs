pub const APP_NAME: &str = "Jam";

pub const VERSION: &str = env!("CARGO_PKG_VERSION");

pub const MIN_TERMINAL_WIDTH: u16 = 90;
pub const MIN_TERMINAL_HEIGHT: u16 = 28;

pub const SPLASH_DELAY_MS: u64 = 1200;

pub const SPINNER_FRAMES: [&str; 10] = ["⠋", "⠙", "⠹", "⠸", "⠼", "⠴", "⠦", "⠧", "⠇", "⠏"];

pub const STARTUP_STEPS: [&str; 7] = [
    "Detecting terminal...",
    "Loading configuration...",
    "Initializing audio backend...",
    "Initializing cache...",
    "Detecting plugins...",
    "Loading themes...",
    "Loading keybindings...",
];

pub const ASCII_BANNER: &str = r#"
       █████   █████████   ██████   ██████
      ░░███   ███░░░░░███ ░░██████ ██████
       ░███  ░███    ░███  ░███░█████░███
       ░███  ░███████████  ░███░░███ ░███
       ░███  ░███░░░░░███  ░███ ░░░  ░███
 ███   ░███  ░███    ░███  ░███      ░███
░░████████   █████   █████ █████     █████
 ░░░░░░░░   ░░░░░   ░░░░░ ░░░░░     ░░░░░
"#;
