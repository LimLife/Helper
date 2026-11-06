mod external_link_card;
mod resources_header;
use super::external_resources::{
    external_link_card::ExternalLinkCard, resources_header::ResourcesHeader,
};
use dioxus::prelude::*;
#[component]
pub fn ExternalResources() -> Element {
    rsx! {
        div { class: "external-resources",
            ResourcesHeader {}
            ExternalLinkCard {}
        }
    }
}
