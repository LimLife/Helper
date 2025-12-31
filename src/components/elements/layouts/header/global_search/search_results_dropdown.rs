use dioxus::prelude::*;


#[component]
pub fn SearchResultDropDown()-> Element{
    rsx!{
        span { id: "search-result", class: "search-result", "SearchResultDropDown" }
    }
}