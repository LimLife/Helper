use crate::pages::{admin_panel::AdminPage, main::MainPage};
use dioxus::prelude::*;
use dioxus_router::Routable;
#[derive(Debug, Clone, Routable, PartialEq)]
#[rustfmt::skip]
pub enum Route {
    #[route("/")]
    MainPage {},
    #[route("/admin")]
    AdminPage {}
}
