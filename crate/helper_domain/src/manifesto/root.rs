use serde::{Deserialize, Serialize};

use crate::manifesto::node::NodeMeta;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct RootManifest {
    pub version_bar: u32,
    pub site_title: String,
    pub root_sections: Vec<NodeMeta>,
}
