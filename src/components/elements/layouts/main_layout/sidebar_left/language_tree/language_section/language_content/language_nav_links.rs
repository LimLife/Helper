mod nav_link;
mod nested_nav_group;

use dioxus::prelude::*;
use super::language_nav_links::{
    nav_link::NavLink,
    nested_nav_group::NestedNavGroup
};

#[component]
pub fn LanguageNavLinks()-> Element{
    rsx!{
        NavLink {  }
        NestedNavGroup { }
    }
}