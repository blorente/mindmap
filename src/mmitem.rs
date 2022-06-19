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
const h: f32 = 100.0;

impl Renderable for MMItem {
    fn render(&self, coords: (f32, f32), settings: &RenderItemSettings, canvas: &mut Canvas) {
        let (x, y) = coords;
        let mut path = Path::new();
        path.rounded_rect(x, y, w, h, 20.0);

        canvas.fill_path(&mut path, Paint::color(self.color));

        let mut text_paint = Paint::color(settings.text_color);
        text_paint.set_font(&[settings.font.expect(&format!(
            "Font not initialized in settings when rendering MMItem({})",
            &self.text
        ))]);
        text_paint.set_font_size(settings.font_size);
        let font_metrics: FontMetrics = canvas
            .measure_font(text_paint)
            .expect("Error measuring font");

        let lines = canvas
            .break_text_vec(w, &self.text, text_paint)
            .expect("Error while breaking text");

        // We're going to offset y per line of text
        let mut y = y;
        for line_range in lines {
            if let Ok(_res) = canvas.fill_text(x, y, &self.text[line_range], text_paint) {
                y += font_metrics.height();
            }
        }
    }

    fn bounds(&self, settings: &RenderItemSettings) -> crate::renderable::RenderItemBounds {
        // TODO Calculate bounds with text, margin and stuff.
        RenderItemBounds { w, h }
    }
}
