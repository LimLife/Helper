use core::fmt;
use std::fmt::Display;

pub mod article;
pub mod article_body;
pub mod content;
pub mod node;
#[derive(Debug)]
pub enum ContentError {
    Transport(String),
    NotFound,
    Internal(String),
    InvalidData(String),
}
impl Display for ContentError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ContentError::Transport(msg) => write!(f, "{:?}", msg),
            ContentError::NotFound => write!(f, "Content not found"),
            ContentError::Internal(msg) => write!(f, "{:?}", msg),
            ContentError::InvalidData(msg) => write!(f, "{:?}", msg),
        }
    }
}
