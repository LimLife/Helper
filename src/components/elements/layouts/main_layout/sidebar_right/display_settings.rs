mod font_size_control;
mod theme_switcher;

use dioxus::prelude::*;
use super::display_settings::{
    font_size_control::FontSizeControl,
    theme_switcher::ThemeSwitcher
};
#[component]
pub fn DisplaySettings() -> Element {
    rsx! {
        FontSizeControl {},
        ThemeSwitcher {}
    }
}