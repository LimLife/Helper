use dioxus::prelude::*;
use domain::{data::WarningData, id::ContentID};

use crate::features::articel::block::warning_box::WarningBox;

#[component]
pub fn WarningEditor(warning_data: WarningData, id: ContentID) -> Element {
    rsx! {
        div { id: "warning-editor", class: "warning-editor",
            WarningBox { warning_data }
        }
    }
}
