mod external_link;
use dioxus::prelude::*;

use super::external_link_card::external_link::ExternalLink;
#[component]
pub fn ExternalLinkCard() -> Element {
    rsx!{
        ul {
            ExternalLink { link: "Git" }
            ExternalLink { link: "Habr" }
            ExternalLink { link: "Another Link" }
        }
    }
}
