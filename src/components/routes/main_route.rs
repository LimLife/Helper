use dioxus::prelude::*;
use dioxus_router::{Routable};

use crate::components::{elements::{
   pages::home::Home,
   layouts::nav_bar::Navbar,
   pages::blog::Blog
}};


#[derive(Debug, Clone, Routable, PartialEq)]
#[rustfmt::skip]
pub enum Route {
    #[layout(Navbar)]
    #[route("/")]
    Home {},
    #[route("/blog/:id")]
    Blog { id: i32 },
}
