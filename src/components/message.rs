use dioxus::prelude::*;

#[derive(Clone, PartialEq)]
pub struct Msg {
    pub text: String,
    pub time: String,
    pub sent: bool,
}

#[component]
pub fn Message(msg: Msg) -> Element {
    rsx! {
        div {
            class: if msg.sent { "flex justify-end px-4 py-0.5" } else { "flex justify-start px-4 py-0.5" },
            div {
                class: "relative max-w-[80%]",
                // Tail
                div {
                    class: if msg.sent {
                        "absolute top-0 -right-3 w-5 h-5 overflow-hidden"
                    } else {
                        "absolute top-0 -left-3 w-5 h-5 overflow-hidden"
                    },
                    svg {
                        class: "w-full h-full",
                        view_box: "0 0 20 20",
                        if msg.sent {
                            path {
                                // To change roundness: Adjust the Q control points.
                                // Q 18 0 (top-right control) and Q 4 20 (bottom-left curve)
                                // Making the first Q closer to 20 0 makes it sharper.
                                d: "M 0,0 H 15 Q 18,0 12,10 L 6,18 Q 4,20 0,20 Z",
                                fill: "#DCF8C6"
                            }
                        } else {
                            path {
                                d: "M 20,0 H 5 Q 2,0 8,10 L 14,18 Q 16,20 20,20 Z",
                                fill: "#ffffff"
                            }
                        }
                    }
                }
                // Bubble
                div {
                    class: if msg.sent {
                        "bg-[#DCF8C6] rounded-xl rounded-tr-none px-3 py-2 shadow-sm"
                    } else {
                        "bg-white rounded-xl rounded-tl-none px-3 py-2 shadow-sm"
                    },
                    p { class: "text-[15px] text-gray-900", "{msg.text}" }
                    div {
                        class: "flex justify-end items-center gap-1 mt-1",
                        span { class: "text-[11px] text-gray-500", "{msg.time}" }
                    }
                }
            }
        }
    }
}
