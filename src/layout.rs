use crate::{renderable::Renderable, settings::RenderItemSettings};

type LayoutPositions = Vec<(f32, f32)>;
pub fn layout_line(
    items: &Vec<Box<dyn Renderable>>,
    settings: &RenderItemSettings,
) -> LayoutPositions {
    let mut positions: LayoutPositions = vec![];
    let mut running_x = 0.0;
    for item in items {
        positions.push((running_x, 0.0));
        running_x += item.bounds(settings).w + 10.0;
    }
    positions
}
