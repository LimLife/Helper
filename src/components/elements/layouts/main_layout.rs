mod content_area;
mod sidebar_left;
mod sidebar_right;
mod tree_root;
use dioxus::prelude::*;

use super::main_layout::{
    content_area::ContentArea, sidebar_left::SidebarLeft, sidebar_right::SidebarRight,
};
use crate::render_components::content_render::RenderCoomponent;
use shared::{
    data_block::CodeData,
    manifesto::{Block, NodeMeta},
};
#[component]
pub fn MainLayout() -> Element {
    //сдесь должны быть разные масивы и должен быть
    let node_left = use_signal(|| vec![NodeMeta::new()]);
    let node_right = use_signal(|| vec![NodeMeta::new()]);
    let node_content = use_signal(|| {
        vec![Block::Code(CodeData {
            code_content: Some(String::from("")),
            code_header: Some(String::from("")),
            title: Some(String::from("")),
        })]
    });
    rsx! {
        main { id: "main-layout", class: "main-layout",
            SidebarLeft {node_left },
            ContentArea {
                RenderCoomponent{node_block: node_content}
            }
            SidebarRight {node_right}
        }
    }
}
