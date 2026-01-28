use crate::components::shared::navigation::{NavigationItem};

#[derive(Debug, Clone)]
pub struct NavigationStore {
    current: Vec<String>,
}

impl NavigationStore {
    pub fn new () -> Self {
        Self { current: Vec::<String>::new() }
    }
    pub fn set_path(&mut self, path:&str) {
        self.current = path
        .trim_matches('/')
        .split('/')
        .filter(|s| !s.is_empty())
        .map(|s|s.to_string())
        .collect();       
    }
    pub fn current(&self)-> &[String]{
        &self.current
    }
    pub fn breadcrumbs(&self)-> Vec<NavigationItem> {
        let mut items = Vec::with_capacity(self.current.len());
        let mut current_path = String::new();

        for segments in &self.current {
            current_path.push('/');
            current_path.push_str(segments);
            items.push(NavigationItem { label: segments.clone(), path: current_path.clone()
            });
        }
        items
    }
    pub fn clear(&mut self) {
        self.current.clear();
    }
    pub fn is_empty(&self)-> bool{
        self.current.is_empty()
    }
}
impl Default for NavigationStore {
    fn default() -> Self {
        Self::new()
    }
}