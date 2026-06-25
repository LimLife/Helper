#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Selector {
    pub value: String,
    pub name: String,
    pub selected: bool,
    pub disabled: bool,
}

impl Selector {
    pub fn empty() -> Self {
        Self {
            value: String::new(),
            name: String::new(),
            selected: bool::default(),
            disabled: bool::default(),
        }
    }
    pub fn empty_arr() -> Vec<Self> {
        vec![Selector::empty()]
    }
}
