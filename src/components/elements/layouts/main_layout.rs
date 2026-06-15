mod content_area;
mod sidebar_left;
mod sidebar_right;

use dioxus::prelude::*;

use super::main_layout::{
    content_area::ContentArea, sidebar_left::SidebarLeft, sidebar_right::SidebarRight,
};
use shared::manifesto::NodeMeta;

#[component]
pub fn MainLayout() -> Element {
    //сдесь должны быть разные масивы и должен быть
    let node_left = use_signal(|| vec![NodeMeta::new()]);
    let node_contnet = use_signal(|| vec![NodeMeta::new()]);
    let node_right = use_signal(|| vec![NodeMeta::new()]);
    rsx! {
        main { id: "main-layout", class: "main-layout",
            SidebarLeft {node_left },
            ContentArea {node_contnet}
            SidebarRight {node_right}
        }
    }
}
