use dioxus::prelude::*;

use crate::components::assets::images::USER;

#[component]
pub fn UserAvatarMenu() -> Element {
    rsx!{
        div { id: "user-avatar-menu", class: "user-avatar-menu",
            img { src: USER, alt: "user" }
        }
    }
}