use crate::components::shared::card_type_date::CardTypeDate;
use dioxus::prelude::*;
#[component]
pub fn TypeCard(card: CardTypeDate) -> Element {
    rsx! {
        div {
            span { {card.type_name} }
            span { {card.type_description} }
        }
    }
}
