use std::collections::{HashMap, HashSet};

use crate::{
    manifesto::{Article, ArticleBody, Block, Content, IntoBlock, NodeMeta, NodeType},
    new_type_id::{ArticleID, ContentID, NodeMetaID},
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

#[derive(Debug, Default, PartialEq, Eq)]
pub struct ArticleStore {
    pub articles: HashMap<ArticleID, Article>,
    pub bodies: HashMap<ArticleID, ArticleBody>,
    pub current: Option<ArticleID>,
}

#[derive(Clone, Default)]
pub struct EditorStore {
    pub selected: Option<ContentID>,
    pub drafts: HashMap<ContentID, Block>,
}
impl EditorStore {
    pub fn set_draft<T>(&mut self, id: ContentID, data: T)
    where
        T: IntoBlock,
    {
        self.drafts.insert(id, data.into_block());
    }
    pub fn select(&mut self, contnet_id: ContentID) {
        self.selected = Some(contnet_id);
    }
    pub fn deselect(&mut self) {
        self.selected = None;
    }
    pub fn is_select(&self, id: ContentID) -> bool {
        self.selected == Some(id)
    }
    pub fn toggel(&mut self, id: ContentID) {
        if self.is_select(id) {
            self.deselect();
        } else {
            self.select(id);
        }
    }

    pub fn draft(&self, id: ContentID) -> Option<&Block> {
        self.drafts.get(&id)
    }
    pub fn has_draft(&self, id: ContentID) -> bool {
        self.drafts.contains_key(&id)
    }
    pub fn discard_draft(&mut self, id: ContentID) {
        self.drafts.remove(&id);
    }
    pub fn clear_draft(&mut self) {
        self.drafts.clear();
    }
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

impl ArticleStore {
    pub fn clear(&mut self) {
        self.articles.clear();
        self.bodies.clear();
        self.current = None;
    }

    pub fn open(&mut self, id: ArticleID) {
        self.current = Some(id);
    }

    pub fn current(&self) -> Option<ArticleID> {
        self.current
    }

    pub fn has_article(&self, id: ArticleID) -> bool {
        self.articles.contains_key(&id)
    }

    pub fn has_body(&self, id: ArticleID) -> bool {
        self.bodies.contains_key(&id)
    }
    pub fn insert_article(&mut self, article: Article) {
        self.articles.insert(article.id, article);
    }

    pub fn insert_body(&mut self, body: ArticleBody) {
        self.bodies.insert(body.article_id, body);
    }
    pub fn current_body(&self) -> Option<&ArticleBody> {
        let id = self.current?;
        self.bodies.get(&id)
    }
    pub fn current_article(&self) -> Option<&Article> {
        let id = self.current?;
        self.articles.get(&id)
    }

    pub fn current_content(&self, id: ContentID) -> Option<&Content> {
        let body = self.current_body()?;
        body.bloks.iter().find(|content| content.id == id)
    }

    pub fn current_content_mut(&mut self, id: ContentID) -> Option<&mut Content> {
        let article = self.current()?;
        let body = self.bodies.get_mut(&article)?;
        body.bloks.iter_mut().find(|content| content.id == id)
    }

    pub fn current_block(&self, id: ContentID) -> Option<&Block> {
        self.current_content(id).map(|block| &block.block)
    }

    pub fn current_block_mut(&mut self, id: ContentID) -> Option<&mut Block> {
        self.current_content_mut(id).map(|block| &mut block.block)
    }
}
