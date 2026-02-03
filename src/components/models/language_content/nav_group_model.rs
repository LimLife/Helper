use super::nav_link_model::NavLink;
#[derive(Clone, PartialEq, Eq)]
pub struct NavGroup{
    pub name: String,
    pub links: Vec<NavLink>,
    pub childrens: Option<Vec<NavGroup>>
}
