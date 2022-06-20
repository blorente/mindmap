use std::u32::MIN;

use crate::renderable::{Canvas, RenderItemBounds, Renderable};
use crate::settings::RenderItemSettings;
use femtovg::{Color, FontMetrics, Paint, Path};
use rand::random;

pub struct MMItem {
    text: String,
    color: Color,
}

impl MMItem {
    pub fn new(text: String) -> Self {
        let r = random::<u8>() % 255;
        let g = random::<u8>() % 255;
        let b = random::<u8>() % 255;
        return MMItem {
            text,
            color: Color::rgba(r, g, b, 180),
        };
    }
}

const w: f32 = 300.0;
const min_h: f32 = 100.0;

impl Renderable for MMItem {
    fn render(&self, coords: (f32, f32), settings: &RenderItemSettings, canvas: &mut Canvas) {
        let (x, y) = coords;

        let mut text_paint = Paint::color(settings.text_color);
        text_paint.set_font(&[settings.font.expect(&format!(
            "Font not initialized in settings when rendering MMItem({})",
            &self.text
        ))]);
        text_paint.set_font_size(settings.font_size);

        // Draw the text
        let font_metrics: FontMetrics = canvas
            .measure_font(text_paint)
            .expect("Error measuring font");

        let text_bounds_w = w - (2.0 * settings.margin);
        let lines = canvas
            .break_text_vec(text_bounds_w, &self.text, text_paint)
            .expect("Error while breaking text");

        let text_height = font_metrics.height() * lines.len() as f32;
        let text_x = x + settings.margin;
        let text_y = y + settings.margin + (font_metrics.height() / 2.0);

        // Draw the box
        let mut container = Path::new();
        let container_h = f32::max(min_h, text_height + (settings.margin * 2.0));
        container.rounded_rect(x, y, w, container_h, settings.border_radius);
        canvas.fill_path(&mut container, Paint::color(self.color));

        // Draw the text
        let mut text_y = text_y;
        for line_range in lines {
            if let Ok(_res) = canvas.fill_text(text_x, text_y, &self.text[line_range], text_paint) {
                text_y += font_metrics.height();
            }
        }
    }

    fn bounds(&self, settings: &RenderItemSettings) -> crate::renderable::RenderItemBounds {
        // TODO Calculate bounds with text, margin and stuff.
        RenderItemBounds { w, h: min_h }
    }
}
