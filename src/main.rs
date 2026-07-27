mod app;
mod audio;
mod cache;
mod config;
mod keybindings;
mod library;
mod plugin;
mod search;
mod theme;
mod tui;
mod youtube;

use anyhow::Result;
use app::startup::{
    initializer::Initializer,
    splash::{SplashScreen, create_terminal},
};
use std::path::Path;

fn main() -> Result<()> {
    let mut terminal = create_terminal()?;

    let mut splash = SplashScreen::new();

    let initializer = Initializer::new();

    let result = initializer.run(&mut splash, &mut terminal);

    result?;

    // println!("Jam started successfully!");
    // let mut app = tui::app::App::new(terminal)?;

    // app.run()?;

    // let terminal = app.into_terminal();

    // app::startup::splash::restore_terminal(terminal)?;

    // Ok(())

    // app::startup::splash::restore_terminal(terminal)?;

    // // Test the folder picker
    //if let Some(folder) = library::FilePicker::pick_folder() {
    //    library::Importer::import_folder(folder)?;
    //}
    let mut app = tui::app::App::new(terminal)?;

    app.run()?;

    let terminal = app.into_terminal();

    app::startup::splash::restore_terminal(terminal)?;

    Ok(())

    // Ok(())
}
