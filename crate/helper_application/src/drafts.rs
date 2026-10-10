use domain::{
    data::{Block, InsertPosition},
    id::{ArticleID, ContentID},
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Draft {
    pub content_id: ContentID,
    pub article_id: ArticleID,
    pub order: u32,
    pub block: Block,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct NewDraftContent {
    pub article_id: ArticleID,
    pub position: InsertPosition,
    pub block: Block,
}

impl NewDraftContent {
    pub fn new(position: InsertPosition, block: Block, article_id: ArticleID) -> Self {
        Self {
            position,
            block,
            article_id,
        }
    }

    pub fn position(&self) -> InsertPosition {
        self.position
    }

    pub fn block(&self) -> &Block {
        &self.block
    }

    pub fn block_mut(&mut self) -> &mut Block {
        &mut self.block
    }

    pub fn into_parts(self) -> (InsertPosition, Block) {
        (self.position, self.block)
    }

    pub fn set_block(&mut self, block: Block) {
        self.block = block;
    }
}
