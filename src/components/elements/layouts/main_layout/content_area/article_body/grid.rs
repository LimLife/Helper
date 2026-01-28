mod data_type_card;

use dioxus::prelude::*;
use super::grid::data_type_card::DataTypeCard;
use super::info_box::InfoBox;
use super::warning_box::WarningBox;

#[component]
pub fn Grid()->Element{
    rsx!{
        div { class: "artical-body-grid-wraper",
            DataTypeCard {}
            InfoBox {}
            WarningBox {}
        }
    }
}