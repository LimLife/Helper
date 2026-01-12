use dioxus::prelude::*;


#[component]
pub fn SectionTitle() -> Element {
    rsx!{
        p {
            "Variables are containers 
            for storing data values.
         In JavaScript, you can declare
          variables using var, let, or con
          and mutability characteristics.
         Understanding variable declaration and scoping 
         is fundamental to writing clean, maintainable 
         JavaScript code that avoids common pitfalls. "
        }
    }
}