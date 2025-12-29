use dioxus::prelude::*;

use crate::components::elements::layouts::{
    header::Header,
    main_layout::MainLayout,
    footer::Footer
};



#[component]
pub fn Main() -> Element {
   
   rsx!{
    Header {},
    MainLayout {},
    Footer {}
   }
}