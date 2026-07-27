use anyhow::Result;
use image::load_from_memory;
use lofty::{
    file::TaggedFileExt,
    picture::PictureType,
    prelude::{Accessor, AudioFile},
    probe::Probe,
};
use std::{path::Path, time::Duration};

use crate::library::track::Track;

pub fn read_track(path: &Path) -> Result<Track> {
    let tagged_file = Probe::open(path)?.read()?;

    let properties = tagged_file.properties();

    let tag = tagged_file
        .primary_tag()
        .or_else(|| tagged_file.first_tag());
    // if let Some(tag) = tag {
    // dbg!("========================");

    //     for (i, tag) in tagged_file.tags().iter().enumerate() {
    //         dbg!(i);
    //         dbg!(tag.tag_type());
    //         dbg!(tag.pictures().len());
    //         dbg!(tag.title());
    //         dbg!(tag.artist());
    //         dbg!(tag.album());
    //         dbg!("------------------------");
    //     }

    //     dbg!(tagged_file.file_type());
    // }
    let artwork = tag
        .and_then(|tag| {
            tag.pictures()
                .iter()
                .find(|picture| picture.pic_type() == PictureType::CoverFront)
                .or_else(|| tag.pictures().iter().next())
        })
        .and_then(|picture| load_from_memory(picture.data()).ok());

    let title = tag
        .and_then(|t| t.title())
        .map(|s| s.to_string())
        .unwrap_or_else(|| path.file_stem().unwrap().to_string_lossy().to_string());

    let artist = tag.and_then(|t| t.artist()).map(String::from);

    let album = tag.and_then(|t| t.album()).map(String::from);

    Ok(Track {
        path: path.to_path_buf(),

        title,

        artist,

        album,

        genre: None,

        year: None,

        duration: Some(properties.duration()),

        sample_rate: properties.sample_rate(),

        bitrate: properties.audio_bitrate(),

        artwork,
    })
}
