use std::collections::HashMap;

use domain::{
    data::{Block, InsertPosition, IntoBlock},
    id::{ArticleID, ContentID},
};

use crate::{
    block_editor::BlockEditorContext,
    drafts::{Draft, NewDraftContent},
    traits::editor_store::{exsist_draft::ExsistDraft, new_draft::NewDraft, select::Select},
};

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
