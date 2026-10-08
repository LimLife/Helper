use crate::pages::{admin_panel::AdminPage, main::Main};
use dioxus::prelude::*;
use dioxus_router::Routable;
#[derive(Debug, Clone, Routable, PartialEq)]
#[rustfmt::skip]
pub enum Route {
    #[route("/")]
    Main {},
    #[route("/admin")]
    AdminPage,
}
