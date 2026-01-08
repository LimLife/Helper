use dioxus::prelude::*;

use crate::components::{assets::{
    css::import_styles,
    images::FAVICON
},
    routes::main_route::Route
};
#[component]
pub fn App() -> Element {
    rsx! {
        import_styles {}
        document::Link { rel: "icon", href: FAVICON }
        Router::<Route> {}
    }
}