use dioxus::prelude::*;
use dioxus_free_icons::icons::ld_icons::{LdCamera, LdEllipsisVertical};
use dioxus_free_icons::Icon;

#[component]
pub fn Header() -> Element {
    rsx! {
        div {
            class: "px-4 h-14 flex flex-row items-center justify-between",
            h1 {
                class: "text-[25px] font-semibold tracking-wide text-green-600",
                "WhatsApp"
            }
            div {
                class: "flex flex-row gap-x-1 items-center",
                button {
                    class: "rounded-full p-2 hover:bg-gray-50 transition-colors",
                    Icon {
                        width: 24,
                        height: 24,
                        fill: "black",
                        icon: LdCamera,
                    }
                }
                button {
                    class: "rounded-full p-2 hover:bg-gray-50 transition-colors",
                    Icon {
                        width: 24,
                        height: 24,
                        fill: "black",
                        icon: LdEllipsisVertical,
                    }
                }
            }
        }
    }
}
