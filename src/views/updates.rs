use dioxus::prelude::*;

#[component]
pub fn Updates() -> Element {
    rsx! {
        div {
            class: "flex-1 flex items-center justify-center text-gray-500 text-lg",
            "Updates"
        }
    }
}
