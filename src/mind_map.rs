use std::{collections::HashMap, hash::Hash};

use log::info;

use crate::components::{MMItem, MMItemId};

pub(crate) struct MindMap {
    // Stores actual instances of nodes.
    // The items have to be boxed so that we can treat them as `dyn Renderizable` in layout.
    pub(crate) nodes: HashMap<MMItemId, Box<MMItem>>,

    // Links items to children
    pub(crate) children: HashMap<MMItemId, Vec<MMItemId>>,

    // A node may have exactly one parent.
    // Root nodes have no parent.
    pub(crate) parents: HashMap<MMItemId, Option<MMItemId>>,
}

impl MindMap {
    pub(crate) fn new() -> Self {
        MindMap {
            nodes: HashMap::new(),
            children: HashMap::new(),
            parents: HashMap::new(),
        }
    }

    pub(crate) fn create_node(
        &mut self,
        text: String,
        maybe_parent: Option<MMItemId>,
    ) -> Result<MMItemId, String> {
        let (new_node, new_id) = MMItem::new(text);
        self.nodes.insert(new_id, Box::new(new_node));
        self.parents.insert(new_id, maybe_parent);
        if let Some(parent) = maybe_parent {
            if let Some(parent_entry) = self.children.get_mut(&parent) {
                parent_entry.push(new_id);
            } else {
                self.children.insert(parent, vec![new_id]);
            }
        }
        Ok(new_id)
    }

    pub(crate) fn roots(&self) -> Vec<MMItemId> {
        self.parents
            .iter()
            .filter(|(_, parent)| parent.is_none())
            .map(|(node, _)| node.clone())
            .collect()
    }

    pub(crate) fn children(&self, of: MMItemId) -> Vec<MMItemId> {
        info!("Getting children of {}: {:?}", of, self.children.get(&of));
        if let Some(children) = self.children.get(&of) {
            children.to_vec()
        } else {
            vec![]
        }
    }

    pub(crate) fn item(&self, id: &MMItemId) -> Result<&Box<MMItem>, String> {
        self.nodes
            .get(id)
            .ok_or("Error retrieving item".to_string())
    }
}
