use dioxus::prelude::*;

#[get("/api/name")]
pub async fn get_name() -> Result<String, ServerFnError> {
    Ok("LimLife".to_string())
}