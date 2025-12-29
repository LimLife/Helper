use dioxus::prelude::*;
use dioxus_router::{Routable};

use crate::components::{elements::{
    pages::blog::Blog
},
    elements::layouts::main_layout::MainLayout};


#[derive(Debug, Clone, Routable, PartialEq)]
#[rustfmt::skip]
pub enum Route {
    #[route("/")]
    MainLayout {},
    #[route("/blog/:id")]
    Blog { id: i32 },
}
