mod bookmark_button;
mod history_button;
mod user_avatar_menu;

use dioxus::prelude::*;

use super::user_menu::{
    bookmark_button::BookmarkButton,
    history_button::HistoryButton,
    user_avatar_menu::UserAvatarMenu
};


#[component]
pub fn UserMenu() -> Element {
    rsx!{
        BookmarkButton{},
        HistoryButton{},
        UserAvatarMenu{}
    }
}