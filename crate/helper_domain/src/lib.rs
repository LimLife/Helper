mod data_block;
mod insert_position;
mod manifesto;
mod type_id;

pub mod data {
    pub use crate::data_block::{
        ArticleHeaderData, ArticleNavigationData, BreadcrumbData, CodeData, InfoData, SectionData,
        WarningData,
    };
    pub use crate::insert_position::InsertPosition;
    pub use crate::manifesto::{
        article::Article,
        article_body::ArticleBody,
        block::{Block, IntoBlock},
        content::Content,
        node::{NodeMeta, NodeType},
        root::RootManifest,
    };
}

pub mod id {
    pub use crate::type_id::{ArticleID, ContentID, NodeMetaID};
}
