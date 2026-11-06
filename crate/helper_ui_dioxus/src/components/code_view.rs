use dioxus::prelude::*;

#[component]
pub fn TextEditor(
    value: ReadSignal<String>,
    on_input: EventHandler<String>,
    #[props(default ="Введите или вставьте код...".to_string())] place_holder: String,
) -> Element {
    rsx! {
        div { class: "code-textarea-pos",
            textarea {
                id: "code-code-content",
                class: "code-textarea",
                value: value,
                oninput: move |e| { on_input.call(e.value())},
                placeholder: place_holder,
                spellcheck: "false",
                autocapitalize: "off",
                autocomplete: "off",
            }
        }
    }
}
