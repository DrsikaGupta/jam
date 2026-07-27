#[path = "../youtube/mod.rs"]
mod youtube;

use anyhow::Result;

fn main() -> Result<()> {
    println!("Searching YouTube...\n");

    let results = youtube::client::YoutubeClient::search("coldplay")?;

    println!("Found {} results\n", results.len());

    if let Some(video) = results.first() {
        println!("Title : {}", video.title);
        println!("ID    : {}", video.id);

        println!("\nGetting stream URL...\n");

        let url = youtube::client::YoutubeClient::stream_url(&video.id)?;

        println!("{}", url);
    }

    Ok(())
}