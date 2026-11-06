mod footer_brand;
mod footer_community;
mod footer_copyright;
mod footer_languages;
mod footer_resources;

pub mod footer_components {
    pub use crate::layouts::footer::{
        footer_brand::FooterBrand, footer_community::FooterCommunity,
        footer_copyright::FooterCopyright, footer_languages::FooterLanguages,
        footer_resources::FooterResources,
    };
}
