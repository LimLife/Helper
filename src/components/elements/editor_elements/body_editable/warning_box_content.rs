use crate::components::elements::shared_elements::warning_box::WarningBox;
use dioxus::prelude::*;
use domain::{data::WarningData, id::ContentID};

#[component]
pub fn WarningBoxEditor(warning_data: WarningData, id: ContentID) -> Element {
    rsx! {
        div { id: "warning-editor", class: "warning-editor",
            WarningBox { warning_data }
        }
    }
}
