// ── Imports ──
use crate::components::Message;
use crate::data::{get_chats, sample_messages};
use dioxus::prelude::*;
use dioxus_free_icons::icons::ld_icons::{
    LdArrowLeft, LdCamera, LdEllipsisVertical, LdMic, LdPaperclip, LdPhone, LdSmile, LdVideo,
};
use dioxus_free_icons::Icon;

const CHAT_BACKGROUND: Asset = asset!("/assets/background.png");

// ── Chat Screen ──
#[component]
pub fn Chat(index: usize) -> Element {
    // ── Hooks & Data ──
    let chats = get_chats();
    let chat = chats.get(index);
    let nav = use_navigator();
    let messages = sample_messages();

    match chat {
        Some(chat) => {
            rsx! {
                // ── Root: full-screen chat container ──
                div {
                    class: "h-screen flex flex-col bg-[#ECE5DD]/10",
                    style: "background-image: url('{CHAT_BACKGROUND}'); background-repeat: repeat; background-size: 400px; background-blend-mode: multiply;",

                    // ── Header: back arrow, profile pic, name, action icons ──
                    div {
                        class: "h-14 bg-white flex flex-row items-center justify-between px-1 flex-shrink-0 border border-[1px] border-gray-50",

                        // Left section: back + photo + name
                        div {
                            class: "flex flex-row items-center gap-x-1.5",
                            button {
                                class: "rounded-full p-2 hover:bg-gray-100 transition-colors",
                                onclick: move |_| nav.go_back(),
                                Icon { width: 20, height: 20, fill: "gray", icon: LdArrowLeft }
                            }
                            {
                                let first = chat.name.chars().next().unwrap_or('?');
                                match &chat.image {
                                    Some(asset) => rsx! {
                                        img { class: "w-10 h-10 rounded-full object-cover", src: *asset }
                                    },
                                    None => rsx! {
                                        div { class: "w-10 h-10 rounded-full bg-gray-300 flex items-center justify-center text-gray-600 text-sm font-semibold", "{first}" }
                                    },
                                }
                            }
                            h1 { class: "text-md text-gray font-extralight", "{chat.name}" }
                        }

                        // Right section: video, phone, more buttons
                        div {
                            class: "flex flex-row items-center gap-x-1",
                            button {
                                class: "rounded-full p-2 hover:bg-gray-100 transition-colors",
                                Icon { width: 22, height: 22, fill: "gray", icon: LdVideo }
                            }
                            button {
                                class: "rounded-full p-2 hover:bg-gray-100 transition-colors",
                                Icon { width: 20, height: 20, fill: "gray", icon: LdPhone }
                            }
                            button {
                                class: "rounded-full p-2 hover:bg-gray-100 transition-colors",
                                Icon { width: 20, height: 20, fill: "gray", icon: LdEllipsisVertical }
                            }
                        }
                    }

                    // ── Messages area (scrollable) ──
                    div {
                        class: "flex-1 overflow-y-auto py-2",
                        for msg in messages {
                            Message { msg: msg.clone() }
                        }
                    }

                    // ── Bottom input bar (floating) ──
                    div {
                        class: "px-2 py-2 flex-shrink-0",
                        div {
                            class: "flex flex-row items-center gap-x-1.5 min-w-0",

                            // Input container: smiley | input | paperclip | camera
                            div {
                                class: "flex-1 flex flex-row items-center bg-white rounded-full px-3 py-2.5 gap-x-3 min-w-0",
                                button {
                                    class: "rounded-full p-0.5 hover:bg-gray-200 transition-colors flex-shrink-0 text-[#8E8E93]",
                                    Icon { width: 22, height: 22, fill: "gray", icon: LdSmile }
                                }
                                input {
                                    class: "flex-1 min-w-0 bg-transparent outline-gray focus:outline-gray active:outline-gray text-[15px]",
                                    placeholder: "Type a message"
                                }
                                button {
                                    class: "rounded-full p-0.5 hover:bg-gray-200 transition-colors flex-shrink-0 text-[#8E8E93]",
                                    Icon { width: 22, height: 22, fill: "gray", icon: LdPaperclip }
                                }
                                button {
                                    class: "rounded-full p-0.5 hover:bg-gray-200 transition-colors flex-shrink-0 text-[#8E8E93]",
                                    Icon { width: 22, height: 22, fill: "gray", icon: LdCamera }
                                }
                            }

                            // Microphone button (standalone green circle)
                            button {
                                class: "w-11 h-11 rounded-full bg-[#075E54] flex items-center justify-center flex-shrink-0 hover:opacity-90 transition-opacity",
                                Icon { width: 22, height: 22, fill: "white", icon: LdMic }
                            }
                        }
                    }
                }
            }
        }
        None => {
            rsx! {
                div { class: "h-screen flex items-center justify-center", "Chat not found" }
            }
        }
    }
}
