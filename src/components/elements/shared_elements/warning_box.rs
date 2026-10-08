use dioxus::prelude::*;
use shared::data_block::WarningData;

#[component]
pub fn WarningBox(warnig_data: WarningData) -> Element {
    rsx! {
        aside { class: "artical-body-warning-box", {
            {warnig_data.warning.unwrap_or_default()}
         }
        }
    }
}
