#[cfg(feature = "server")]
use dioxus::prelude::*;



#[cfg(feature = "server")]
#[server]
pub async fn echo_server(input: String) -> Result<String, ServerFnError> {
    println!("Echoing {}", input);
    Ok(input)
}