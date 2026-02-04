use dioxus::prelude::*;
use crate::components::models::selector_version::selector::Selector;

#[component]
pub fn VersionSelector(selector: Vec<Selector>)-> Element {
    rsx!{
        select { class: "selector",
            for selector in &selector {
                SelectorOptions {
                    disabled: selector.disabled.clone(),
                    selected: selector.selected.clone(),
                    value: selector.value.clone(),
                    name: selector.name.clone(),
                }
            }
        }
    }
}
#[component]
pub fn SelectorOptions(disabled: bool, selected: bool, value: String, name: String) -> Element {
    rsx!{
        option { disabled, selected, value, {name} }
    }
}