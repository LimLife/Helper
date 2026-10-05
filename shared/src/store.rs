use crate::{
    editor::{BlockEditorContext, NewDraftContent},
    manifesto::{
        Article, ArticleBody, Block, Content, InsertPosition, IntoBlock, NodeMeta, NodeType,
    },
    new_type_id::{ArticleID, ContentID, NodeMetaID},
    traits::editor_store::{exsist_draft::ExsistDraft, new_draft::NewDraft, select::Select},
};
use std::collections::{HashMap, HashSet};
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
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Draft {
    pub content_id: ContentID,
    pub article_id: ArticleID,
    pub order: u32,
    pub block: Block,
}
#[derive(Clone, Default)]
pub struct EditorStore {
    pub selected: Option<ContentID>,
    pub drafts: HashMap<ContentID, Draft>,
    pub new_draft: Option<NewDraftContent>,
    pub current: Option<ArticleID>,
}

impl EditorStore {
    pub fn set_new_draft(&mut self, draft: NewDraftContent) {
        self.new_draft = Some(draft);
    }
}

impl Select for EditorStore {
    fn select(&mut self, content_id: ContentID) {
        self.selected = Some(content_id);
    }

    fn deselect(&mut self) {
        self.selected = None;
    }

    fn is_select(&self, id: ContentID) -> bool {
        self.selected == Some(id)
    }

    fn toggle(&mut self, id: ContentID) {
        if self.is_select(id) {
            self.deselect();
        } else {
            self.select(id);
        }
    }
}
impl ExsistDraft for EditorStore {
    fn set_draft<T>(&mut self, order: u32, article_id: ArticleID, content_id: ContentID, data: T)
    where
        T: IntoBlock,
    {
        self.drafts.insert(
            content_id,
            Draft {
                article_id,
                content_id,
                order,
                block: data.into_block(),
            },
        );
    }
    fn create_or_update<T>(&mut self, context: BlockEditorContext, block: T)
    where
        T: IntoBlock,
    {
        let block = block.into_block();

        match context {
            BlockEditorContext::Existing {
                article_id,
                content_id,
                order,
            } => {
                if let Some(draft) = self.drafts.get_mut(&content_id) {
                    draft.block = block;
                } else {
                    self.drafts.insert(
                        content_id,
                        Draft {
                            article_id,
                            content_id,
                            order,
                            block,
                        },
                    );
                }
            }

            BlockEditorContext::New {
                article_id,
                position,
            } => {
                self.new_draft = Some(NewDraftContent {
                    article_id,
                    position,
                    block,
                });
            }
        }
    }

    fn update_draft<T>(&mut self, id: ContentID, data: T) -> bool
    where
        T: IntoBlock,
    {
        let Some(draft) = self.drafts.get_mut(&id) else {
            return false;
        };
        draft.block = data.into_block();
        true
    }

    fn draft_block(&self, id: ContentID) -> Option<&Block> {
        self.drafts.get(&id).map(|draft| &draft.block)
    }

    fn draft_as<T>(&self, id: ContentID) -> Option<T>
    where
        T: TryFrom<Block>,
    {
        self.draft(id)
            .map(|draft| draft.block.clone())
            .and_then(|block| T::try_from(block).ok())
    }
    fn draft(&self, id: ContentID) -> Option<&Draft> {
        self.drafts.get(&id)
    }
    fn has_draft(&self, id: ContentID) -> bool {
        self.drafts.contains_key(&id)
    }

    fn take_draft(&mut self, id: ContentID) -> Option<Draft> {
        self.drafts.remove(&id)
    }

    fn discard_draft(&mut self, id: ContentID) {
        self.drafts.remove(&id);
    }
    fn clear_draft(&mut self) {
        self.drafts.clear();
    }
}
impl NewDraft for EditorStore {
    fn new_block(&self) -> Option<&Block> {
        self.new_draft.as_ref().map(|draft| draft.block())
    }

    fn start_new(&mut self, position: InsertPosition, article_id: ArticleID, block: Block) {
        self.new_draft = Some(NewDraftContent {
            position,
            block,
            article_id,
        })
    }

    fn new_draft(&self) -> Option<&NewDraftContent> {
        self.new_draft.as_ref()
    }

    fn new_draft_mut(&mut self) -> Option<&mut NewDraftContent> {
        self.new_draft.as_mut()
    }

    fn update_new_block<T>(&mut self, data: T) -> bool
    where
        T: IntoBlock,
    {
        let Some(draft) = self.new_draft.as_mut() else {
            return false;
        };
        draft.set_block(data.into_block());
        true
    }

    fn take_new_draft(&mut self) -> Option<NewDraftContent> {
        self.new_draft.take()
    }

    fn cancel_new_draft(&mut self) {
        self.new_draft = None
    }

    fn has_new_draft(&self) -> bool {
        self.new_draft.is_some()
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
    pub fn draft(&self, content_id: ContentID) -> Option<Draft> {
        let body = self.current_body()?;
        let content = body
            .blocks
            .iter()
            .find(|content| content.id == content_id)?;

        Some(Draft {
            article_id: body.article_id,
            content_id: content.id,
            order: content.order,
            block: content.block.clone(),
        })
    }
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
        body.blocks.iter().find(|content| content.id == id)
    }
    pub fn current_content_mut(&mut self, id: ContentID) -> Option<&mut Content> {
        let article = self.current()?;
        let body = self.bodies.get_mut(&article)?;
        body.blocks.iter_mut().find(|content| content.id == id)
    }
    pub fn current_block(&self, id: ContentID) -> Option<&Block> {
        self.current_content(id).map(|block| &block.block)
    }
    pub fn current_block_mut(&mut self, id: ContentID) -> Option<&mut Block> {
        self.current_content_mut(id).map(|block| &mut block.block)
    }
    pub fn delete_content(&mut self, id: ContentID) -> Option<Content> {
        let article_id = self.current?;
        let body = self.bodies.get_mut(&article_id)?;
        body.remove(id)
    }
    pub fn duplicate_content(&mut self, id: ContentID) -> Option<ContentID> {
        let article_id = self.current?;
        let body = self.bodies.get_mut(&article_id)?;
        body.duplicate(id)
    }
    pub fn move_content_down(&mut self, id: ContentID) -> bool {
        let Some(article_id) = self.current() else {
            return false;
        };
        let Some(body) = self.bodies.get_mut(&article_id) else {
            return false;
        };
        body.move_down(id)
    }
    pub fn move_content_up(&mut self, id: ContentID) -> bool {
        let Some(article_id) = self.current else {
            return false;
        };
        let Some(body) = self.bodies.get_mut(&article_id) else {
            return false;
        };
        body.move_up(id)
    }
    pub fn insert_content(&mut self, position: InsertPosition, block: Block) -> Option<ContentID> {
        let article_id = self.current?;
        let body = self.bodies.get_mut(&article_id)?;
        body.insert(position, block)
    }
    #[deprecated(note = "старый способ")]
    pub fn insert_content_above(&mut self, target: ContentID, block: Block) -> Option<ContentID> {
        let article_id = self.current()?;
        let body = self.bodies.get_mut(&article_id)?;
        body.insert_above(target, block)
    }
    #[deprecated(note = "старый способ")]
    pub fn insert_content_below(&mut self, target: ContentID, block: Block) -> Option<ContentID> {
        let article_id = self.current()?;
        let body = self.bodies.get_mut(&article_id)?;
        body.insert_below(target, block)
    }
}
