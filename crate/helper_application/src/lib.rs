pub mod app_service;
mod block_editor;
pub mod config;
mod drafts;
mod gateways;
mod repositories;
mod services;
mod stores;
mod traits;
pub mod store {
    pub use crate::stores::{
        article_store::ArticleStore, editor_store::EditorStore, section_store::SectionStore,
    };
    pub use crate::traits::editor_store::{
        exsist_draft::ExsistDraft, new_draft::NewDraft, select::Select,
    };
}
pub mod draft {
    pub use crate::drafts::{Draft, NewDraftContent};
}
pub mod editor_state {
    pub use crate::block_editor::{BlockEditorContext, BlockEditorMode, BlockEditorTarget};
}

pub mod gateway {
    pub use crate::gateways::{
        ContentError, article::ArticleGateway, content::ContentGateway, node::NodeGateway,
    };
}
pub mod service {
    pub use crate::repositories::{
        article::ArticleRepository, content::ContentRepository, node::NodeRepository,
    };
    pub use crate::services::content_service::ContentService;
}
