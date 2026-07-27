pub struct YoutubeScreen {
    pub query: String,

    pub search_mode: bool,

    pub results: Vec<YoutubeTrack>,

    pub selected: usize,
}