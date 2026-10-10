mod block_editor;
mod drafts;
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
