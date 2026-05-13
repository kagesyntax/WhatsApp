use dioxus::prelude::*;

#[component]
pub fn Tags(tags: Vec<String>) -> Element {
    rsx! {
        div {
            class: "flex flex-row gap-2 px-3 py-3 overflow-x-auto",
            for (i, tag) in tags.iter().enumerate() {
                if i == 0 {
                    div {
                        class: "bg-green-100/95 border border-[1px] border-gray-300 text-gray-600 text-sm rounded-full px-2.5 py-1 whitespace-nowrap",
                        "{tag}"
                    }
                } else {
                    div {
                        class: "bg-transparent border border-gray-300 text-gray-600 text-sm rounded-full px-2.5 py-1 whitespace-nowrap",
                        "{tag}"
                    }
                }
            }
        }
    }
}
