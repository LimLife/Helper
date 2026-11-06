use dioxus::prelude::*;
pub mod content_area;
pub mod sidebar_left;
pub mod sidebar_right;

use crate::layouts::main_layout::{
    content_area::ContentArea, sidebar_left::SidebarLeft, sidebar_right::SidebarRight,
};
#[component]
pub fn MainLayout() -> Element {
    rsx! {
        main { id: "main-layout", class: "main-layout",
        SidebarLeft { },
        ContentArea { },
        SidebarRight { }
        }
    }
}
