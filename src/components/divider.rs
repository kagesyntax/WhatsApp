use dioxus::prelude::*;

#[component]
pub fn Divider() -> Element {
    rsx! {
        div {
            class: "bg-gray-800",
            ul {
                class: "block items-center justify-center",
                li {class: "bg-white", "Hi" }
                li { "Hello" }
            }
        }
    }
}
