use dioxus::prelude::*;


#[component]
pub fn Comment(comment:String)->Element {
    rsx! {
        li { {comment} }
    }
}