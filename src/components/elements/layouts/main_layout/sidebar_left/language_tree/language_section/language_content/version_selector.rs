use dioxus::prelude::*;


#[component]
pub fn VersionSelector()-> Element {
    rsx!{
        select { class: "lenguage-section-selector",
            option { value: "Version 1.0", "Version 1.0" }
            option { value: "Version 2.0", selected: true, "Version 2.0" }
            option { value: "Version 3.0", "Version 3.0" }
            option { value: "Version 4.0", "Version 4.0" }
        }
    }
}