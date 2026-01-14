use dioxus::prelude::*;

#[component]
pub fn ViewAllCommentsLink() -> Element {
    rsx!{
        a { href: "#", "View All Comments" }
    }
}