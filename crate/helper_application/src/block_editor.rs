use domain::{
    data::InsertPosition,
    id::{ArticleID, ContentID},
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
