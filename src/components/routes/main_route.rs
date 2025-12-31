use dioxus::prelude::*;
use dioxus_router::{Routable};

use crate::components::{elements::{
    pages::main::Main
}};


#[derive(Debug, Clone, Routable, PartialEq)]
#[rustfmt::skip]
pub enum Route {
    #[route("/")]
    Main {},
}
