use dioxus::prelude::*;
use shared::{new_type_id::ContentID, store::ArticleStore};

#[component]
pub fn ToolBarBlock(id: ContentID) -> Element {
    let store_content = use_context::<Signal<ArticleStore>>();

    let move_up = {
        let mut store_contnent = store_content;
        move |_| {
            store_contnent.write().move_content_up(id);
        }
    };

    let down = {
        let mut store_contnent = store_content;
        move |_| {
            store_contnent.write().move_content_down(id);
        }
    };

    let dublicate = {
        let mut store_contnent = store_content;
        move |_| {
            let id_new = store_contnent.write().dublicate_content(id);
        }
    };

    let delete = {
        let mut store_contnent = store_content;
        move |_| {
            store_contnent.write().delete_content(id);
        }
    };

    let insert_above = {
        let mut store_contnent = store_content;
        move |_| {
            // store_contnent.write().insert_content_above(target, block);
        }
    };

    let insert_below = {
        let mut store_contnent = store_content;
        move |_| {
            // store_contnent.write().insert_content_above(target, block);
        }
    };

    rsx! {
        ul {
            id: "tool-bar-block",
            class: "tool-bar-block",
            li {
                id: "move-up",
                onclick: move_up,
                "Move UP"
            }
            li {
                id: "move-down",
                onclick:down,
                "Move DOWN"
            }
            li {
                id: "dublicate",
                onclick: dublicate,
                "DUBLICATE"
            }
            li {
                id: "delete",
                onclick: delete,
                "DELETE"
            }
            li {
                id: "insert above",
                onclick: insert_above,
                "INSERT ABOVE"
            }
            li {
                id: "insert below",
                onclick: insert_below,
                "INSERT BELOW"
            }
        }
    }
}
