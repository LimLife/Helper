use dioxus::prelude::*;



#[component]
pub fn CodeContent() -> Element {
    rsx! {
        pre {
            code { "use dioxus::prelude::*;" }
        }
    }
}