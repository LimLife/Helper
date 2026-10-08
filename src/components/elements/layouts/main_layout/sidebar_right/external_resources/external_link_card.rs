mod external_link;
use super::external_link_card::external_link::ExternalLink;
use dioxus::prelude::*;
#[component]
pub fn ExternalLinkCard() -> Element {
    rsx! {
        ul {
            ExternalLink { link: "Git" }
            ExternalLink { link: "Habr" }
            ExternalLink { link: "Another Link" }
        }
    }
}
