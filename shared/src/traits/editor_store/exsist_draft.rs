use crate::editor::BlockEditorContext;
use crate::manifesto::{Block, IntoBlock};
use crate::new_type_id::{ArticleID, ContentID};
use crate::store::Draft;
pub trait ExsistDraft {
    fn create_or_update<T>(&mut self, context: BlockEditorContext, block: T)
    where
        T: IntoBlock;
    fn draft(&self, id: ContentID) -> Option<&Draft>;
    fn draft_block(&self, id: ContentID) -> Option<&Block>;
    fn draft_as<T>(&self, id: ContentID) -> Option<T>
    where
        T: TryFrom<Block>;
    fn set_draft<T>(&mut self, order: u32, article_id: ArticleID, content_id: ContentID, data: T)
    where
        T: IntoBlock;
    fn has_draft(&self, id: ContentID) -> bool;
    fn update_draft<T>(&mut self, id: ContentID, data: T) -> bool
    where
        T: IntoBlock;
    fn take_draft(&mut self, id: ContentID) -> Option<Draft>;

    fn discard_draft(&mut self, id: ContentID);
    fn clear_draft(&mut self);
}
