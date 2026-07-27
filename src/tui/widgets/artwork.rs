use image::DynamicImage;

use ratatui::{
    Frame,
    layout::Rect,
    widgets::{Block, Borders, Paragraph},
};

use ratatui_image::{StatefulImage, picker::Picker, protocol::StatefulProtocol};

pub struct ArtworkWidget {
    protocol: Option<StatefulProtocol>,
}

impl ArtworkWidget {
    pub fn new() -> Self {
        Self { protocol: None }
    }

    pub fn set_image(&mut self, picker: &mut Picker, image: Option<&DynamicImage>) {
        self.protocol = image.map(|img| picker.new_resize_protocol(img.clone()));
    }

    pub fn draw(&mut self, frame: &mut Frame, area: Rect) {
        match &mut self.protocol {
            Some(protocol) => {
                frame.render_stateful_widget(StatefulImage::default(), area, protocol);
            }

            None => {
                let placeholder = Paragraph::new("\n\nAlbum Art\n\nComing Soon")
                    .block(Block::default().title(" Cover ").borders(Borders::ALL));

                frame.render_widget(placeholder, area);
            }
        }
    }
}
