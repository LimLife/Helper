use crate::components::elements::shared_elements::warning_box::WarningBox;
use dioxus::prelude::*;
use shared::data_block::WarningData;

#[component]
pub fn WarningBoxEditor(warnig_data: WarningData) -> Element {
    rsx! {
        div {
            id: "warning-editor",
            class: "warning-editor",
        WarningBox{warnig_data}
        }
    }
}
