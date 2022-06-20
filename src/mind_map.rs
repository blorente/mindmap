use std::collections::HashMap;

use crate::components::{MMItem, MMItemId};

pub(crate) struct MindMap {
    // Stores actual instances of nodes.
    // The items have to be boxed so that we can treat them as `dyn Renderizable` in layout.
    pub(crate) nodes: HashMap<MMItemId, Box<MMItem>>,
    // Links items to children
    pub(crate) links: HashMap<MMItemId, Vec<MMItemId>>,
}

impl MindMap {
    pub(crate) fn new() -> Self {
        MindMap {
            nodes: HashMap::new(),
            links: HashMap::new(),
        }
    }

    pub(crate) fn create_node(
        &mut self,
        text: String,
        maybe_parent: Option<MMItemId>,
    ) -> Result<MMItemId, String> {
        let (new_node, new_id) = MMItem::new(text);
        self.nodes.insert(new_id, Box::new(new_node));
        if let Some(parent) = maybe_parent {
            if let Some(parent_entry) = self.links.get_mut(&parent) {
                parent_entry.push(new_id);
            } else {
                self.links.insert(parent, vec![new_id]);
            }
        }
        Ok(new_id)
    }
}
