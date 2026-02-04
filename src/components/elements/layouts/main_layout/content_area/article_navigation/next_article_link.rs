use dioxus::prelude::*;


#[component]
pub fn NextArticleLink()->Element{
    rsx!{
        a { href: "#", "Next Article" }
    }
}