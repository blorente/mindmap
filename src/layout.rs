use std::collections::HashMap;

use crate::{
    components::{MMItem, MMItemId},
    mind_map::MindMap,
    renderable::Renderable,
    settings::{RenderItemSettings, Settings},
};

pub(crate) type LayoutPosition = (f32, f32);
pub(crate) fn layout_mind_map(
    map: &MindMap,
    settings: &Settings,
) -> HashMap<MMItemId, LayoutPosition> {
    let mut positions: HashMap<MMItemId, LayoutPosition> = HashMap::new();
    let mut running_x = 0.0;
    for (id, item) in map.nodes.iter() {
        positions.insert(id.clone(), (running_x, 0.0));
        running_x += item.bounds(&settings.render_item_settings).w + 10.0;
    }
    positions
}
