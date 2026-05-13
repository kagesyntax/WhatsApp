use dioxus::prelude::*;
use dioxus_free_icons::icons::ld_icons::LdSearch;
use dioxus_free_icons::Icon;

#[component]
pub fn Search() -> Element {
    rsx! {
        div {
            class: "px-3 py-2",
            div {
                class: "bg-gray-50 rounded-full px-3 py-2 flex flex-row items-center gap-x-3",
                Icon {
                    width: 20,
                    height: 20,
                    fill: "black",
                    icon: LdSearch,
                }
                input {
                    class: "outline-none focus:outline-none active:outline-none bg-transparent flex-1 text-[15px]",
                    placeholder: "Ask Meta AI or Search"
                }
            }
        }
    }
}
