use crate::components::shared::card_type_date::{CardDate, CardTypeDate};

pub fn cards() -> Vec<CardDate> {
    let card_types = vec![CardDate {
        label: "Frontend".to_string(),
        typs: Some(vec![
            CardTypeDate {
                type_name: "React".to_string(),
                type_description: "UI library for building interactive interfaces".to_string(),
                code: None,
            },
            CardTypeDate {
                type_name: "Dioxus".to_string(),
                type_description: "Rust-based UI framework".to_string(),
                code: None,
            },
            CardTypeDate {
                type_name: "Vue".to_string(),
                type_description: "Progressive JavaScript framework".to_string(),
                code: None,
            },
        ]),
    }];
    card_types
}
