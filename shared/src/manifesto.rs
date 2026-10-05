use crate::{
    data_block::*,
    new_type_id::{ArticleID, ContentID, NodeMetaID},
};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct RootManifest {
    pub version_bar: u32,
    pub site_title: String,
    pub root_sections: Vec<NodeMeta>,
}
#[derive(Debug, Clone, Deserialize, Serialize, Default, PartialEq, Eq)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum NodeType {
    #[default]
    Section,
    Article {
        article_id: ArticleID,
    },
}
#[derive(Debug, Clone, Deserialize, Serialize, Default, PartialEq, Eq)]
pub struct NodeMeta {
    pub id: NodeMetaID,
    pub parent_id: Option<NodeMetaID>,
    pub sort_order: u32,
    pub title: String,
    pub icon: Option<String>,
    pub description: Option<String>,
    pub has_children: bool,
    pub type_node: NodeType,
}
impl NodeMeta {
    pub fn new() -> Self {
        Self {
            id: NodeMetaID::new(),
            parent_id: Default::default(),
            sort_order: Default::default(),
            title: Default::default(),
            icon: Default::default(),
            description: Default::default(),
            has_children: Default::default(),
            type_node: Default::default(),
        }
    }
}
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
pub struct Article {
    pub id: ArticleID,
    pub title: String,
    pub author: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub tags: Vec<String>,
}
impl Article {
    pub fn new(title: String, author: String) -> Self {
        Self {
            id: ArticleID::new(),
            title,
            author,
            created_at: Default::default(),
            updated_at: Default::default(),
            tags: vec![],
        }
    }
}
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
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
pub struct Content {
    pub id: ContentID,
    pub order: u32,
    pub block: Block,
}
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum Block {
    Code(CodeData),
    Warning(WarningData),
    Section(SectionData),
    Info(InfoData),
}
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum InsertPosition {
    First,
    Above(ContentID),
    Below(ContentID),
    End,
}
impl Block {
    pub fn empty_code() -> Self {
        Self::Code(CodeData::default())
    }
    pub fn empty_warning() -> Self {
        Self::Warning(WarningData::default())
    }
    pub fn empty_section() -> Self {
        Self::Section(SectionData::default())
    }
    pub fn empty_info() -> Self {
        Self::Info(InfoData::default())
    }
}
impl Content {
    pub fn new(block: Block, order: u32) -> Self {
        Self {
            id: ContentID::new(),
            order,
            block,
        }
    }
    pub fn id(&self) -> ContentID {
        self.id
    }
    pub fn order(&self) -> u32 {
        self.order
    }
    pub fn block(&self) -> &Block {
        &self.block
    }
    pub fn block_mut(&mut self) -> &mut Block {
        &mut self.block
    }
    pub fn set_order(&mut self, order: u32) {
        self.order = order;
    }
}
pub trait IntoBlock {
    fn into_block(self) -> Block;
}
impl TryFrom<Block> for CodeData {
    type Error = ();
    fn try_from(value: Block) -> Result<Self, Self::Error> {
        match value {
            Block::Code(data) => Ok(data),
            _ => Err(()),
        }
    }
}
impl IntoBlock for CodeData {
    fn into_block(self) -> Block {
        Block::Code(self)
    }
}
impl TryFrom<Block> for InfoData {
    type Error = ();
    fn try_from(value: Block) -> Result<Self, Self::Error> {
        match value {
            Block::Info(data) => Ok(data),
            _ => Err(()),
        }
    }
}
impl IntoBlock for InfoData {
    fn into_block(self) -> Block {
        Block::Info(self)
    }
}
impl TryFrom<Block> for SectionData {
    type Error = ();
    fn try_from(value: Block) -> Result<Self, Self::Error> {
        match value {
            Block::Section(data) => Ok(data),
            _ => Err(()),
        }
    }
}
impl IntoBlock for SectionData {
    fn into_block(self) -> Block {
        Block::Section(self)
    }
}
impl TryFrom<Block> for WarningData {
    type Error = ();
    fn try_from(value: Block) -> Result<Self, Self::Error> {
        match value {
            Block::Warning(data) => Ok(data),
            _ => Err(()),
        }
    }
}
impl IntoBlock for WarningData {
    fn into_block(self) -> Block {
        Block::Warning(self)
    }
}
