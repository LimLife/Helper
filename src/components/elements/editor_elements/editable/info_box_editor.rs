use crate::components::elements::shared_elements::info_box::InfoBox;
use dioxus::prelude::*;
use shared::data_block::InfoData;
use shared::new_type_id::ContentID;
#[component]
pub fn InfoBoxEditor(info_data: InfoData, id: ContentID) -> Element {
    rsx! {
        div {
            id: "info-box-editor",
            class: "info-box-editor",
            InfoBox{info_data}
        }
    }
}
