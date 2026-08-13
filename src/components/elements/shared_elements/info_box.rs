use dioxus::prelude::*;
use shared::data_block::InfoData;

#[component]
pub fn InfoBox(info_data: InfoData) -> Element {
    rsx! {
        aside { class: "artical-body-info-box",
            { info_data.info.unwrap_or_default()}
        }
    }
}
