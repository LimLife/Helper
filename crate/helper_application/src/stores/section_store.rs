use std::collections::{HashMap, HashSet};

use domain::{
    data::{NodeMeta, NodeType},
    id::{ArticleID, NodeMetaID},
};

#[derive(Debug, Default, PartialEq, Eq, Clone)]
pub struct SectionStore {
    pub root: Vec<NodeMetaID>,
    pub nodes: HashMap<NodeMetaID, NodeMeta>,
    pub children: HashMap<NodeMetaID, Vec<NodeMetaID>>,
    pub loaded: HashSet<NodeMetaID>,
    pub expand: HashSet<NodeMetaID>,
    pub selected: Option<NodeMetaID>,
}

impl SectionStore {
    pub fn clear(&mut self) {
        self.root.clear();
        self.nodes.clear();
        self.children.clear();
        self.loaded.clear();
        self.expand.clear();
        self.selected = None;
    }

    pub fn is_root(&self) -> bool {
        self.root.is_empty()
    }

    pub fn init_root(&mut self, nodes: Vec<NodeMeta>) {
        self.root.clear();
        for node in nodes {
            let id = node.id;
            self.root.push(id);
            self.nodes.insert(id, node);
        }
    }

    pub fn insert_children(&mut self, parent_id: NodeMetaID, nodes: Vec<NodeMeta>) {
        let mut ids = Vec::new();
        for node in nodes {
            let id = node.id;
            ids.push(id);
            self.nodes.insert(id, node);
        }
        self.children.insert(parent_id, ids);
        self.loaded.insert(parent_id);
    }

    pub fn children_of(&self, parent_id: NodeMetaID) -> &[NodeMetaID] {
        self.children
            .get(&parent_id)
            .map(Vec::as_slice)
            .unwrap_or(&[])
    }

    pub fn article_id(&self, node_id: NodeMetaID) -> Option<ArticleID> {
        self.nodes
            .get(&node_id)
            .and_then(|node| match node.type_node {
                NodeType::Article {
                    article_id: artical_id,
                } => Some(artical_id),
                _ => None,
            })
    }

    pub fn get(&self, id: NodeMetaID) -> Option<&NodeMeta> {
        self.nodes.get(&id)
    }

    pub fn set_selected(&mut self, id: NodeMetaID) {
        self.selected = Some(id);
    }

    pub fn is_selected(&self, id: NodeMetaID) -> bool {
        self.selected == Some(id)
    }

    pub fn is_expanded(&self, id: &NodeMetaID) -> bool {
        self.expand.contains(&id)
    }

    pub fn expand(&mut self, id: NodeMetaID) {
        self.expand.insert(id);
    }

    pub fn collapse(&mut self, id: NodeMetaID) {
        self.expand.remove(&id);
    }

    pub fn toggle_expand(&mut self, id: NodeMetaID) {
        if self.expand.contains(&id) {
            self.expand.remove(&id);
        } else {
            self.expand.insert(id);
        }
    }

    pub fn is_loaded(&self, id: NodeMetaID) -> bool {
        self.loaded.contains(&id)
    }

    pub fn mark_loaded(&mut self, id: NodeMetaID) {
        self.loaded.insert(id);
    }
}
