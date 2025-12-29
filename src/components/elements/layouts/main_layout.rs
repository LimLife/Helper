mod content_area;
mod sidebar_left;
mod sidebar_right;

use dioxus::prelude::*;

use super::main_layout::{
    content_area::ContentArea,
    sidebar_left::SidebarLeft,
    sidebar_right::SidebarRight
};


#[component]
pub fn MainLayout() -> Element{
    rsx!{
        SidebarLeft {  },
        ContentArea {  },
        SidebarRight {  }
    }
}