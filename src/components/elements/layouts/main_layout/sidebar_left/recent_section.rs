mod recent_header;
mod recent_item;


use dioxus::prelude::*;
use super::recent_section::{
    recent_header::RecentHeader,
    recent_item::RecentItem
};


#[component]
pub fn RecentSection() -> Element{
    rsx!{
        RecentHeader {  },
        RecentItem {  }
    }
}