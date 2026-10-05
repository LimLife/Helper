use crate::{
    manifesto::{Block, InsertPosition},
    new_type_id::{ArticleID, ContentID},
};

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum BlockEditorMode {
    Create(InsertPosition),
    Edit(ContentID),
    EditNew,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum BlockEditorTarget {
    Existing(ContentID, ArticleID),
    New(ArticleID),
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct NewDraftContent {
    pub article_id: ArticleID,
    pub position: InsertPosition,
    pub block: Block,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BlockEditorContext {
    Existing {
        article_id: ArticleID,
        content_id: ContentID,
        order: u32,
    },
    New {
        article_id: ArticleID,
        position: InsertPosition,
    },
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
