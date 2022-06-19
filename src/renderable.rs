use crate::settings::RenderItemSettings;

pub type Canvas = femtovg::Canvas<femtovg::renderer::OpenGl>;

pub struct RenderItemBounds {
    pub w: f32,
    pub h: f32,
}
pub trait Renderable {
    fn render(&self, coords: (f32, f32), settings: &RenderItemSettings, canvas: &mut Canvas) {}
    fn bounds(&self, settings: &RenderItemSettings) -> RenderItemBounds;
}
