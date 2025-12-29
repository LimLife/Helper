mod footer_brand;
mod footer_community;
mod footer_copyright;
mod footer_languages;
mod footer_resources;

use dioxus::prelude::*;
use super::footer::{
    footer_brand::FooterBrand,
    footer_community::FooterCommunity,
    footer_copyright::FooterCopyright,
    footer_languages::FooterLanguages,
    footer_resources::FooterResources
};

#[component]
pub fn Footer() -> Element {
    rsx!{
        FooterBrand {  },
        FooterCommunity {},
        FooterCopyright {},
        FooterLanguages {},
        FooterResources {}
    }
}