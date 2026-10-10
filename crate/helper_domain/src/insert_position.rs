use serde::{Deserialize, Serialize};

use crate::type_id::ContentID;

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum InsertPosition {
    First,
    Above(ContentID),
    Below(ContentID),
    End,
}
