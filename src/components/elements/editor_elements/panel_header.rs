use dioxus::{core::EventHandler, core_macro::component, html::MouseEvent, prelude::*};
#[component]
pub fn PanelHeader(save: EventHandler<MouseEvent>, remove: EventHandler<MouseEvent>) -> Element {
    rsx! {
        div { class: "panel_header",
            span { class: "panel_header-name", "Редактор кода" }
            button { class: "panel_header-save", onclick: save, "Сохранить" }
            button { class: "panel_header-save", onclick: remove, "Удалить изменения" }
        }
    }
}
