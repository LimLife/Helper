#[cfg(feature = "server")]
use dioxus::prelude::*;


#[cfg(feature = "server")]
#[server]
pub async fn get_name() -> Result<String, ServerFnError> {
    Ok("LimLife".to_string())
}