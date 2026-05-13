use crate::components::{Chats, Header, Search, Tags};
use crate::data::{get_chats, get_tags};
use crate::Route;
use dioxus::prelude::*;

#[component]
pub fn Home() -> Element {
    let chats = get_chats();
    let tags = get_tags();
    let nav = use_navigator();

    rsx! {
        div {
            class: "flex-1 flex flex-col min-h-0",
            Header {}
            Search {}
            div {
                class: "flex-1 overflow-y-auto snap-y snap-proximity min-h-0",
                Tags { tags }
                Chats {
                    chats: chats.clone(),
                    on_chat_click: move |idx| {
                        nav.push(Route::Chat { index: idx });
                    },
                }
            }
        }
    }
}
