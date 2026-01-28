mod external_link_card;
mod resources_header;

use dioxus::prelude::*;
use super::external_resources::{
    external_link_card::ExternalLinkCard,
    resources_header::ResourcesHeader
};

#[component]
pub fn ExternalResources() -> Element {
    rsx!{
        div { class: "external-resources",
            ResourcesHeader {}
            ExternalLinkCard {}
        }
    }
}