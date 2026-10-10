use serde::{Deserialize, Serialize};

use crate::{
    insert_position::InsertPosition,
    manifesto::{block::Block, content::Content},
    type_id::{ArticleID, ContentID},
};

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
pub struct ArticleBody {
    pub article_id: ArticleID,
    pub blocks: Vec<Content>,
}
impl ArticleBody {
    pub fn new(id: ArticleID) -> Self {
        Self {
            article_id: id,
            blocks: vec![],
        }
    }
    pub fn len(self) -> usize {
        self.blocks.len()
    }
    pub fn is_empty(&self) -> bool {
        self.blocks.is_empty()
    }
    pub fn content(&self, id: ContentID) -> Option<&Content> {
        self.blocks.iter().find(|content| content.id == id)
    }
    pub fn content_mut(&mut self, id: ContentID) -> Option<&mut Content> {
        self.blocks.iter_mut().find(|content| content.id == id)
    }
    pub fn contains(&self, id: ContentID) -> bool {
        self.content(id).is_some()
    }
    pub fn normalize_order(&mut self) {
        let mut includes: Vec<usize> = (0..self.blocks.len()).collect();
        includes.sort_by_key(|&index| self.blocks[index].order);
        for (position, index) in includes.into_iter().enumerate() {
            self.blocks[index].order = ((position + 1) as u32) * 10;
        }
    }
    pub fn push(&mut self, block: Block) -> ContentID {
        let order = self.next_order();
        let content = Content::new(block, order);
        let id = content.id;
        self.blocks.push(content);
        id
    }
    fn next_order(&self) -> u32 {
        self.blocks
            .iter()
            .map(|content| content.order)
            .max()
            .map(|order| order.saturating_add(10))
            .unwrap_or(10)
    }
    pub fn remove(&mut self, id: ContentID) -> Option<Content> {
        let index = self.index_of(id)?;
        Some(self.blocks.remove(index))
    }
    pub fn move_up(&mut self, id: ContentID) -> bool {
        let Some(curren_index) = self.index_of(id) else {
            return false;
        };
        let current_order = self.blocks[curren_index].order;
        let Some(previous_index) = self
            .blocks
            .iter()
            .enumerate()
            .filter(|(_, content)| content.order < current_order)
            .max_by_key(|(_, content)| content.order)
            .map(|(index, _)| index)
        else {
            return false;
        };
        let previous_order = self.blocks[previous_index].order;
        self.blocks[curren_index].order = previous_order;
        self.blocks[previous_index].order = current_order;
        true
    }
    pub fn move_down(&mut self, id: ContentID) -> bool {
        let Some(curren_index) = self.index_of(id) else {
            return false;
        };
        let current_order = self.blocks[curren_index].order;
        let Some(next_index) = self
            .blocks
            .iter()
            .enumerate()
            .filter(|(_, content)| content.order > current_order)
            .max_by_key(|(_, content)| content.order)
            .map(|(index, _)| index)
        else {
            return false;
        };
        let next_order = self.blocks[next_index].order;
        self.blocks[curren_index].order = next_order;
        self.blocks[next_index].order = current_order;
        true
    }
    pub fn duplicate(&mut self, id: ContentID) -> Option<ContentID> {
        if let Some(id) = self.try_duplicate(id) {
            return Some(id);
        }
        self.normalize_order();
        self.try_duplicate(id)
    }
    pub fn insert_above(&mut self, target: ContentID, block: Block) -> Option<ContentID> {
        if let Some(id) = self.try_insert_above(target, &block) {
            return Some(id);
        }
        self.normalize_order();
        self.try_insert_below(target, &block)
    }
    pub fn insert_below(&mut self, target: ContentID, block: Block) -> Option<ContentID> {
        if let Some(id) = self.try_insert_below(target, &block) {
            return Some(id);
        }
        self.normalize_order();
        self.try_insert_below(target, &block)
    }
    fn try_duplicate(&mut self, id: ContentID) -> Option<ContentID> {
        let current_order = self.content(id)?.order;
        let next_order = self
            .blocks
            .iter()
            .filter(|content| content.order > current_order)
            .map(|content| content.order)
            .min();
        let new_order = match next_order {
            Some(next) => Self::between(current_order, next)?,
            None => current_order.checked_add(10)?,
        };
        let index = self.index_of(id)?;
        let mut duplicate = self.blocks[index].clone();
        duplicate.id = ContentID::new();
        duplicate.order = new_order;
        let new_id = duplicate.id;
        self.blocks.push(duplicate);
        Some(new_id)
    }
    pub fn insert(&mut self, position: InsertPosition, block: Block) -> Option<ContentID> {
        match position {
            InsertPosition::First => {
                if self.blocks.is_empty() {
                    Some(self.push(block))
                } else {
                    let first_id = self.blocks.iter().min_by_key(|content| content.order)?.id;
                    self.insert_above(first_id, block)
                }
            }
            InsertPosition::Above(id) => self.insert_above(id, block),
            InsertPosition::Below(id) => self.insert_below(id, block),
            InsertPosition::End => Some(self.push(block)),
        }
    }
    fn try_insert_above(&mut self, target: ContentID, block: &Block) -> Option<ContentID> {
        let target_order = self.content(target)?.order;
        let previous_order = self
            .blocks
            .iter()
            .filter(|content| content.order < target_order)
            .map(|content| content.order)
            .max();
        let new_order = match previous_order {
            Some(previous) => Self::between(previous, target_order)?,
            None => {
                if target_order == 0 {
                    return None;
                }
                target_order / 2
            }
        };
        let content = Content::new(block.clone(), new_order);
        let id = content.id;
        self.blocks.push(content);
        Some(id)
    }
    fn try_insert_below(&mut self, target: ContentID, block: &Block) -> Option<ContentID> {
        let target_order = self.content(target)?.order;
        let previous_order = self
            .blocks
            .iter()
            .filter(|content| content.order > target_order)
            .map(|content| content.order)
            .min();
        let new_order = match previous_order {
            Some(previous) => Self::between(previous, target_order),
            None => target_order.checked_add(10),
        }?;
        let content = Content::new(block.clone(), new_order);
        let id = content.id;
        self.blocks.push(content);
        Some(id)
    }
    fn between(left: u32, right: u32) -> Option<u32> {
        if right <= left || right - left <= 1 {
            return None;
        }
        Some(left + (right - left) / 2)
    }
    fn index_of(&self, id: ContentID) -> Option<usize> {
        self.blocks.iter().position(|content| content.id == id)
    }
}
