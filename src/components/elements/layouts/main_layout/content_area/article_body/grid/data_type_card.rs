use dioxus::prelude::*;

use super::card::CardGrid;
use crate::components::fake_data::card_data::cards;

#[component]
pub fn DataTypeCard() -> Element {
    let card_types = cards();
    rsx! {
        div { class: "artical-body-grid", CardGrid { card_types } }
    }
}
