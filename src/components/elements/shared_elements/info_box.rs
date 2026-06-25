use dioxus::prelude::*;
use shared::data_block::InfoData;

#[component]
pub fn InfoBox(info_block: InfoData) -> Element {
    rsx! {
        aside { class: "artical-body-info-box",
            { info_block.info.unwrap_or_default()}
        }
    }
}
