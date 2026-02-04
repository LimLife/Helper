
#[derive(Debug,Clone,PartialEq, Eq)]
pub struct Selector {
    pub value: String,
    pub name: String,
    pub selected: bool,
    pub disabled: bool,
}