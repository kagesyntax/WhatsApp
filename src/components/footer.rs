use crate::Route;
use dioxus::prelude::*;
use dioxus_free_icons::icons::ld_icons::{
    LdMessageCircleDashed, LdMessageSquareText, LdPhone, LdUsers,
};
use dioxus_free_icons::Icon;

#[component]
pub fn Footer() -> Element {
    let nav = use_navigator();
    let route = use_route::<Route>();

    let active_tab = match route {
        Route::Home { .. } => 0,
        Route::Updates { .. } => 1,
        Route::Communities { .. } => 2,
        Route::Calls { .. } => 3,
        _ => 0,
    };

    rsx! {
        div {
            class: "w-full h-16 bg-white flex flex-row items-center justify-around border-t border-gray-200 flex-shrink-0",
            button {
                class: if active_tab == 0 { "flex flex-col items-center gap-0.5 text-[#075E54]" } else { "flex flex-col items-center gap-0.5 text-gray-500" },
                onclick: move |_| { nav.push(Route::Home {}); },
                div {
                    class: if active_tab == 0 { "rounded-full px-5 py-1.5 bg-green-50 transition-colors" } else { "rounded-full px-5 py-1.5 hover:bg-gray-100 transition-colors" },
                    Icon { width: 24, height: 24, fill: if active_tab == 0 { "#075E54" } else { "#8E8E93" }, icon: LdMessageSquareText }
                }
                span { class: "text-[10px] font-medium", "Chats" }
            }
            button {
                class: if active_tab == 1 { "flex flex-col items-center gap-0.5 text-[#075E54]" } else { "flex flex-col items-center gap-0.5 text-gray-500" },
                onclick: move |_| { nav.push(Route::Updates {}); },
                div {
                    class: if active_tab == 1 { "rounded-full px-5 py-1.5 bg-green-50 transition-colors" } else { "rounded-full px-5 py-1.5 hover:bg-gray-100 transition-colors" },
                    Icon { width: 24, height: 24, fill: if active_tab == 1 { "#075E54" } else { "#8E8E93" }, icon: LdMessageCircleDashed }
                }
                span { class: "text-[10px]", "Updates" }
            }
            button {
                class: if active_tab == 2 { "flex flex-col items-center gap-0.5 text-[#075E54]" } else { "flex flex-col items-center gap-0.5 text-gray-500" },
                onclick: move |_| { nav.push(Route::Communities {}); },
                div {
                    class: if active_tab == 2 { "rounded-full px-5 py-1.5 bg-green-50 transition-colors" } else { "rounded-full px-5 py-1.5 hover:bg-gray-100 transition-colors" },
                    Icon { width: 24, height: 24, fill: if active_tab == 2 { "#075E54" } else { "#8E8E93" }, icon: LdUsers }
                }
                span { class: "text-[10px]", "Communities" }
            }
            button {
                class: if active_tab == 3 { "flex flex-col items-center gap-0.5 text-[#075E54]" } else { "flex flex-col items-center gap-0.5 text-gray-500" },
                onclick: move |_| { nav.push(Route::Calls {}); },
                div {
                    class: if active_tab == 3 { "rounded-full px-5 py-1.5 bg-green-50 transition-colors" } else { "rounded-full px-5 py-1.5 hover:bg-gray-100 transition-colors" },
                    Icon { width: 24, height: 24, fill: if active_tab == 3 { "#075E54" } else { "#8E8E93" }, icon: LdPhone }
                }
                span { class: "text-[10px]", "Calls" }
            }
        }
    }
}
