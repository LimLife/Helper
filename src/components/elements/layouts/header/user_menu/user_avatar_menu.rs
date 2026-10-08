use crate::components::assets::images::USER;
use dioxus::prelude::*;
#[component]
pub fn UserAvatarMenu() -> Element {
    rsx! {
        div { id: "user-avatar-menu", class: "user-avatar-menu",
            img { src: USER, alt: "user" }
        }
    }
}
