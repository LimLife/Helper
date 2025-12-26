use dioxus::prelude::*;



#[component]
pub fn Collapse(id:String, label: String, content:Element)->Element {
    let id_clone = id.clone();
    let mut open = use_signal(||false);
    rsx!{
        div { 
            class: "ui_collapse",
                input {  
                    r#type: "checkbox",
                    id: "{id_clone}",
                    onclick: move |_| {open.set(!open())}
                }
                CollapseLabel { id:id_clone.clone(), label }
                CollapseContent { content }
         }
    }
}

#[component]
pub fn CollapseLabel(id:String,label:String) -> Element {
    rsx!{
        label { 
            id: "{id}",
            r#for: "{id}",
            class: "ui_collapse-label",
            {label}
        }
    }
}

#[component]
pub fn CollapseContent(content:Element)->Element {
    rsx!{
        div {
            class: "ui_collapse-content-wrapper",
            div { 
                class:"ui_collapse-content",
                {content}
            }
        }
    }
}