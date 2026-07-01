use crate::components::elements::shared_elements::info_box::InfoBox;
use dioxus::prelude::*;
use shared::data_block::InfoData;
#[component]
pub fn InfoBoxEditor(info_block: InfoData) -> Element {
    rsx! {
       InfoBox{info_block}
    }
}
