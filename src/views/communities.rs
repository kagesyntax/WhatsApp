use dioxus::prelude::*;

#[component]
pub fn Communities() -> Element {
    rsx! {
        div {
            class: "flex-1 flex items-center justify-center text-gray-500 text-lg",
            "Communities"
        }
    }
}
