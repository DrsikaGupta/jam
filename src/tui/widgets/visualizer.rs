use ratatui::{
    Frame,
    layout::{Alignment, Rect},
    style::{Color, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph},
};

pub struct VisualizerWidget;

impl VisualizerWidget {
    pub fn draw(frame: &mut Frame, area: Rect, levels: &[f32]) {
        const HEIGHT: usize = 8;

        //--------------------------------------------------
        // Smooth neighbouring bars
        //--------------------------------------------------

        let mut smooth = levels.to_vec();

        if smooth.len() >= 3 {
            for i in 2..smooth.len() - 2 {
                // smooth[i] =
                //     levels[i - 1] * 0.20
                //     + levels[i] * 0.60
                //     + levels[i + 1] * 0.20;
                smooth[i] = levels[i - 2] * 0.05
                    + levels[i - 1] * 0.20
                    + levels[i] * 0.50
                    + levels[i + 1] * 0.20
                    + levels[i + 2] * 0.05;
            }
        }

        //--------------------------------------------------
        // Build every row
        //--------------------------------------------------

        let mut lines = Vec::new();

        for row in (0..HEIGHT).rev() {
            let mut spans = Vec::new();

            for (i, level) in smooth.iter().enumerate() {
                let height = (*level * HEIGHT as f32).round() as usize;

                let color = Self::gradient(i, smooth.len());

                if height > row {
                    spans.push(Span::styled("█", Style::default().fg(color)));
                } else {
                    spans.push(Span::raw(" "));
                }

                // Space between bars
                spans.push(Span::raw(" "));
            }

            lines.push(Line::from(spans));
        }

        //--------------------------------------------------
        // Render
        //--------------------------------------------------

        let paragraph = Paragraph::new(lines)
            .alignment(Alignment::Center)
            .block(Block::default().title(" Visualizer ").borders(Borders::ALL));

        frame.render_widget(paragraph, area);
    }

    fn gradient(index: usize, total: usize) -> Color {
        let t = index as f32 / (total.max(2) - 1) as f32;

        let stops = [
            (128.0, 0.0, 255.0),
            (0.0, 128.0, 255.0),
            (0.0, 255.0, 255.0),
            (0.0, 255.0, 128.0),
            (255.0, 255.0, 0.0),
            (255.0, 128.0, 0.0),
            (255.0, 0.0, 0.0),
        ];

        let segments = stops.len() - 1;

        let scaled = t * segments as f32;

        let i = scaled.floor() as usize;

        let frac = scaled - i as f32;

        let (r1, g1, b1) = stops[i];
        let (r2, g2, b2) = stops[(i + 1).min(segments)];

        Color::Rgb(
            (r1 + (r2 - r1) * frac) as u8,
            (g1 + (g2 - g1) * frac) as u8,
            (b1 + (b2 - b1) * frac) as u8,
        )
    }
}
