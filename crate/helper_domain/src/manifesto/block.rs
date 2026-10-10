use serde::{Deserialize, Serialize};

use crate::data_block::{CodeData, InfoData, SectionData, WarningData};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum Block {
    Code(CodeData),
    Warning(WarningData),
    Section(SectionData),
    Info(InfoData),
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
