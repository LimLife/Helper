mod app;
mod components;
mod data_component;
mod pages;
mod render_components;
use crate::components::elements::app::App;

fn main() {
    dioxus::launch(App);
}
