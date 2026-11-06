use dioxus::prelude::*;

use crate::features::articel::article_view::ArticleView;
#[component]
pub fn ContentArea() -> Element {
    rsx! {
        div { id: "content-area", class: "content-area",
            ArticleView {}
        }
    }
}
