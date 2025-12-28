mod data_type_card;

use dioxus::prelude::*;
use super::grid::data_type_card::DataTypeCard;


#[component]
pub fn Grid()->Element{
    rsx!{
        DataTypeCard {  }
    }
}