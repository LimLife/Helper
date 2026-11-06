use dioxus::prelude::*;
use domain::{data::InfoData, id::ContentID};

use crate::features::articel::block::info_box::InfoBox;

#[component]
pub fn InfoEditor(info_data: InfoData, id: ContentID) -> Element {
    rsx! {
        div { id: "info-box-editor", class: "info-box-editor",
            InfoBox { info_data }
        }
    }
}
