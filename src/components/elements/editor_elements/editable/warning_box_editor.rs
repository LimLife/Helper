use crate::components::elements::shared_elements::warning_box::WarningBox;
use dioxus::prelude::*;
use shared::data_block::WarningData;
use shared::new_type_id::ContentID;

#[component]
pub fn WarningBoxEditor(warnig_data: WarningData, id: ContentID) -> Element {
    rsx! {
        div {
            id: "warning-editor",
            class: "warning-editor",
        WarningBox{warnig_data}
        }
    }
}
