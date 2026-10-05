use crate::app::App;

mod app;
mod components;
mod data_component;
mod pages;
mod render_components;
fn main() {
    dioxus::launch(App);
}
