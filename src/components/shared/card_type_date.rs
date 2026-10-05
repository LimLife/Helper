#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CardTypeDate {
    pub type_name: String,
    pub type_description: String,
    pub code: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CardDate {
    pub label: String,
    pub typs: Option<Vec<CardTypeDate>>,
}
