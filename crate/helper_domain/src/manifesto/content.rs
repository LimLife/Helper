use serde::{Deserialize, Serialize};

use crate::{manifesto::block::Block, type_id::ContentID};

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
pub struct Content {
    pub id: ContentID,
    pub order: u32,
    pub block: Block,
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
