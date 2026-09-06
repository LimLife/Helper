use super::data_block::*;
use super::new_type_id::{ArticleID, ContentID, NodeMetaID};
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
//temp-remove-future
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
    pub bloks: Vec<Content>,
}
impl ArticleBody {
    pub fn new(id: ArticleID) -> Self {
        Self {
            article_id: id,
            bloks: vec![],
        }
    }

    pub fn len(self) -> usize {
        self.bloks.len()
    }

    pub fn is_empty(&self) -> bool {
        self.bloks.is_empty()
    }

    pub fn contnet(&self, id: ContentID) -> Option<&Content> {
        self.bloks.iter().find(|content| content.id == id)
    }

    pub fn content_mut(&mut self, id: ContentID) -> Option<&mut Content> {
        self.bloks.iter_mut().find(|content| content.id == id)
    }

    pub fn contains(&self, id: ContentID) -> bool {
        self.contnet(id).is_some()
    }

    pub fn normalaze_order(&mut self) {
        let mut includes: Vec<usize> = (0..self.bloks.len()).collect();

        includes.sort_by_key(|&index| self.bloks[index].order);
        for (position, index) in includes.into_iter().enumerate() {
            self.bloks[index].order = ((position + 1) as u32) * 10;
        }
    }

    pub fn push(&mut self, block: Block) -> ContentID {
        let order = self.next_order();
        let content = Content::new(block, order);

        let id = content.id;
        self.bloks.push(content);

        id
    }
    fn next_order(&self) -> u32 {
        self.bloks
            .iter()
            .map(|content| content.order)
            .max()
            .map(|order| order.saturating_add(10))
            .unwrap_or(10)
    }

    pub fn remove(&mut self, id: ContentID) -> Option<Content> {
        let index = self.index_of(id)?;
        Some(self.bloks.remove(index))
    }
    pub fn move_up(&mut self, id: ContentID) -> bool {
        let Some(curren_index) = self.index_of(id) else {
            return false;
        };
        let curent_order = self.bloks[curren_index].order;
        let Some(previous_index) = self
            .bloks
            .iter()
            .enumerate()
            .filter(|(_, content)| content.order < curent_order)
            .max_by_key(|(_, content)| content.order)
            .map(|(index, _)| index)
        else {
            return false;
        };
        let previous_order = self.bloks[previous_index].order;

        self.bloks[curren_index].order = previous_order;
        self.bloks[previous_index].order = curent_order;
        true
    }
    pub fn move_down(&mut self, id: ContentID) -> bool {
        let Some(curren_index) = self.index_of(id) else {
            return false;
        };
        let curent_order = self.bloks[curren_index].order;
        let Some(next_index) = self
            .bloks
            .iter()
            .enumerate()
            .filter(|(_, content)| content.order > curent_order)
            .max_by_key(|(_, content)| content.order)
            .map(|(index, _)| index)
        else {
            return false;
        };
        let next_order = self.bloks[next_index].order;

        self.bloks[curren_index].order = next_order;
        self.bloks[next_index].order = curent_order;
        true
    }

    pub fn dublicate(&mut self, id: ContentID) -> Option<ContentID> {
        if let Some(id) = self.try_dublicate(id) {
            return Some(id);
        }
        self.normalaze_order();
        self.try_dublicate(id)
    }
    pub fn insert_above(&mut self, target: ContentID, block: Block) -> Option<ContentID> {
        if let Some(id) = self.try_insert_above(target, &block) {
            return Some(id);
        }
        self.normalaze_order();
        self.try_insert_below(target, &block)
    }
    pub fn insert_below(&mut self, target: ContentID, block: Block) -> Option<ContentID> {
        if let Some(id) = self.try_insert_below(target, &block) {
            return Some(id);
        }
        self.normalaze_order();
        self.try_insert_below(target, &block)
    }
    fn try_dublicate(&mut self, id: ContentID) -> Option<ContentID> {
        let current_order = self.contnet(id)?.order;

        let next_order = self
            .bloks
            .iter()
            .filter(|content| content.order > current_order)
            .map(|content| content.order)
            .min();

        let new_orader = match next_order {
            Some(next) => Self::beetween(current_order, next)?,
            None => current_order.checked_add(10)?,
        };

        let index = self.index_of(id)?;
        let mut dublicate = self.bloks[index].clone();

        dublicate.id = ContentID::new();
        dublicate.order = new_orader;
        let new_id = dublicate.id;

        self.bloks.push(dublicate);

        Some(new_id)
    }
    fn try_insert_above(&mut self, target: ContentID, block: &Block) -> Option<ContentID> {
        let target_order = self.contnet(target)?.order;
        let previous_order = self
            .bloks
            .iter()
            .filter(|content| content.order < target_order)
            .map(|content| content.order)
            .max();
        let new_order = match previous_order {
            Some(previous) => Self::beetween(previous, target_order),
            None => Some(target_order / 2),
        }?;
        let content = Content::new(block.clone(), new_order);
        let id = content.id;
        self.bloks.push(content);
        Some(id)
    }
    fn try_insert_below(&mut self, target: ContentID, block: &Block) -> Option<ContentID> {
        let target_order = self.contnet(target)?.order;
        let previous_order = self
            .bloks
            .iter()
            .filter(|content| content.order > target_order)
            .map(|content| content.order)
            .min();
        let new_order = match previous_order {
            Some(previous) => Self::beetween(previous, target_order),
            None => target_order.checked_add(10),
        }?;
        let content = Content::new(block.clone(), new_order);
        let id = content.id;
        self.bloks.push(content);
        Some(id)
    }
    fn beetween(a: u32, b: u32) -> Option<u32> {
        if b <= a + 1 {
            return None;
        }
        Some(a + (b - a) / 2)
    }

    fn index_of(&self, id: ContentID) -> Option<usize> {
        self.bloks.iter().position(|content| content.id == id)
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
        self.order = order
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
