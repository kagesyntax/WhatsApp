use dioxus::prelude::*;
use dioxus_free_icons::icons::ld_icons::{LdCheck, LdCheckCheck};
use dioxus_free_icons::Icon;

const PROFILE_PICTURE: Asset = asset!("/assets/profile.png");

#[derive(Clone, PartialEq)]
pub enum MessageStatus {
    Sent,
    Delivered,
    Read,
}

#[derive(Clone, PartialEq)]
pub struct ChatInfo {
    pub name: String,
    pub message: String,
    pub time: String,
    pub status: MessageStatus,
}

#[component]
pub fn Chats(chats: Vec<ChatInfo>, on_chat_click: EventHandler<usize>) -> Element {
    rsx! {
        div {
            class: "w-full flex flex-col bg-white",
            for (i, chat) in chats.iter().enumerate() {
                div {
                    class: "flex flex-row items-center px-5 py-3 hover:bg-gray-50 active:bg-gray-100 transition-colors cursor-pointer snap-start",
                    onclick: move |_| on_chat_click.call(i),
                    div {
                        class: "flex-shrink-0",
                        img {
                            class: "w-13 h-13 rounded-full object-cover bg-gray-180",
                            src: PROFILE_PICTURE,
                            alt: "Profile Picture"
                        }
                    }
                    div {
                        class: "ml-3 flex-1 min-w-0 flex flex-col justify-center",
                        div {
                            class: "flex flex-row justify-between items-start",
                            h1 {
                                class: "text-[16px] text-gray-900 truncate",
                                "{chat.name}"
                            }
                            span {
                                class: "text-[12px] text-gray-500 ml-2 whitespace-nowrap",
                                "{chat.time}"
                            }
                        }
                        div {
                            class: "flex flex-row justify-between items-center",
                            {
                                match chat.status {
                                    MessageStatus::Sent => rsx! {
                                        Icon { width: 18, height: 18, fill: "gray", icon: LdCheck }
                                    },
                                    MessageStatus::Delivered => rsx! {
                                        Icon { width: 18, height: 18, fill: "gray", icon: LdCheckCheck }
                                    },
                                    MessageStatus::Read => rsx! {
                                        span { class: "text-green-600",
                                            Icon { width: 18, height: 18, fill: "gray", icon: LdCheckCheck }
                                        }
                                    },
                                }
                            }
                            p {
                                class: "text-[15px] text-gray-500 pl-2 truncate leading-tight flex-1",
                                "{chat.message}"
                            }
                        }
                    }
                }
            }
        }
    }
}
