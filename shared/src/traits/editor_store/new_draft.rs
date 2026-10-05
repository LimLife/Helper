use crate::{
    editor::NewDraftContent,
    manifesto::{Block, InsertPosition, IntoBlock},
    new_type_id::ArticleID,
};

pub trait NewDraft {
    fn start_new(&mut self, position: InsertPosition, article_id: ArticleID, block: Block);
    fn new_draft(&self) -> Option<&NewDraftContent>;
    fn new_draft_mut(&mut self) -> Option<&mut NewDraftContent>;
    fn new_block(&self) -> Option<&Block>;
    fn update_new_block<T>(&mut self, data: T) -> bool
    where
        T: IntoBlock;
    fn take_new_draft(&mut self) -> Option<NewDraftContent>;
    fn cancel_new_draft(&mut self);
    fn has_new_draft(&self) -> bool;
}
