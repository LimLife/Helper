use std::collections::HashMap;

use domain::{
    data::{Article, ArticleBody, Block, Content, InsertPosition},
    id::{ArticleID, ContentID},
};

use crate::drafts::Draft;

#[derive(Debug, Default, PartialEq, Eq)]
pub struct ArticleStore {
    pub articles: HashMap<ArticleID, Article>,
    pub bodies: HashMap<ArticleID, ArticleBody>,
    pub current: Option<ArticleID>,
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
