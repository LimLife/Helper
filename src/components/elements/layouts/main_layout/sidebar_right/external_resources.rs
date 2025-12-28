mod external_link_card;

use dioxus::prelude::*;
use super::external_resources::{
    external_link_card::ExternalLinkCard,
};

#[component]
pub fn ExternalResources() -> Element {
    rsx!{
        ExternalLinkCard {}
    }
}