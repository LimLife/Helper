use dioxus::prelude::*;
use crate::components::elements::echo::Echo;


#[component]
pub fn Home() -> Element {
    rsx! {
        Echo {}
    }
}