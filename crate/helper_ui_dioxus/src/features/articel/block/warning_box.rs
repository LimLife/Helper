use dioxus::prelude::*;
use domain::data::WarningData;
#[component]
pub fn WarningBox(warning_data: WarningData) -> Element {
    rsx! {
        aside { class: "artical-body-warning-box", {{ warning_data.warning.unwrap_or_default() }} }
    }
}
