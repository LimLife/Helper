mod card;
mod data_type_card;
use super::grid::data_type_card::DataTypeCard;
use super::info_box::InfoBox;
use super::warning_box::WarningBox;
use dioxus::prelude::*;

#[component]
pub fn Grid() -> Element {
    rsx! {
        section { class: "artical-body-grid-wraper",
            DataTypeCard {}
            InfoBox {}
            WarningBox {}
        }
    }
}
