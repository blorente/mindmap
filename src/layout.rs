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
    for root in map.roots().iter() {
        let mut running_y = 0.0;
        positions.insert(root.clone(), (running_x, running_y));
        running_y += map
            .item(&root)
            .expect("failed to get handle to root")
            .bounds(&settings.render_item_settings)
            .h
            + 10.0;

        for child in map.children(root.clone()).iter() {
            positions.insert(child.clone(), (running_x, running_y));
        }
        running_x += map
            .item(&root)
            .expect("Error retrieving the item")
            .bounds(&settings.render_item_settings)
            .w
            + 10.0;
    }
    positions
}
