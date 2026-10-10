use crate::components::elements::shared_elements::info_box::InfoBox;
use dioxus::prelude::*;
use domain::{data::InfoData, id::ContentID};

#[component]
pub fn InfoBoxEditor(info_data: InfoData, id: ContentID) -> Element {
    rsx! {
        div { id: "info-box-editor", class: "info-box-editor",
            InfoBox { info_data }
        }
    }
}
