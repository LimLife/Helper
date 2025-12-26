use dioxus::prelude::*;
use crate::components::elements::layouts::echo::Echo;


#[component]
pub fn Home() -> Element {
    rsx! {
        Echo {}
    }
}