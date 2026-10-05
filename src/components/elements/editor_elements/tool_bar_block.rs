use dioxus::prelude::*;
use shared::{manifesto::InsertPosition, new_type_id::ContentID, store::ArticleStore};
#[component]
pub fn ToolBarBlock(
    id: ContentID,
    on_create: EventHandler<InsertPosition>,
    on_edit: EventHandler<ContentID>,
) -> Element {
    let store_content = use_context::<Signal<ArticleStore>>();
    let move_up = {
        let mut store_content = store_content;
        move |_| {
            store_content.write().move_content_up(id);
        }
    };
    let on_above = { move |_| on_create.call(InsertPosition::Above(id)) };
    let on_below = { move |_| on_create.call(InsertPosition::Below(id)) };
    let down = {
        let mut store_content = store_content;
        move |_| {
            store_content.write().move_content_down(id);
        }
    };
    let duplicate = {
        let mut store_content = store_content;
        move |_| {
            store_content.write().duplicate_content(id);
        }
    };
    let delete = {
        let mut store_content = store_content;
        move |_| {
            store_content.write().delete_content(id);
        }
    };
    let edit = move |_| on_edit.call(id);
    rsx! {
        ul { id: "tool-bar-block", class: "tool-bar-block",
            li { id: "edit", position: 0, onclick: edit, "Edit Overlay" }
            li { id: "move-up", position: 1, onclick: move_up, "Move UP" }
            li { id: "move-down", position: 2, onclick: down, "Move DOWN" }
            li { id: "duplicate", position: 3, onclick: duplicate, "DUPLICATE" }
            li { id: "delete", position: 4, onclick: delete, "DELETE" }
            li { id: "insert above", position: 5, onclick: on_above, "INSERT ABOVE" }
            li { id: "insert below", position: 6, onclick: on_below, "INSERT BELOW" }
        }
    }
}
