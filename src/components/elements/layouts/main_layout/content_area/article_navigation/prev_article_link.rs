use dioxus::prelude::*;


#[component]
pub fn PrevArticleLink()->Element{
    rsx!{
        a { href: "#", "Prev Article" }
    }
}