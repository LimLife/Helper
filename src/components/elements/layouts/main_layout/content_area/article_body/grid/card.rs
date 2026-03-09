mod type_card;

use crate::components::shared::card_type_date::CardDate;

use dioxus::prelude::*;
use type_card::TypeCard;
#[component]
pub fn CardGrid(card_types: Vec<CardDate>) -> Element {
    rsx! {
        ul {
            for card in card_types {
                li {
                    "{card.label}"

                    if let Some(types) = card.typs {
                        ul {
                            for item in types {
                                li {
                                    TypeCard { card: item }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
