
pub struct NavigationItem {
    pub label: String,
    pub path: String,
}

pub struct NavigationPath {
    pub segment: Vec<NavigationItem>
}