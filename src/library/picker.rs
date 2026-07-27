use std::path::PathBuf;

pub struct FilePicker;

impl FilePicker {
    pub fn pick_files() -> Option<Vec<PathBuf>> {
        rfd::FileDialog::new()
            .add_filter(
                "Audio",
                &["mp3", "flac", "wav", "ogg", "opus", "m4a", "aac"],
            )
            .pick_files()
    }

    pub fn pick_folder() -> Option<PathBuf> {
        rfd::FileDialog::new().pick_folder()
    }
}
