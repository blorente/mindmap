use crate::renderable::{Canvas, RenderItemBounds, Renderable};
use crate::settings::RenderItemSettings;
use femtovg::{Color, Paint, Path};

pub struct MMItem {
    text: String,
}

impl MMItem {
    pub fn new(text: String) -> Self {
        return MMItem { text };
    }
}

impl Renderable for MMItem {
    fn render(&self, coords: (f32, f32), settings: &RenderItemSettings, canvas: &mut Canvas) {
        let (h, w) = (100.0, 100.0);

        let mut path = Path::new();
        path.rounded_rect(coords.0, coords.1, w, h, 20.0);
        canvas.fill_path(&mut path, Paint::color(Color::rgba(180, 0, 0, 180)));
    }

    fn bounds(&self, settings: &RenderItemSettings) -> crate::renderable::RenderItemBounds {
        // TODO Calculate bounds with text, margin and stuff.
        RenderItemBounds { w: 100.0, h: 100.0 }
    }
}
